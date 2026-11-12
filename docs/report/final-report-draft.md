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

### What failed, and what remains uncertain

World 4 was the clearest **performance and economic failure**, not a program crash. It had only four observed direct peer routes, the largest Tier-1 fee ($2,112.60 for 1,408.4 Mb), and the weakest Calls and Zoom on-time fractions. Only 15.36 of 31.30 Mb of our home Calls arrived on time. Our Jane Street on-time fraction was 8.9%, particularly costly because its company contract is shared according to squared on-time fractions. Compared with World 1, World 4's utility was $735.15 lower: home revenue was $356.80 lower, company revenue $90.83 lower, and the Tier-1 fee $287.52 higher. This is an accounting comparison across different worlds, **not proof that four rather than seven peers caused the entire gap**.

All 165 logged egress drops in World 4 were at switch 231's port 2, which the startup log identifies as a Tier-1 border. With the kit's 1,000-byte packets, those drops represent at most 1.32 Mb, while 15.94 Mb of offered home Calls missed the deadline. The logged drops alone therefore cannot explain the Calls result, even if every dropped packet were a Call. Additional packets could have arrived late without being dropped, or been lost elsewhere. Worlds 2 and 3 logged no egress drops despite imperfect Calls delivery, further weakening a simple “increase our buffers” diagnosis. [World 4 log](evidence/round-1/log-w4-r1.log) and [statement](evidence/round-1/statement-w4-r1.json).

Our design also lacked active link-failure detection and class-based queueing. A failed crossing could leave a precomputed route pointing at it; a shared queue could let bulk traffic delay Calls or Trading. These are **plausible weaknesses**, not confirmed causes of the Round 1 result. The statements show no switch-program failure and no route-settlement failure. Their `failed` field concerns settlement programs, not physical links. We need the Round 1 replay and border meters to identify any failure interval, path change, or congestion episode before attributing specific losses to one mechanism.

Our local full-ring tests used other ASes on the kit's starter. They did not reproduce the live competitors' company delivery fractions. Company revenue depends on our `q²` relative to the other three ASes in our sector, so improving a latency-sensitive route can change our share even when a local starter-only test shows little change in dollar payout. This is why we treated local tests as comparisons of routes and capacity, not as a prediction of live utility.

### Response after Round 1, to verify at the Round 2 cutoff

The immediate proposal was to seek free peering with reachable neighbors still lacking an agreement, because this can reduce Tier-1 traffic and shorten Calls paths. The newer November 11, 18:01 site screenshots show agreements scheduled at the Round 2 cutoff: World 1 reports 9 renewals and 3 starts; World 2, 7 and 2; World 3, 6 and 1; World 4, 5 and 3. Some counterparties appear more than once, so these are agreement counts rather than numbers of distinct peers. World 4's visible list includes the four direct peers seen in our Round 1 log plus networkarena. These are the site's scheduled view before the cutoff, not measured Round 2 routes. After the cutoff we must check what routes actually appeared; a sent offer by itself cannot change routing. We also compared auction options against the evidence: a second cable did not improve utility in our local World 2 or World 4 baseline tests, and a larger World 4 hub buffer did not materially improve the result.

**World 4 microwave decision and auction outcome:** We considered a microwave link across the ring because World 4 had our weakest score (31.78), highest Tier-1 fee, and lowest Trading on-time fraction. The ordinary cross-ring link takes 22 ms and carries 100 Mb/s; microwave takes 12 ms but carries only 50 Mb/s. In a local full-ring test with two *hypothetical* far-side peers and the other ASes on the starter, microwave saved about $40 in Tier-1 fees and raised Jane Street's on-time fraction from 0.100 to 0.175. That was a counterfactual, not a measured live gain, and required signed partners at the far IXP. We observed an existing bid of 28 credits on November 11 and set a provisional 35–40-credit ceiling without a firm far-side partner. Our microwave bids across the four worlds were outbid. **After the auction closed, we owned no cross-ring link.** The auction screenshots show one World 4 program-memory upgrade in our garage, won for 3 credits and not yet installed; it does not improve the current small program's routing or delivery. Worlds 2 and 3 show no item and 100 credits each; World 1 shows no item and 80 credits. The screenshots do not explain World 1's 20-credit difference, so the ledger is needed before attributing it. Our next-round plan relies on signed local peering and our existing routes. If another team's cross-ring link terminates at one of our IXPs, a direct agreement with that owner could reach the owner's own AS; reaching the owner's other peers through it requires transit and an appropriate export policy. [Auction snapshot](evidence/round-2/auction1-snapshot.md), [model assumptions and results](evidence/round-1/world-4-microwave-model.md), [hardware handout](../assignment/a4-hardware-handout.md), and [World 4 statement](evidence/round-1/statement-w4-r1.json).

