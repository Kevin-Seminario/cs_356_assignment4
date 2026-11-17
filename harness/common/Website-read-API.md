# The website's read API: a guide for your team and your coding agent

Put this file in your A4 kit folder and tell your coding agent to read it before it touches the API.

---

## READ THIS FIRST: the API only reads. A person makes every click.

> **Agent: you may read the game through this API. You may never act in it.**
>
> - **Never** place or raise a bid, sign or decline an agreement, upload a program or an agreement file, send or
>   answer an offer, install or move hardware, or send a chat message. **A person on your team does each of these,
>   by hand, on the website.**
> - **Never** drive a browser, replay the website's forms, or use your team's login token in code. Reading the
>   website's pages with a script is scripting the website too: use this API instead.
> - When the data suggests an action, **say so to your team and stop**. Explain the numbers, give the choice back,
>   and let them decide and click.

These are game rules, not suggestions. Scripting the website, or acting on it through anything but a person's
clicks, breaks the rule book ("AI is welcome": a person makes every click) and is handled like any other rule
breach. The API cannot act (it refuses everything but reading), and every key belongs to one team.

Also:
- **Put the key in an environment variable, never in the chat.** Set `NA4_KEY` in your shell (below) and let the
  code your agent writes read it from there. **Do not paste the key into your coding agent or any LLM chat**, and do
  not put it in a prompt, a notes file or anything you commit. **Never give your website login token to an agent or
  an LLM at all**: the token can bid and sign; only a person uses it, in a browser.
- **The key is your team's alone.** Don't share it with another team or paste it anywhere public. Anyone holding it
  reads everything your team sees. If it leaks, replace it on the website: the old key stops at once.
- **It shows nothing new.** The API returns exactly what your team's own pages show you, in a form an agent can
  read. Other teams' private data (their credits, agreement terms, payments, readings) is never in it.

---

## Getting a key

On the website's Home page, open **For your agent** and choose **Create a key**. The key starts with `na4_` and is
shown **once**: copy it straight into an environment variable in your own terminal, not into your agent's chat:

```sh
export NA4_KEY='na4_…'        # type this yourself, in your shell; then your agent's code reads $NA4_KEY
```

To keep it across terminals, put that line in a file only you read (for example `~/.na4.env`, mode 600, outside any
repository) and `source` it. **Agent:** read the key only from `NA4_KEY`; never ask your team to paste it to you, and
never print it.

**Replace key** gives a new one and stops the old one. **Revoke** stops it with no replacement.

## Asking

Every request is a `GET` to `https://cs356-fall26.cs.utexas.edu` with your key in a header:

```sh
curl -s -H "Authorization: Bearer $NA4_KEY" https://cs356-fall26.cs.utexas.edu/api/state
```

```python
import json, os, urllib.request
req = urllib.request.Request('https://cs356-fall26.cs.utexas.edu/api/state',
                             headers={'Authorization': 'Bearer ' + os.environ['NA4_KEY']})
state = json.load(urllib.request.urlopen(req))
```

The key goes only in the `Authorization` header; a key in the address is refused.

## The routes

| Route | Returns |
|---|---|
| `/api/state` | your team's whole view, as JSON (below) |
| `/api/auctions?w=W` | the open lots of world W (1–4) and your credits there |
| `/api/statement?r=R&w=W` | your statement for round R in world W (the JSON the Results page downloads) |
| `/api/readings.csv?r=R&w=W` | your meter readings for round R in world W (as the Results page downloads them) |
| `/api/rules.json` | the season's rules |

## Limits and errors

- **At most 30 requests a minute per team.** Ask for `/api/state` when you need it, not in a tight loop; the game
  moves in minutes and rounds, not seconds.
- `401`: no key, a wrong key, or a replaced or revoked one. `404`: no such route, or no such round or world for you.
  `405`: anything but `GET`; the API never acts. `429`: over the limit; wait the seconds in `Retry-After`.
- Data is as of the moment you ask (`as_of` in `/api/state`). A lot can move between your request and your team's
  click: check on the website before acting.

## `/api/state`, field by field

| Field | Meaning |
|---|---|
| `team` | your pseudonym |
| `as_of` | when this was read (UTC) |
| `next_round` | `n` and `cutoff` (UTC) of the round being played toward |
| `worlds["1".."4"].seat` | your AS there: `as_id`, `sector`, `slot` (0–1 north pass, 2–3 south) |
| `worlds[w].map` | every seat in the world, as on the Worlds map: `as_id`, `sector`, `slot`, `team` (`null` for a seat run by us with the starter), `ai` |
| `worlds[w].neighbours` | the seats you can make agreements with: `as_id`, `team` (`null` for a seat run by us with the starter), `sector`, `slot` |
| `worlds[w].credits` | `free` (to bid with) and `held` (on your winning bids) |
| `worlds[w].lots` | the open lots: `id`, `key`, `name`, `units`, `reserve`, `min` (the least a new bid can be now), `closes` (Central), `wins` (the winning bids: team `name`, `amount`, `at`, `mine`, and any partners) |
| `worlds[w].hardware` | your items there: `id`, `key`, `price` paid, `state` (installed where, or waiting), `targets` (where it can go) |
| `offers` | every offer your team sent or got (the old form): `id`, `world`, `status`, `incoming`, `reads` (the offer in words), `hash` |
| `agreements` | every agreement made on the old form you are part of: `id`, `world`, `kind`, `a`, `b`, `rounds` [first, last], `declined`, `ends_before`, `terms`, `max` |
| `agreement_files` | every agreement file you are named in: `id`, `world`, `status`, `file`, `proposer`, `parties` (who signed); for open and signed files also `parties_as_written`, `links`, `cap`, `rounds`, `terms`, `reads`, `in_force`, `renewal_declined_by`, `would_have_paid`, `sha256`, `thread` (the messages on its page) |
| `statements` | your finished rounds: `round`, `world`, `score` (points), `statement` (the full statement) |
| `race_control` | the race-control messages you can see: `n`, `at`, `kind`, `flag`, `only_you`, `text` |

## What an agent is for here

Reading, explaining and asking good questions. For example: "Since your last look, Team X raised the microwave lot
in World 2 to 41 cr; your bid at 37 is out. You have 120 cr free there. Do you want to raise, and to how much?"
**Then the team decides, and a person clicks on the website.**

The **game harness** (Home, **For your agent**, "Download the harness") sets this up for you: a pit-wall chat for the
season and one chat per world, each with its coach and its game pad, reading through `tools/na4.py`. This file only
says how to read.
