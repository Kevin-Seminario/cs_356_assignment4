# Assignment 4 final report — working draft

**Team:** jknetwork
**Required submission:** five pages on Canvas by December 11, 2026, 11:59 p.m. Central.
**Status:** Round 1 is documented from our submitted program, four statements, and switch logs. Later rounds must be filled from their own results and replays. This file is a working draft, not a claim that planned Round 2 actions occurred.

The handout asks us to explain, round by round, **what we decided and why, what the evidence taught us, and what we changed for the next round**. The final report should spend its space on causal explanations and decisions rather than a list of scores. It is graded on evidence and reasoning, not on whether a decision made us money.

## Our starting model

We operate one AS in each of four worlds. Each AS has two hubs joined by a crossing, access switches and homes under the hubs, company traffic, two Tier-1 exits, and possible neighbors at its two IXPs. The Tier-1 always reaches every destination but charges per Mb. A signed agreement can expose a peer route, which can shorten a trip and avoid that fee. We used the usual valley-free interdomain policy: prefer customer routes, then peer routes, then providers, with the Tier-1 last; announce our own or customer routes broadly but do not carry traffic between two peers for free. Our switch then chooses an available exit using the offered routes and shortest paths through our own AS. [Source: submitted program](../../examples/a4_round1/src/lib.rs), [routing helper](../../examples/a4_round1/src/router.rs), and [policy](../../examples/a4_round1/src/policy.rs).

## Round 1 — first live run, November 10

### Decision before the cutoff

We submitted the tested switch program archived as [switch.wasm](../../switch.wasm). It used the usual route policy and installed destination-prefix forwarding entries. Within our AS it used hop distance to reach an offered border exit, so a local switch did not have to send everything through one hub. With no better route, packets took the nearest Tier-1 exit. We made one deliberate data-plane exception: when Trading traffic had only the Tier-1 default route, we dropped it before paying the Tier-1 fee. Trading's target is 30 ms while the Tier-1 link alone takes 32 ms, so that route cannot meet the deadline even before other delays. Trading on a specific peer route still forwarded. The program logged its selected exits and periodic per-port drops. It did **not** detect failed internal links, change routes after a failure, use a backup transit route, or prioritize queues by traffic class. Those limitations mattered when interpreting losses; a route was selected at startup and could become stale.

We sought free peering because the starter policy uses a direct peer route when one is available, reducing distance and Tier-1 usage without a settlement payment. The startup logs show direct peer routes to seven ASes in World 1, seven in World 2, six in World 3, and four in World 4. These are **observed routes**, not proof that every earlier offer had been accepted. [World 1 log](evidence/round-1/log-w1-r1.log), [World 2 log](evidence/round-1/log-w2-r1.log), [World 3 log](evidence/round-1/log-w3-r1.log), [World 4 log](evidence/round-1/log-w4-r1.log).

### What the statements and logs show

| World | Utility ($) | Score | Tier-1 fee ($) | Home Calls on time | Zoom on time (`q`) | Jane Street on time (`q`) | Logged egress drops |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 1 | 1,789.57 | 56.05 | 1,825.08 | 73.4% | 74.3% | 15.3% | 5 |
| 2 | 1,835.62 | 58.00 | 1,792.20 | 88.4% | 87.5% | 13.4% | 0 |
| 3 | 1,411.97 | 44.96 | 1,925.64 | 71.0% | 71.2% | 12.0% | 0 |
| 4 | 1,054.43 | 31.78 | 2,112.60 | 49.1% | 48.5% | 8.9% | 165 |

The scores use the handout's `(utility − floor)/(par − floor)` formula with each statement's own floor and par. [World 1 statement](evidence/round-1/statement-w1-r1.json), [World 2 statement](evidence/round-1/statement-w2-r1.json), [World 3 statement](evidence/round-1/statement-w3-r1.json), [World 4 statement](evidence/round-1/statement-w4-r1.json).