**Record the final auction ledger, actual signed agreements, installations, and submitted program here after the cutoff.**

**Program experiment before Round 2:** We built a separate [Calls-priority candidate](../../examples/a4_priority/src/lib.rs) and [WASM](../../switch-priority-candidate.wasm). It places Calls in a strict-priority queue while leaving other classes in the bulk queue; each queue has the original per-port packet cap, while the simulator retains the switch-wide shared buffer limit. It preserves the Round 1 routes and Trading-on-Tier-1 drop rule. In a local full-ring model with AS 23 peered to ASes 18, 19, 20, 21, and 24, its utility changed from $2,146.16 to $2,146.72; at twice the traffic load it changed from $3,007.80 to $3,006.42. In both cases Calls delivered the same number of on-time Mb and Tier-1 fees were identical. Earlier tests with four peers and with no peers also did not show a Calls gain. This evidence does not justify replacing the live submitted [switch.wasm](../../switch.wasm) yet. The local [test world](../../worlds/practice/queue-eval.toml) and [comparison notes](evidence/round-2/priority-local-test.md) make the experiment reproducible. A site dry run and a later live result would be needed to justify a change of submitted program.

**Replay evidence to add before final submission:** a timestamped view of one World 4 Tier-1 congestion episode; whether any internal or IXP link failed; the destination and class of traffic affected; and the corresponding border meter reading. Keep observation separate from inference.

## Round 2 — November 12

**Plan before cutoff:** Keep the existing submitted `switch.wasm`: the separate Calls-priority candidate showed no meaningful gain. Depend on the signed local peering already scheduled, especially in World 4. Auction winners with possible relevance are RedBull's microwave in Worlds 2 and 4, Lucky Comets' microwave in World 3, and ordinary cross-ring links owned by 123455 in World 2 and Toyotathon in World 4. The latter two are already direct peers; reaching other far-side ASes through them would require a transit agreement and usable exported routes. The auction screenshots did not show their installations or far IXPs, so auction ownership alone implied no new route. Check the scheduled agreements and their terms before the cutoff; after the run, compare direct routes, Tier-1 fees, home Calls on-time Mb, and company on-time fractions with Round 1.

**Agreement correction:** The signed $0.80/Mb RedBull transit, capped at $400 for one round, was in **World 1**, not World 4. World 4 had **free RedBull peering**. World 1 also briefly had overlapping signed free peering and transit with RedBull; the Round 2 switch log shows RedBull as a provider, so the effective routed relationship was transit. The accepted $1/Mb GPT 2 transit in World 2 made GPT 2 our customer. A full signed-agreement inventory should still be retained from the website; offers alone did not affect this run.

