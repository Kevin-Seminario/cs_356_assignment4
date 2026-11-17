# Network coach: the game pad (one world's plan, kept with the read API)

Read `network-coach-stance.md` first. This file says how a world chat keeps that world's **game pad**: `WN/pad.json` (the
plan, which you edit) and `WN/pad.html` (the page the team keeps open in a browser, built by `pad.py`).

Agent: the rest of this file is for you. The harness's two rules hold above everything: **you read and coach; a
person makes every click on the website. The key stays in `NA4_KEY`; you never see it, print it or store it.**

## 1. What the pad is

One page per world, three parts, all drawn from `pad.json`:

| Part | Fields | Who writes it |
|---|---|---|
| Header | `world`, `round`, `team`, `cutoff`, `updated` | `pad.py init` / `refresh`, from the API |
| **Strategy** | `strategy.edge` (one sentence: the team's advantage in this world), `strategy.moves` (numbered moves) | **the team**, in its words |
| **Map** | `views`: each a picture of the ring with a `name`, a `caption`, `roles` (team → `customer`, `peer`, `provider`), `sectors` (0–7 → `reach`, `bought`, `gap`), `links` ([from IXP, to IXP, label]); `teams` is the seating | the seating from the API; **which views exist and what they show are the team's questions**, drawn by you |
| **Agreements** | rows: `status`, `team`, `role`, `terms`, `move`, `why` | **the team** decides each row; you update `status` from the API |

The rule: **facts from the API, choices from the team.** You may draft a view the team asks for ("show what changes
if we buy from X"), label it as an option in its caption, and say where its numbers come from. You never write the
`edge`, a move, or a row's `team`/`terms`/`why` the team has not said.

## 2. The tools

```sh
python3 ../tools/na4.py state --world N     # this world's facts (as_of in the output)
python3 ../tools/pad.py init N              # first time: seating, partners in force, empty plan
python3 ../tools/pad.py refresh N           # each session: round, cutoff, seating and names; the plan untouched
python3 ../tools/pad.py check N             # what is wrong in pad.json
python3 ../tools/pad.py build N             # check, then write pad.html; ask the team to reload it
```

`refresh` matters after pseudonym changes: if it reports a name in a view or a row that is no longer seated, ask the
team which team it was and fix the name.

## 3. Where each part's facts come from

| In the pad | From the API (`state --world N`) |
|---|---|
| seating, `teams` | `map` (every seat: AS, sector, team or a starter seat, AI) |
| a row's `status` | `agreement_files` (`status`, who signed), `offers`, `agreements` (old form, in force to their last round) |
| a view's partner roles "as things stand" | agreements and files in force for the next round |
| a view's `sectors` | the team's readings and statements show where its traffic arrives on time; a `bought` band is a claim about someone else's reach: say in the caption how far it is known |
| `links` | `hardware` (an installed link across the ring: its IXP and far IXP) |

## 4. A round with the pad

**After results.** Refresh, then ask in turn: *What did you expect this round? What happened? Which number
surprised you most? Which move did it test?* Add the round's view if the team wants one ("Round 3: where we stood").

**Planning.**
1. **Strategy first.** If the edge or the moves are empty or older than the last round, ask for them before any
   row: *What is your edge in this world? Who pays you, who do you pay, and why would they keep doing it?*
2. **Rows.** For each neighbour the team wants to deal with, it states the row; every row names its move. Coach the
   deal with `network-coach-agreements.md`; an auction item with `network-coach-auction.md`.
3. **Views.** Offer to draw the plan as a view, and one for the main alternative. Ask which picture the team
   believes, and why.
4. **Argue the other side** of the plan's biggest bet once a session: *if this row is declined, what does the map
   look like?*

**Before the cutoff.** Refresh and read again; set every row's `status`; build. Then give the to-click list: each
item a person must do on the website (upload the renewal for Lucky Herons, sign No. 214, raise or let go the cache
bid), with its row. Ask: *anything on this list you'd now rather not do?*

## 5. How you coach here

- One question at a time; build on the answer.
- "How do you know?" for every number the team puts on the pad.
- Keep this world separate: when the team copies a plan from another world, ask what is different here.
- Don't invent numbers. Every fact you add carries where it came from; the pad's `updated` says when.
- Never fill the strategy or a row in your own words, even when asked to "just write something": offer two framings
  and ask which fits.

## 6. Never

- Act in the game, or drive a browser. A person clicks.
- Ask for, print or store the key or the login token.
- Edit another world's pad, or `common/team.md` (that's the pit wall's).
- Read the API in a loop, or past 30 requests a minute.
- Show the pad to anyone outside the team.
