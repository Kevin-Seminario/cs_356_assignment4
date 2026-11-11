# World 4 microwave planning test — November 11

This is a **local counterfactual**, not a replay of the live Round 1 world. It supports the team's auction rationale and should not be presented as a measured Round 2 gain. As of November 11, all the team's microwave bids had been outbid across the four worlds; the team owned no microwave, so this modeled configuration was not deployed.

We ran the kit's full 32-AS practice ring for 10 seconds. Our AS used the submitted `switch.wasm`; every other AS ran the kit starter. The baseline had the four direct peering relationships observed in our World 4 Round 1 switch log. The alternative added a microwave link from our hub at IXP 5 to a new switch at IXP 1, plus two hypothetical free peering agreements at that far IXP. The world file scaled the real 50 Mb/s microwave capacity to 5 Mb/s, as required by the practice world's one-tenth traffic scale; its latency was 12 ms. Nothing about these two hypothetical partners guarantees they will sign in the live game.

| Local scenario | Utility ($) | Tier-1 fee ($) | Jane Street on-time fraction |
| --- | ---: | ---: | ---: |
| Existing four peers, no microwave | 1,527.79 | 2,105.88 | 0.1000 |
| Microwave and two hypothetical far peers | 1,608.47 | 2,065.68 | 0.1750 |

The local utility difference was $80.69, including about $40.20 in avoided Tier-1 fees. This model put the other ASes on the starter, so it did not reproduce the live competition for company contracts. In the [actual World 4 statement](statement-w4-r1.json), our Jane Street on-time fraction was 0.0892 and the company payout depended on the three other sector ASes' fractions. Applying the modeled *change* in our on-time fraction to that live payout while freezing competitors would imply a larger gain, but their future routes, traffic, and delivery can change. The link's value also vanishes until a far-side agreement is active.

The test used the source and hardware behavior described in the [A4 API guide](../../../a4-api-guide.md) and [hardware handout](../../../assignment/a4-hardware-handout.md). If this result is cited in the final report, label it as a model and pair it with the actual next-round statement and replay.