World 4 was the clearest shortfall. It had the fewest observed direct peer routes, the largest Tier-1 fee, and the weakest Calls and Zoom on-time fractions. All 165 logged egress drops were at switch 231's port 2, which the startup log identifies as a Tier-1 border. That supports investigating the Tier-1 exit, but **does not prove** that those drops caused every late Call: a late packet need not be dropped, and some packets may have been delayed elsewhere or affected by failures. The zero or low drop counts in the other worlds make a blanket “our buffers were too small” explanation untenable. The statements show no program failure and no route-settlement failure. Their `failed` field is about settlement programs; it does not tell us whether a physical link failed. We need the Round 1 replay and per-border meter readings to identify any failure interval or congestion path before claiming a specific cause.

Our local full-ring tests used other ASes on the kit's starter. They did not reproduce the live competitors' company delivery fractions. Company revenue depends on our `q²` relative to the other three ASes in our sector, so improving a latency-sensitive route can change our share even when a local starter-only test shows little change in dollar payout. This is why we treated local tests as comparisons of routes and capacity, not as a prediction of live utility.

### Response after Round 1, to verify at the Round 2 cutoff

The immediate proposal was to seek free peering with every reachable neighbor still lacking an agreement, particularly same-sector neighbors, because this can reduce Tier-1 traffic and shorten Calls paths. Screenshots taken November 11 showed only one agreement displayed for World 1 and none for Worlds 2–4 for the next round. Before the cutoff we must verify which Round 1 agreements actually renewed and which new offers were accepted; a sent offer by itself cannot change routing. We also compared auction options against the evidence: a second cable did not improve utility in our local World 2 or World 4 baseline tests, and a larger World 4 hub buffer did not materially improve the result. A link across the ring could improve latency and Tier-1 cost only if we first obtain useful far-IXP agreements; its bid and placement must be judged with that condition in mind. **Record the actual signed agreements, winning bids, installation, and submitted program here after the cutoff.**

**Replay evidence to add before final submission:** a timestamped view of one World 4 Tier-1 congestion episode; whether any internal or IXP link failed; the destination and class of traffic affected; and the corresponding border meter reading. Keep observation separate from inference.

## Round 2 — November 12

**Decision and rationale:** [Record the program version, accepted agreements by world, auction purchases and installation, and why each was chosen.]
**Evidence:** [Insert statement values, relevant replay moment, port or border measurements, and comparison with Round 1.]
**Lesson and change for Round 3:** [Explain what improved or failed, with a competing explanation considered.]

## Round 3 — November 17

**Decision and rationale:** [To complete after cutoff.]
**Evidence:** [Statements, meter readings, and a replay moment.]
**Lesson and change for Round 4:** [What we changed and why.]

## Round 4 — November 19

**Decision and rationale:** [To complete after cutoff.]
**Evidence:** [Statements, meter readings, and a replay moment.]
**Lesson and change for Round 5:** [What we changed and why.]

## Round 5 — November 30

**Decision and rationale:** [To complete after cutoff.]
**Evidence:** [Statements, meter readings, and a replay moment.]
**Lesson and change for Round 6:** [What we changed and why.]

## Round 6 — December 2

**Decision and rationale:** [To complete after cutoff.]
**Evidence:** [Statements, meter readings, and a replay moment.]
**Lesson and change for Round 7:** [What we changed and why.]

## Round 7 — December 4

**Decision and rationale:** [To complete after cutoff.]
**Evidence:** [Statements, meter readings, and a replay moment.]
**Lesson and change for Round 8:** [What we changed and why.]

## Round 8 — December 7

**Decision and rationale:** [To complete after cutoff.]
**Evidence:** [Statements, meter readings, and a replay moment.]
**Lesson and change for Round 9:** [What we changed and why.]

## Round 9 — December 9

**Decision and rationale:** [To complete after cutoff.]
**Evidence:** [Statements, meter readings, and a replay moment.]
**Final lesson:** [Which design decision proved most important across the season, and what evidence would change our mind.]

## Evidence management and final edit

Archive each round's four statements, four logs, and a small number of replay screenshots or timestamped notes under `docs/report/evidence/round-N/`. Record offers separately from signed agreements and bids separately from installed hardware. Before submission, remove every bracketed prompt, trim to **five pages**, verify all numeric claims against statements, and export the final report for Canvas.
