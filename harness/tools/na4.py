#!/usr/bin/env python3
"""na4.py: read the A4 game through the website's read-only API. It only reads; it cannot act.

The key comes from the NA4_KEY environment variable and nowhere else. This script never prints it.

  python3 ../tools/na4.py state [--world N]   your team's view (one world's slice with --world)
  python3 ../tools/na4.py summary              the four worlds side by side (seat, credits, last score, open files, lots)
  python3 ../tools/na4.py auctions N           world N's open lots and your credits there
  python3 ../tools/na4.py statement R W        your statement for round R in world W
  python3 ../tools/na4.py readings R W         your meter readings (CSV) for round R in world W
  python3 ../tools/na4.py rules                the season's rules.json

A person makes every bid, signature, upload, offer and message, by hand, on the website.
"""
import json
import os
import sys
import urllib.error
import urllib.request

BASE = os.environ.get('NA4_URL', 'https://cs356-fall26.cs.utexas.edu').rstrip('/')


def get(path):
    """One GET to the API: the parsed JSON, or the text for CSV."""
    key = os.environ.get('NA4_KEY', '')
    if not key.startswith('na4_'):
        sys.exit('NA4_KEY is not set. Make a key on the website (Home, "For your agent") and set it yourself in your '
                 "terminal: export NA4_KEY='na4_...'. Never paste it into an agent's chat.")
    req = urllib.request.Request(BASE + path, headers={'Authorization': 'Bearer ' + key, 'User-Agent': 'na4.py'})
    try:
        with urllib.request.urlopen(req, timeout=30) as r:
            body = r.read().decode('utf-8')
            return json.loads(body) if 'json' in r.headers.get('Content-Type', '') else body
    except urllib.error.HTTPError as e:
        msg = {401: 'the key was refused (wrong, replaced or revoked): make a new one on the website',
               404: 'not found (no such round or world for your team yet)',
               429: f'over 30 requests a minute: wait {e.headers.get("Retry-After", "60")} s'}.get(e.code, e.reason)
        sys.exit(f'{e.code}: {msg}')


def world(state, w):
    """The slice one world chat works from: its world, and only that world's offers, agreements, files, statements."""
    w = int(w)
    keep = lambda xs: [x for x in xs if x.get('world') == w]
    return dict(team=state['team'], as_of=state.get('as_of'), next_round=state['next_round'], world=w,
                **state['worlds'][str(w)], offers=keep(state['offers']), agreements=keep(state['agreements']),
                agreement_files=keep(state['agreement_files']), statements=keep(state['statements']),
                race_control=state['race_control'])


def summary(state):
    rows = []
    for w, x in sorted(state['worlds'].items()):
        last = max((s for s in state['statements'] if s['world'] == int(w)), key=lambda s: s['round'], default=None)
        rows.append(dict(world=int(w), seat=x['seat'], credits=x['credits'],
                         last_round=last and dict(round=last['round'], score=round(last['score'], 1)),
                         files_open=sum(1 for f in state['agreement_files'] if f['world'] == int(w) and f['status'] == 'open'),
                         lots_open=len(x['lots'])))
    return dict(team=state['team'], as_of=state.get('as_of'), next_round=state['next_round'], worlds=rows)


def main(a):
    if not a or a[0] in ('-h', '--help'):
        sys.exit(__doc__)
    cmd = a[0]
    if cmd == 'state':
        s = get('/api/state')
        out = world(s, a[a.index('--world') + 1]) if '--world' in a else s
    elif cmd == 'summary':
        out = summary(get('/api/state'))
    elif cmd == 'auctions' and len(a) == 2:
        out = get(f'/api/auctions?w={int(a[1])}')
    elif cmd == 'statement' and len(a) == 3:
        out = get(f'/api/statement?r={int(a[1])}&w={int(a[2])}')
    elif cmd == 'readings' and len(a) == 3:
        return print(get(f'/api/readings.csv?r={int(a[1])}&w={int(a[2])}'), end='')
    elif cmd == 'rules':
        out = get('/api/rules.json')
    else:
        sys.exit(__doc__)
    print(json.dumps(out, indent=1))


if __name__ == '__main__':
    main(sys.argv[1:])
