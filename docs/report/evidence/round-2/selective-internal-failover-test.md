# Selective internal-link failover experiment

**Question.** Can we recover valuable Calls traffic during an internal crossing failure without sending all stranded traffic through paid Tier-1? This is a local experiment against the submitted program; it is not a live Round 2 result or a claim about the hidden world.

**Candidate.** [`examples/a4_selective_failover`](../../../../examples/a4_selective_failover/) retains the submitted route policy, route ranking, queue behavior, and Trading guard. It sends hellos on internal ports every 100 ms and declares a silent link down after 300 ms. When a route disappears because a hub crossing failed, it permits Calls (DSCP 3) to use the Tier-1 default while blocking other classes for that prefix. When an access link failed and the destination is unreachable even with the crossing restored, it blocks every class for that prefix. A recovered route is installed before its block is removed. This candidate does not detect border failures.

**Controlled runs.** The World 1-like world has ten direct peers plus RedBull transit at $0.80/Mb, approximating the Round 2 startup log. The crossing schedule fails AS 23's two hubs from 5.4 to 7.2 seconds. The access schedule instead fails a hub-to-access link for that same interval. All comparisons use the same local practice world and traffic. Other ASes run the kit starter.

| Case | Submitted utility | Selective utility | Difference | Submitted Tier-1 fee | Selective Tier-1 fee | Calls on time, submitted → selective |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| World 1-like, crossing failed | $3,059.87 | $3,121.09 | **+$61.22** | $1,262.40 | $1,268.88 | 26.40 → 27.44 Mb |
| World 1-like, access failed | $2,950.84 | $2,949.94 | −$0.89 | $1,222.68 | $1,222.68 | 26.56 → 26.56 Mb |
| World 1-like, no failure | $3,294.15 | $3,293.99 | −$0.16 | $1,262.40 | $1,262.40 | 29.20 → 29.20 Mb |
| Five-peer practice, crossing failed | $1,979.50 | $2,023.55 | **+$44.05** | $1,982.64 | $1,985.52 | 19.76 → 20.40 Mb |

The earlier broad fallback candidate lost $31.60 in the World 1-like crossing case: it recovered $111.92 of delivered value while adding $143.52 in Tier-1 fees. The first selective version handled that crossing profitably but lost $62.42 on an access-link failure; it sent Calls toward an unreachable host and added $60.96 in Tier-1 fees. The revised candidate distinguishes the two failures. Its access-failure replay installs all-class blocks; the crossing replay installs Calls-bypass blocks. The revised access run has no extra Tier-1 fee. All four revised runs report zero stopped programs.

**Decision.** Keep the tested candidate available for a site dry run before changing the live submission. Local practice evidence supports a gain for this crossing schedule, but cannot establish benefit in the hidden live worlds or for border-link failures. The small no-failure and access-case differences remain costs to watch.
