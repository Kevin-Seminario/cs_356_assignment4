#!/usr/bin/env python3
"""pad.py: a world's game pad. pad.json is the plan (the agent edits it); pad.html is the page (built, never edited).

  python3 ../tools/pad.py init N      write WN/pad.json from the API: the seating and the agreements in force
  python3 ../tools/pad.py refresh N   update the facts (round, cutoff, seating, names) from the API; plan untouched
  python3 ../tools/pad.py check N     check pad.json; say what is wrong
  python3 ../tools/pad.py build N     check, then write WN/pad.html (open it in a browser; reload after each build)

Run from any folder of the harness. The strategy, the moves, the views' choices and the agreement rows are the
team's: the agent writes them down in the team's words and never decides them.
"""
import json
import os
import pathlib
import sys
from datetime import datetime, timedelta, timezone

CT = timezone(timedelta(hours=-6))  # Central time; the season (Nov 9 to Dec 11) is all standard time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import na4  # noqa: E402

H = pathlib.Path(__file__).resolve().parent.parent
STATUS = ('to upload', 'to sign', 'sent', 'signed', 'running', 'hold', 'declined')
ROLES = ('customer', 'peer', 'provider')
BANDS = ('reach', 'bought', 'gap')


def path(w):
    return H / f'W{w}' / 'pad.json'


def facts(s, w):
    """The pad's facts from the API: round, cutoff (Central), when, the seating [AS, sector, name, AI]."""
    x, nr = s['worlds'][str(w)], s['next_round'] or {}
    cut = datetime.fromisoformat(nr['cutoff'].replace('Z', '+00:00')) if nr.get('cutoff') else None
    ct = cut.astimezone(CT).strftime('%a %b %d %H:%M') if cut else ''
    teams = [[t['as_id'], t['sector'], t['team'] or f'AS {t["as_id"]} (starter)'] + ([1] if t['ai'] else []) for t in x['map']]
    return dict(world=int(w), round=nr.get('n'), team=s['team'], cutoff=ct,
                updated=datetime.now(CT).strftime('%b %d %H:%M'), teams=teams)


def init(w):
    p = path(w)
    if p.exists():
        sys.exit(f'{p} exists; use refresh to update its facts.')
    s = na4.get('/api/state')
    pad = facts(s, w)
    roles = {}
    for g in s['agreements']:
        if g['world'] == int(w) and g['rounds'][1] >= (pad['round'] or 0):
            other = g['b'] if g['a'] == s['team'] else g['a']
            roles[other] = 'peer' if g['kind'] == 'peer' else ('provider' if g['a'] == s['team'] else 'customer')
    for f in s['agreement_files']:  # signed files in force: links are [buyer, seller, kind] by the names written in the file
        if f['world'] == int(w) and f['status'] == 'signed' and f.get('in_force') and f['in_force'][1] >= (pad['round'] or 0):
            name = dict(zip(f['parties_as_written'], [p['team'] for p in f['parties']]))
            for a, b, kind in f['links']:
                a, b = name.get(a, a), name.get(b, b)
                if s['team'] in (a, b):
                    roles[b if a == s['team'] else a] = 'peer' if kind == 'peer' else ('provider' if a == s['team'] else 'customer')
    pad.update(startView=0, strategy=dict(edge='', moves=[]),
               views=[dict(name='Now', caption='The agreements in force for the next round (from the API).',
                           roles=roles, sectors={}, links=[])],
               agreements=[])
    p.write_text(json.dumps(pad, indent=1, ensure_ascii=False) + '\n')
    print(f'wrote {p.relative_to(H)}: {len(pad["teams"])} seats, {len(roles)} partners in force. '
          'The strategy and the agreement rows are empty: the team writes them.')


def refresh(w):
    p = path(w)
    pad = json.loads(p.read_text())
    pad.update(facts(na4.get('/api/state'), w))
    p.write_text(json.dumps(pad, indent=1, ensure_ascii=False) + '\n')
    print(f'refreshed the facts in {p.relative_to(H)}')
    problems(pad)


def problems(pad, fail=False):
    out, names = [], {t[2] for t in pad.get('teams', [])}
    for k in ('world', 'round', 'team', 'cutoff', 'updated', 'strategy', 'views', 'agreements', 'teams'):
        if k not in pad:
            out.append(f'missing "{k}"')
    moves = len(pad.get('strategy', {}).get('moves', []))
    if pad.get('team') not in names:
        out.append(f'your team {pad.get("team")!r} is not in the seating: run refresh')
    for v in pad.get('views', []):
        for t, r in v.get('roles', {}).items():
            if t not in names:
                out.append(f'view {v.get("name")!r}: {t!r} is not seated in this world (renamed? run refresh)')
            if r not in ROLES:
                out.append(f'view {v.get("name")!r}: role {r!r} is not one of {", ".join(ROLES)}')
        for s, b in v.get('sectors', {}).items():
            if s not in [str(i) for i in range(8)] or b not in BANDS:
                out.append(f'view {v.get("name")!r}: sector {s!r} -> {b!r} (sectors 0-7; {", ".join(BANDS)})')
    if not pad.get('views'):
        out.append('no views: the map needs at least one')
    for a in pad.get('agreements', []):
        if a.get('status') not in STATUS:
            out.append(f'{a.get("team")}: status {a.get("status")!r} is not one of {", ".join(STATUS)}')
        if a.get('team') not in names:
            out.append(f'{a.get("team")!r} is not seated in this world')
        if a.get('role') not in ROLES:
            out.append(f'{a.get("team")}: role {a.get("role")!r}')
        if not (isinstance(a.get('move'), int) and 1 <= a['move'] <= max(moves, 1)):
            out.append(f'{a.get("team")}: "move" must name one of the strategy\'s {moves} moves')
    for x in out:
        print('  -', x)
    if out and fail:
        sys.exit(f'{len(out)} problem(s); pad.html not built.')
    return out


def build(w):
    pad = json.loads(path(w).read_text())
    problems(pad, fail=True)
    js = json.dumps(pad, ensure_ascii=False, indent=1).replace('</', '<\\/')
    page = (H / 'pads' / 'template.html').read_text().replace('/*PAD*/', js).replace('/*W*/', str(w))
    (H / f'W{w}' / 'pad.html').write_text(page)
    print(f'built W{w}/pad.html ({len(pad["agreements"])} agreements, {len(pad["views"])} views). Reload it in the browser.')


if __name__ == '__main__':
    a = sys.argv[1:]
    if len(a) != 2 or a[0] not in ('init', 'refresh', 'check', 'build') or a[1] not in '1234':
        sys.exit(__doc__)
    if a[0] == 'check':
        print('ok' if not problems(json.loads(path(a[1]).read_text())) else '')
    else:
        globals()[a[0]](a[1])
