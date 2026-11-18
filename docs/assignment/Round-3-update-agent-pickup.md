# Round 3 update: a brief for your coding agent

Put this file in your A4 kit folder and tell your coding agent to read it.
It lists what changes from Round 3 and Auction 2, updates the kit, and
shows how to test the team's program and value an item before bidding.
Agent: coach the team, do not decide for them. Explain every number you
propose, and leave bids, signatures and uploads to the team.

## 0. Set up once

This brief needs the Round 3 update from the website
(`a4-round3-update.tar.gz`, on the Round 3 update page).

```sh
cd networkarena-a4                      # your kit folder
tar -xzf ~/Downloads/a4-round3-update.tar.gz
cargo build --release --features wasm   # the simulator, rebuilt
```

The update replaces `rules.json` and adds:

| File | What it is |
|---|---|
| `worlds/practice/failures/storm.toml` | one round in the shape of the failures from Round 3, on `full.toml` |
| `tools/try-hardware.py` | adds Auction 2's new items to a copy of a practice world |
| `docs/a4-auction-2-items.md` | what each Auction 2 item does |
| `examples/agreements/joint_bid/` | the starter for a joint-bid file (section 4) |

It also carries every file of the agreements pack (`examples/agreements/`,
`src/`, the SDK and types, `README.md`, `docs/a4-api-guide.md`), unchanged
since the pack, so a team that skipped the pack is now up to date. It does
not touch `examples/a4_starter/`, the team's own programs or the practice
worlds.

## 1. What changed

- **The weather turns: heavier failures from the Round 3 cutoff (Tue Nov 17,
  10:00 AM Central).** Traffic does not change.
  Each world, each round:
  - 6 or 7 passes go down for 2.7 s, each in a different sector (a pass
    takes down the crossings of both ASes in it), with start times spread
    over the run.
  - One IXP port (every IXP link of one hub) and one crossing on its own go
    down for 3 s each.
  - Until Round 2 it was 4 passes for 1.8 s, a port and a crossing for 2 s,
    at fixed times.
  - As before, every pass is hit equally often over the season. No team's
    ASes take more than 3 pass hits in a round. The Season page shows the
    commitment hash for Rounds 3 to 9.
- **Agreements are files from the Round 3 cutoff.** See
  `Programmable-agreement-system-agent-pickup.md` if the team has not set
  that up.
- **Auction 2: opens Mon Nov 16, closes Wed Nov 18 at 8:00 PM Central.**
  - New items: a content cache, a shared cache at an IXP and low-latency
    access, alongside links across the ring and repair crews.
  - There is no cap on how many units of a lot one team may win.
  - Any lot can be bought by a joint bid (section 4).
  - An item won can be installed from the Round 4 cutoff (Thu Nov 19,
    10:00 AM) and counts in Rounds 4 to 9.
- **`rules.json`.** New `hardware` keys `cdn_cache`, `shared_cache` and
  `low_latency_access`, and the Auction 2 lots in `auctions.lots`
  (`"auction": 2`). `provisional` is now `false`. No other number changed.

## 2. Test the program against the new failures

Put the team's program in one of the ASes the file hits. Edit the
`program =` line of that AS's `[[a4.as]]` row in a copy of `full.toml`.
The paths are relative to `worlds/practice/`.

- AS 3 or 4: their pass is down 1.3 to 4.0 s.
- AS 8: every IXP link of hub 81 is down 1.5 to 4.5 s.
- AS 6: its crossing alone is down 3.5 to 6.5 s.

```sh
cd worlds/practice
cp full.toml mine.toml                  # then edit the program line
../../target/release/competitive_net_sim run-a4 mine.toml --failures failures/storm.toml
../../target/release/competitive_net_sim run-a4 mine.toml   # the same world without failures
```

Compare the AS's utility and its home traffic on time in the two runs. With
every AS on the starter, AS 3 fell from $97 to −$8 and AS 4 from $74 to
−$39.

