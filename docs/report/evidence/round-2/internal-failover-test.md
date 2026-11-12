# Isolated internal-link failover experiment

**Question.** Would detecting a silent internal crossing failure and rebuilding our forwarding table improve utility? This tests a new candidate against the currently submitted `a4_round1.wasm`; it does not change the live submission.

**Candidate.** [`examples/a4_failover_candidate`](../../../../examples/a4_failover_candidate/) copies the submitted policy, route rank, queue behavior, Trading-on-Tier-1 drop, and logging. It adds a small hello on every internal port every 100 ms. After 300 ms with no hello, that switch marks the local port down and recomputes its routes without the edge; receipt of a hello restores the route. It preserves route entry IDs when a port changes, including the default-route ID used by the Trading guard. It does not monitor border links or select a backup contract.

**Controlled schedule.** [`failover-w1-crossing.toml`](../../../../worlds/practice/failures/failover-w1-crossing.toml) fails the AS 23 hub crossing from simulation time 5.4 to 7.2 seconds, matching the link type and timing seen in the live World 1 Round 2 replay. Both programs run in the *same local practice world* with the same fixed traffic, agreement set, and failures. The practice world is not the hidden live world.

| Practice agreement set | Failure? | Submitted utility | Candidate utility | Change | Candidate added value | Candidate added Tier-1 fee |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Five direct peers in `queue-eval.toml` | No | $2,146.72 | $2,146.16 | −$0.56 | −$0.56 | $0 |
| Five direct peers in `queue-eval.toml` | Yes | $1,979.50 | $1,994.91 | **+$15.41** | +$49.61 | +$34.20 |
| World 1-like agreements in [`failover-w1-eval.toml`](../../../../worlds/practice/failover-w1-eval.toml) | No | $3,294.15 | $3,294.15 | $0 | $0 | $0 |
| World 1-like agreements in [`failover-w1-eval.toml`](../../../../worlds/practice/failover-w1-eval.toml) | Yes | $3,059.87 | $3,028.27 | **−$31.60** | +$111.92 | +$143.52 |

The World 1-like agreement set approximates the Round 2 startup log: AS 23 has ten direct peers and buys RedBull transit from AS 18 at $0.80/Mb; AS 18 peers with AS 14 so its transit can expose the extra AS 14 route. It does not reproduce live partner programs, live customer traffic, or the actual hidden topology. Other ASes use the kit starter. The no-failure pair confirms that the candidate did not materially alter ordinary operation in this more relevant setup. Both programs completed all runs without stopping, and interdomain routes settled.

The candidate's replay shows the failed crossing at 5.400 seconds, both hub switches declaring it down at 5.701 seconds, restoration at 7.200 seconds, and both switches declaring it up around 7.211 seconds. The repair of route entries follows those declarations. While down, packets that had used the crossing can fall back to the Tier-1 default, which restores some delivery but costs $1.50/Mb. In the World 1-like test, the extra fee exceeded the recovered delivered value by $31.60. Calls delivered on time rose from 26.40 to 27.44 Mb in that failed run, but that gain was not sufficient to pay for all the extra Tier-1 use.

**Decision.** Do not submit this broad fallback candidate based on these results. A possible next experiment is selective failover: allow expensive Tier-1 detours only for traffic classes or destinations whose recovered value plausibly exceeds the fee. The current evidence does not prove that such a policy would win, and it does not solve the separate World 2 border-link failure.
