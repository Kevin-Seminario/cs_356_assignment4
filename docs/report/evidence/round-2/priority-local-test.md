# Calls-priority candidate and agreement snapshot — November 11

The user-provided NetworkArena world screenshots taken around 18:01 Central show agreements scheduled for the Round 2 cutoff. The site's summary reads 9 renewals plus 3 starts in World 1, 7 plus 2 in World 2, 6 plus 1 in World 3, and 5 plus 3 in World 4. These totals count agreements, including repeated counterparties, rather than distinct routes. The visible World 4 counterparties include Toyotathon, Rapid Otters, dinnyrob19, 123455, and networkarena. We have not yet measured Round 2 routing or settlement.

We copied the kit's `full.toml` into [queue-eval.toml](../../../../worlds/practice/queue-eval.toml) and added five local free-peering agreements between our World 4 seat, AS 23, and ASes 18, 19, 20, 21, and 24. The test put our program in AS 23 and every other AS on the kit starter. The original submitted program was the baseline; the separate [priority candidate](../../../../switch-priority-candidate.wasm) changed only queue selection and configuration. Each pair of runs used the same world, seed, and load. There were zero stopped programs.

| Local run | Baseline utility | Candidate utility | Baseline Calls on time | Candidate Calls on time | Tier-1 fee, both |
| --- | ---: | ---: | ---: | ---: | ---: |
| Five peers, normal load | $2,146.16 | $2,146.72 | 21.76 Mb | 21.76 Mb | $1,982.64 |
| Five peers, twice the load | $3,007.80 | $3,006.42 | 43.04 Mb | 43.04 Mb | $2,394.96 |

With four peers, the candidate was slightly worse than the baseline at both loads. With no peers, a preliminary version that divided the original buffer between two queues was worse again, so the candidate gives the bulk queue its original cap and uses the switch-wide shared pool as the total bound. The final candidate still showed no Calls improvement in these tests. The sub-$2 utility changes here are small and switch sign; they do not establish a gain. The local model is not the live game: real opponents, agreements, traffic, and failures may differ. We kept the submitted `switch.wasm` unchanged pending a site dry run.