Questions to work through with the team:
- How fast does the program notice a dead link?
- Where does traffic go while the link is down?
- Does traffic come back when the link returns?

`--log` writes a replay for the viewer. The game deals new victims and
times every round and world, so do not tune the program to this one file.

## 3. Value an Auction 2 item before bidding

```sh
cd worlds/practice
python3 ../../tools/try-hardware.py mine.toml mine-cache.toml --as 1 cache
python3 ../../tools/try-hardware.py mine.toml mine-fast.toml --as 1 low-latency
python3 ../../tools/try-hardware.py mine.toml mine-shared.toml --as 1 shared-cache=2,17
../../target/release/competitive_net_sim run-a4 mine-cache.toml
```

- The script makes the same change the website makes to the game's worlds.
  Give an item twice for two units.
- Run the world with the item and without it, with the team's program in
  that AS, and compare its utility.
- With every AS on the starter, a cache raised AS 1 from $1 to $125 a run.
  Low-latency access gained the starter little, because it does not route
  by delay. A program that does may gain more.
- For `shared-cache=B,C`, list the partners at the IXP where the cache would
  sit. Their homes are served over the links between you and them.
- Links across the ring, repair crews and the older items are edits by hand:
  `docs/a4-api-guide.md`, section 6.
- `docs/a4-auction-2-items.md` says what each item does and why real
  networks buy it.

A practice world is not the game's world. Use it to compare choices, not to
predict a score. An item counts in six rounds (4 to 9), and credits buy
nothing else.

This section is the mechanics. Deciding what to bid for, in which world and
at what top price is the team's call: coach it with
`network-coach-auction.md` (with `network-coach-stance.md`), and do not
propose bids or prices yourself.

## 4. Joint bids

- Two to four teams can bid for any lot together; the shared cache is
  sold only to joint bids.
- A joint bid is an agreement file. Copy `examples/agreements/joint_bid`
  and set:
  - the parties, the first of which owns the item and installs it;
  - the term `("bid", LOT_KEY, total)`, with the lot's key from
    `rules.json` `auctions.lots`, e.g. `"shared_cache"`;
  - `share_1`, `share_2`, ... for each party, in party order, adding up
    to the total;
  - `links: []`, and a payment rule for after the auction (the starter's
    partners each pay the owner `fee` a round).
- Build it and upload it on the Agreements page; every team signs.
- At the last signature the auction takes the bid and holds each team's
  share from its own credits. It is refused, with nothing held, if a team
  lacks its share or the total is below the minimum by then. Unsigned when
  the lot closes, it lapses. Outbid: every share comes back; to raise,
  upload a new file.
- If it wins, the file runs every round to the end of the season and
  cannot be declined. The item stays where its owner first installs it,
  and cannot be moved or sold. A shared cache serves the homes of the
  file's other parties that sit at its IXP.

```sh
cd examples/agreements/joint_bid
cargo build --release --target wasm32-unknown-unknown
# -> target/wasm32-unknown-unknown/release/agreement_joint_bid.wasm
```

Before the team signs a joint bid, check the lot, the total, the team's
share, who owns the item, and what the payment rule does every round to
the end of the season. Section 7 of the agreements brief applies.

## 5. Before the cutoffs

- [ ] Update unpacked and the simulator rebuilt.
- [ ] Program tested against `storm.toml` and dry run on the website
      before the Round 3 cutoff (Tue Nov 17, 10:00 AM).
- [ ] Old-form agreements the team wants to keep re-signed as files
      before the same cutoff.
- [ ] A decision sheet (`network-coach-auction.md`, section 5) with a top
      price and a reason for every lot the team bids on.
- [ ] Bids placed, and joint bids signed by every team, before Auction 2
      closes (Wed Nov 18, 8:00 PM).
- [ ] Items won installed before the Round 4 cutoff (Thu Nov 19, 10:00 AM).