**Measured Round 2 results:** The same submitted switch program ran in all four worlds with no program failures or unsettled routes. Utility and normalized score (each round's own floor/par) were: World 1 **$2,157.60 / 82.27**, World 2 **$2,185.34 / 70.22**, World 3 **$1,696.28 / 54.09**, and World 4 **$2,265.87 / 72.32**. Relative to Round 1, the utility changes were **+$368.03, +$349.72, +$284.31, and +$1,211.44** respectively. The four-world score sum rose from approximately **190.79 to 278.90**. These are cross-round comparisons with different failures and traffic; they are not isolated treatment effects of any one agreement. [Round 2 replay event audit](evidence/round-2/replay-event-summary.md), with the statements, logs, and readings in the same evidence folder.

**World 1:** Tier-1 fees fell by **$618.00**, but home/company value fell by **$174.39** and we paid RedBull **$75.584**, for a **+$368.03** net utility change. RedBull was the selected provider for its own AS 18 and one additional destination, AS 14. Its border meter recorded **94.48 Mb** total, matching the $0.80/Mb settlement. The replay shows approximately **25.6 Mb** of that total concerned AS 14, the only extra route; even if all of it otherwise used Tier-1, avoiding $1.50/Mb there saves at most **$38.40**, below the $75.584 transit payment. That comparison excludes possible differences in timeliness and company-contract revenue and is not a full free-peering counterfactual. At **5.4–7.2 seconds**, our own link between hub switches **231 and 232** failed. The unchanged static forwarding entries produced **883 link-drop events at our switches** during the failure window; the program did not reroute. [Statement](evidence/round-2/statement-w1-r2.json), [meter](evidence/round-2/readings-w1-r2.csv), and [switch log](evidence/round-2/log-w1-r2.log); failure endpoints and drop counts from the downloaded Round 2 replay.

**World 2:** Tier-1 fees fell **$404.04** and GPT 2 paid us **$89.28** for transit, while home/company value fell **$143.60**; net utility rose **$349.72**. The switch log confirms GPT 2 is a customer at AS 25. It also shows peer 123455 (AS 19) carrying routes to ASes 2 and 29. At **2.0–3.0 seconds**, our IXP link from switch **232** to Rapid Otters' switch **261** failed, producing **50 link-drop events at our switches**. [Statement](evidence/round-2/statement-w2-r2.json), [meter](evidence/round-2/readings-w2-r2.csv), and [switch log](evidence/round-2/log-w2-r2.log); failure endpoints and drop counts from the replay.

**World 3:** Utility rose **$284.31**, chiefly because Tier-1 fees fell **$378.00** while home/company value fell **$93.69**. Lucky Comets (AS 18) advertised a route to AS 2. The replay contains no physical-link or switch failure. It does contain **400 TTL drops at our switches**, evenly spread across the run, but these packets had protocol **91**, a control/probe packet rather than customer traffic. There were no logged egress queue drops. [Statement](evidence/round-2/statement-w3-r2.json), [meter](evidence/round-2/readings-w3-r2.csv), and [switch log](evidence/round-2/log-w3-r2.log); TTL classification from the replay.

**World 4:** Utility rose **$1,211.44**: Tier-1 fees fell **$798.12** and home/company value rose **$413.32**, with no settlement payment. Our home Calls delivered on time rose from **15.36/31.30 Mb** to **26.32/29.84 Mb**. Free peer RedBull (AS 6), using its cross-ring position, advertised routes to ASes **1 and 3** in addition to its own AS. The replay shows no physical failure, and our switch log has no egress queue drops. The combination of added reachability and no failure is consistent with the large improvement, but the files do not isolate their individual contributions. [Statement](evidence/round-2/statement-w4-r2.json), [meter](evidence/round-2/readings-w4-r2.csv), and [switch log](evidence/round-2/log-w4-r2.log).

**Lesson for Round 3:** Free peering to a partner with customer routes across the ring can be very valuable (World 4); paid transit is only justified by the additional destinations or on-time value it actually provides (World 1 was weak on the measured fee-versus-route comparison). The most concrete program gap is failure handling: the World 1 hub crossing and World 2 IXP-link failures left static routes aimed at broken links. We built a separate internal-link hello and route-repair candidate and compared it against the submitted program with the same World 1-type crossing failure in two practice agreement sets. It improved utility by **$15.41** with five peers, but **reduced** utility by **$31.60** with World 1-like agreements: the extra Tier-1 fee was $143.52 versus $111.92 in recovered delivered value. We kept the submitted switch unchanged. This local counterfactual does not establish what would have happened in the live hidden world; it shows why reducing drops alone is not enough. [Controlled failover test](evidence/round-2/internal-failover-test.md). The World 2 GPT 2 transit earned money this round without observed queue drops, so its renewal merits consideration after the next results.

**Follow-up local test:** A more selective candidate lets Calls take paid Tier-1 only when an internal hub crossing removes a route; it blocks other classes, and blocks every class when an access link itself makes the destination unreachable. In the same World 1-like local crossing test it improved utility by **$61.22** versus the submitted switch, with Calls on time rising from **26.40 to 27.44 Mb** and Tier-1 fees rising only **$6.48**. In an access-link failure it changed utility by **−$0.89** with no added Tier-1 fee; with no failure, by **−$0.16**. All runs completed without program stops. These are practice-world counterfactuals; the live submitted `switch.wasm` remains unchanged pending a site dry run. [Selective failover evidence](evidence/round-2/selective-internal-failover-test.md).

**Site dry-run follow-up:** The v4 plain log confirms the selective candidate started on the four practice AS 1 switches, but it contains only startup entries and no runtime drop samples. That practice AS offered only Tier-1, so its startup routes do not test our live peering choices. We fixed the candidate's runtime log trigger to count timer callbacks and aggregate two 100 ms drop intervals; this leaves the no-failure local statement unchanged. The uploaded v4 result cannot be evaluated for utility from the log alone. [Log and diagnostic](evidence/round-2/selective-internal-failover-test.md).

**Site dry-run v5:** The logging fix produced 200 periodic samples. They recorded 1,582 egress refusals, all but one at practice switch 11's 4 Mb/s Tier-1 port; policy-drop counters were zero and no internal-link change was logged. The starter v3 log had 1,558 refusals over 196 slightly different sample windows, nearly all at the same port. The repeated congestion is a practice Tier-1 bottleneck, and the log alone still cannot establish utility or a live failover gain. [v5 log and comparison](evidence/round-2/selective-internal-failover-test.md).

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
