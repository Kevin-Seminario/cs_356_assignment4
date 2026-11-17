# Programmable agreement system: a brief for your coding agent

Put this file in your A4 kit folder and tell your coding agent to read it.
It explains how agreements work from Round 3 and how to help your team
write, check and sign one. Agent: coach the team, do not decide for them.
Ask what they want from a deal before you write code, and explain every
number you propose.

## 0. Set up once

This brief needs the agreements pack from the website (`agreements-pack.zip`,
on the walkthrough page that the agreements page links to).

```sh
cd networkarena-a4                      # your kit folder
unzip -o ~/Downloads/agreements-pack.zip
cargo build --release --features wasm   # the simulator, rebuilt
```

The pack adds `examples/agreements/` and replaces the kit's SDK and simulator
source (`src/`, `switch_program_sdk/`, `switch_program_types/`), `README.md`
and `docs/a4-api-guide.md`. It does not touch `examples/a4_starter/`, the
team's own programs or `worlds/`. A switch program built before the pack
still builds and runs unchanged.

## 1. What changed

- From the Round 3 cutoff (Tue Nov 17, 10:00 AM Central) an agreement is one
  `.wasm` file. The form on the website is closed.
- An agreement signed on the old form runs to the end of its term and then
  ends, including one with an AI team. To keep it, the parties sign a file.
- AI teams use files like every team: a file may name one, and it signs or
  declines on its own. AI teams do not write in a file's thread.

## 2. What a file holds

One file carries the whole deal:

| Part | Meaning |
|---|---|
| `world` | the one world it applies to (1 to 4) |
| `parties` | 2 to 4 teams, by pseudonym or three-letter code |
| `links` | which links it opens between parties: `"peer"`, `"transit"` or `"backup"` (the first buys from the second) |
| `cap` | the most any one party pays in a round, in whole dollars |
| `rounds` | how many rounds it runs (1 to 3) before it renews |
| `terms` | named numbers the payment rule reads, e.g. `("price", "$/Mb", 0.30)` |
| `reads` | the deal in plain words, one clause a line; the website prints these |
| `settle` | the payment rule: code that runs once after every round |

## 3. The payment rule

`settle` gets the meter readings on the borders between the parties for the
round, the terms, the round count and up to 4 KiB of its own memory from the
round before. It returns what each party pays; payments sum to zero.

```rust
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let up = input.sum(0, 1, |_| true);          // party 0 -> party 1, all epochs
    let down = input.sum(1, 0, |_| true);
    let mut out = input.out();                    // nobody pays, memory kept
    out.pay(0, 1, wire::per_mb(input.term("price"), up.bytes + down.bytes));
    out
}
```

- Money and terms are in millionths: `input.term("price")` of $0.30 is 300_000.
- A reading has `bytes`, `on_time`, `late`, `lost`, `returned`,
  `returned_on_time`, per 5-second epoch and traffic class.
- `input.state` / `out.state` is the memory; `input.rounds_run` counts rounds.
- A file sees only the borders between its own parties.

The working examples are the six starters. The exact types (`SettleInput2W`,
`SettleOutputW`, the readings) are in `switch_program_types/src/lib.rs`, and
the `agreement_program!` macro is in `switch_program_sdk/src/lib.rs`. Section 4
of `docs/a4-api-guide.md` describes the older settlement programs, not files.

## 4. Start from a starter

`examples/agreements/` has six: `transit`, `peering`, `paid_peering`,
`backup`, `ramp` (price falls with a good record; uses memory) and `club`
(three parties share a cost). From Auction 2, `joint_bid` (in the Round 3
update) is a joint bid for an auction lot. Copy one, then change the parties, world,
numbers, words and, if needed, the rule.

```sh
cd examples/agreements/transit
cargo build --release --target wasm32-unknown-unknown
# -> target/wasm32-unknown-unknown/release/agreement_transit.wasm
```

## 5. Try a file in the kit before anyone signs

Copy a practice world and add the agreement at the end of the copy. In a
world file the parties are AS numbers and the numbers are written out; the
`parties`, `cap` and `terms` inside the `.wasm` are what the website uses.

```sh
cp worlds/practice/practice.toml worlds/practice/mine.toml
```

```toml
# at the end of worlds/practice/mine.toml
[[a4.agreement]]
parties = [1, 3]                       # AS numbers in this world
links = [[1, 3, "transit"]]            # AS 1 buys transit from AS 3
caps = [60, 60]                        # most each pays in a round, dollars
terms = { price = 0.30, min_on_time = 0.90, refund = 10.0 }
program = "../../examples/agreements/transit/target/wasm32-unknown-unknown/release/agreement_transit.wasm"
```

```sh
target/release/competitive_net_sim run-a4 worlds/practice/mine.toml --agreements-json out.json
```

`out.json` lists, per agreement, what each party pays in dollars (negative:
receives), the file's memory for the next round (`state`, hex) and `error`
if the rule failed. The run's printed table shows each AS's `in` and `out`
dollars with the deal. Run the world without the agreement to compare. To
test a rule that uses memory, run again with `rounds_run = 1` and
`state = "<hex from out.json>"` added to the `[[a4.agreement]]`.

## 6. From file to deal

1. Upload the `.wasm` on the website's agreements page (attach `lib.rs` so
   the others can read the rule). Uploading proposes it and signs for you.
2. Every other party signs or declines. All sign: it starts at the next
   cutoff. One declines: it is closed. Unsigned at the cutoff: it lapses.
3. It runs after every round and renews for its rounds unless a party
   declines the renewal. A counter-offer is a new file.

## 7. Before the team signs someone else's file

- The words are a summary; the code is what pays. Read the attached source.
  No source: ask for it in the agreement's thread, or do not sign.
- Check the cap: it is the worst round the team can have under this deal.
- For a two-party file the website shows what it would have paid on the last
  three rounds. Compare that with the team's statement for those rounds.
- Download the `.wasm` from the contract and run it as in section 5.

## 8. Is it worth it?

Compare the team's round with the deal and without it: Tier-1 fees saved,
customer dollars gained from traffic that now arrives on time, minus what
the deal pays out. The statement, the readings and the replay for each
finished round are on the website. Think first; then try both versions in
the practice worlds.
