# World 1 RedBull agreement counterfactual

This is a controlled **local practice** comparison, not a prediction of the hidden live world and not a signed agreement. The [transit model](../../../../worlds/practice/failover-w1-eval.toml) approximates our Round 2 World 1 routes with ten direct peers and AS 23 buying RedBull transit at $0.80/Mb. The [peering model](../../../../worlds/practice/failover-w1-free-redbull.toml) changes only that one agreement to free peering. Each model is run with the submitted switch and the separate v5 selective failover candidate, with and without the same 5.4–7.2 second hub crossing failure.

| Program | Crossing | RedBull deal | Utility | Tier-1 fee | RedBull payment | Calls on time |
| --- | --- | --- | ---: | ---: | ---: | ---: |
| Submitted | Up | $0.80/Mb transit | $3,294.15 | $1,262.40 | $84.35 | 29.20 Mb |
| Submitted | Up | Free peering | $3,330.59 | $1,304.04 | $0 | 29.20 Mb |
| v5 candidate | Up | $0.80/Mb transit | $3,293.99 | $1,262.40 | $84.35 | 29.20 Mb |
| v5 candidate | Up | Free peering | $3,330.43 | $1,304.04 | $0 | 29.20 Mb |
| Submitted | Down | $0.80/Mb transit | $3,059.87 | $1,262.40 | $80.64 | 26.40 Mb |
| Submitted | Down | Free peering | $3,163.42 | $1,304.04 | $0 | 27.44 Mb |
| v5 candidate | Down | $0.80/Mb transit | $3,121.09 | $1,268.88 | $80.64 | 27.44 Mb |
| v5 candidate | Down | Free peering | $3,156.46 | $1,310.52 | $0 | 27.44 Mb |

In the no-failure submitted-program comparison, free peering saves the $84.35 settlement, adds $41.64 in Tier-1 fees, and lowers delivered value by $6.27, net **+$36.44**. In the crossing-failure submitted-program comparison, free peering is **+$103.55**. With transit, v5 improves crossing utility **+$61.22** over the submitted switch; with free peering, v5 is **−$6.96** because it adds $6.48 in Tier-1 fees without increasing on-time Calls. Thus the agreement can change whether the fallback is worth using.

The actual Round 2 live RedBull settlement was **$75.584** on **94.48 Mb** of bilateral customer traffic. The only extra destination its transit advertised was AS 14; the replay showed **25.60 Mb** concerning AS 14. At $1.50/Mb, the *maximum fee-only* Tier-1 saving on that traffic is $38.40, below the payment. A direct comparison must still account for delivery value and route changes. RedBull may decline free peering, and an overlapping signed transit deal can remain the effective routed relationship. The local model gives a negotiating direction, not permission to claim live savings.

**Decision use:** Prefer a mutually signed free peer deal if available; otherwise negotiate a lower transit rate or a conditional settlement and check it against actual measured incremental routes. Decline unwanted renewal of an existing agreement before its renewal cutoff. Do not assume a new signed deal preserves v5's measured failover benefit; rerun the combined agreement and program scenario.
