# Network coach: the routing policy (`rank` and `announce`)

Read `network-coach-stance.md` first: it says how to coach. This file is
the topic: what the team decides, where the facts are, and the questions
to ask, in order.

## 1. What the team decides

Two functions in its one switch program, which runs in all four worlds:

- `rank`: for each destination, which of the routes it was offered it
  uses;
- `announce`: for each neighbour, whether to tell it about the route it
  uses.

Leaving them out gives the usual rule.

## 2. Where the facts are

- **Rule book** (the last section of the handout): the clauses "Routing
  policy", "Announcing is a promise", "Settling" and "Backups", and "The
  Tier-1 fee" and "Score".
- **`docs/a4-api-guide.md`**:
  - §2: what `rank` and `announce` receive and return, what the platform
    does for you, settling, failure, no memory between calls;
  - §5: the practice worlds, and "Trying an agreement locally".
- **`examples/a4_starter/src/policy.rs`**: the usual rule, as code.
- **The website, after each round**, for each world:
  - **What the team's program did:** one row per agreement, giving the
    routes that neighbour offered, the destinations `rank` chose it for,
    the routes announced to it, and the Mb each way. It also splits the
    Tier-1 fee into what was sent and what was received. Route counts
    show a dash for a round without a route record.
  - **Downloads:** the statement, `readings.csv`, the switches' log and
    the replay.

## 3. Questions, in order

Ask one at a time. Move on when the team can answer from its own files,
not before.

### 3.1 What the usual rule does

- In the rule book's "Routing policy" clause, in what order does the
  usual rule rank routes? To whom does it announce which routes?
- In `policy.rs`, which lines do that? Does your program change them? If
  so, how, and why?
- What happens for the whole round if your policy traps, runs out of
  fuel or answers out of range? Where would you see that it happened?

### 3.2 What you were offered, and what you chose

- In one world, which neighbours can offer you routes at all? (Rule book:
  you are offered only routes your agreements cover.)
- After the last round, how many destinations did your traffic reach
  through the Tier-1? For how many of those had a partner offered you a
  route?
- Pick one destination. List the routes you were offered: the neighbour,
  who it is to you, the AS path. Which did you use? Which would the usual
  rule use? Why are they the same, or why not?
- What did that choice cost or earn you? Where in the statement do you
  see it? What does the rule book charge per Mb through the Tier-1, and
  what does that agreement charge?

### 3.3 Fast, short or cheap

- What does your team want from a route? Where are each class's price
  and on-time target written?
- `rank` sees the neighbour, who it is to you and the AS path. Which of
  those tell you something about money? About delay? What does `rank`
  not see at all?
- `rank` returns one route per destination. Your traffic comes in
  several classes. What does that mean for what `rank` can and cannot
  decide, and what is left to the rest of your switch program?
- In your readings, how did each class do through each neighbour? Is the
  route with fewer ASes the one that did better here? How sure are you,
  from how many readings?

### 3.4 What announcing brings in

- The rule book says announcing is a promise. When you announce a route
  to a neighbour, what do you agree to carry? Who pays you for carrying
  it, under the agreements you hold?
- After the last round, what did you announce to each neighbour, and how
  many Mb came in from it? The Tier-1 fee counts both directions: how
  many Mb came in through the Tier-1?
- Under the usual rule, which of your routes does a peer hear? A transit
  seller? A customer? What would change if you announced more to one of
  them, and what does the rule book call that?
- If you announce less, where does the traffic that would have come to
  you go? What happens to the money?

### 3.5 Everyone's policies at once

- Your routes are the outcome of every AS's policy. Read "Settling" in
  the rule book. What happens when routes do not settle, and how would
  you know, from your statement or from a kit run?
- On paper: can you build two or three ASes whose policies keep changing
  each other's routes? What about the usual rule stops that? (The
  handout asks you to explain this.)
- If you change your `rank`, whose routes besides yours could change?
  How would you notice, in a kit run or on the website?
- The rule book and API guide §2 say when routes are worked out. When a
  link fails in the middle of a round, what moves your traffic: your
  policy, or the rest of your switch program?

### 3.6 A design, then a test

1. Ask the team to state the change in its own words: what `rank` or
   `announce` does differently, for which routes, why, and which number
   should move, in which direction.
2. Write exactly that, explaining every constant.
3. Test it in the kit:
   - **Give `rank` a choice.** In the practice worlds every AS reaches
     the others only through the Tier-1, so add agreements to a copy of
     the world (API guide §5).
   - **Run it.** Put the team's program in one AS, with the starter
     everywhere else (`--program ... --as N`).
   - **Predict first.** Before each run, ask the team to say what it
     expects.
   - **Compare** utility, the Tier-1 fee, home traffic on time, and
     whether the run printed that routes settled, and after how many
     passes. Use `--json` for the full statements.
   - **Run it again with the landslide.**
4. Ask what would have to be true in the team's real worlds for the
   result to carry over. API guide §5 says the practice worlds lack the
   other teams, their agreements and the failures. And one program runs
   in all four worlds: how would this change behave in each?
5. After the next round, ask the team to compare what its program did,
   round by round, against its prediction.

## 4. Not here

- Do not suggest an order for `rank`, a rule for `announce`, or a kind of
  neighbour to prefer.
- Do not say what other teams' policies are, or what scores well.
- Do not write policy code before the team has stated the design.
- Do not treat real-Internet practice as true in this game until the
  team has checked it in its own files.
