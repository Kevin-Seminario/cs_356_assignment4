# Cross-ring transit pilot for Round 3

## Question and map reading

Can jknetwork buy reachability from a team that owns a link across the ring, then sell that reachability to a nearby AS at a profit? The Nov 16 Worlds screenshots show the following relevant installed hardware:

- **World 1:** Rapid Otters' microwave crosses IXP 0–4; 123455's ordinary link crosses IXP 3–7. Neither owner is directly adjacent to our AS 23 at IXPs 5 and 6, so this is not the simplest place to pilot a one-hop buy-and-resell chain.
- **World 2:** 123455 (AS 19) has an ordinary link from IXP 4 to IXP 0. We are AS 23 at IXPs 5 and 6, adjacent to AS 19. A buyer in sector 6, modeled as AS 27, can reach us at IXP 6 but not AS 19 directly.
- **World 3:** Lucky Comets (AS 18) has a microwave link from IXP 4 to IXP 0. We are AS 15 at IXPs 3 and 4, adjacent to AS 18. A buyer in sector 2, modeled as AS 11, can reach us at IXP 3 but not AS 18 directly.
- **World 4:** RedBull (AS 6) has a microwave link from IXP 1 to IXP 5. We are AS 23 at IXP 5, where the link arrives. Its far-side IXP 5 peers are also directly available to us. Toyotathon's ordinary link reaches IXP 1 from our side; it is not a unique far-side supplier for us.

The owners and endpoints above are interpreted from the screenshots and prior auction winners; agreement status and other teams' route policies are not fully visible in the screenshots. An installed crossing by itself carries no external route: its owner must sign with an AS at the far IXP.

## Controlled local model

We generated separate worlds from the kit's 32-AS `full.toml`, keeping its workload and seed fixed. Each world includes the installed crossing at the correct IXPs and a **hypothetical** free peering agreement between the owner and one far-side AS. World 2/3 use far AS 2. World 4 uses AS 17, which we can also peer with directly. Our AS runs the repository's submitted `switch.wasm`; all other ASes run the kit starter. Every compared run completed with no program stoppages and settled routes. The modeled seller pays its owner $0.50/Mb for transit when that variant calls for it, then charges its buyer either $1/Mb or $0.30/Mb. Agreements cap payment at $400 per round.

The kit's default export policy is central: a far-side route learned from a **peer** is not advertised to another peer. It can be advertised to a **transit customer**. Thus buying transit can add a far-side route, but it charges us on all bilateral traffic, not just traffic to that extra route. A buyer also has a free-peering alternative with us; an attractive transit offer must beat it for the buyer.

## Results: useful counterfactuals

Figures are **our utility / buyer utility**, in practice dollars for a 10-second full-ring run. Absolute values are not live Round 3 forecasts because the model omits competitors' actual programs, contracts, failures and workloads. Differences within each matched world are informative.

| Scenario | World 2: AS 23 / AS 27 | World 3: AS 15 / AS 11 | World 4: AS 23 / AS 27 |
| --- | ---: | ---: | ---: |
| Free peer with owner; buyer free peers with us | $768.79 / $725.91 | $802.80 / $743.40 | $814.22 / $724.20 |
| Free peer with owner; sell transit at $1/Mb | $873.65 / $656.17 | $910.94 / $674.67 | $949.04 / $634.52 |
| Free peer with owner; sell transit at $0.30/Mb | $799.73 / $730.09 | $835.62 / $749.99 | $853.17 / $730.39 |
| Buy owner transit at $0.50/Mb; sell at $0.30/Mb | $761.04 / $757.45 | $790.52 / $772.38 | $822.17 / $730.39 |

At $1/Mb, the buyer loses $70–$90 compared with simply peering with us, so the high-revenue version is not a compelling voluntary offer. At $0.30/Mb with a **free owner peer**, both sides gain over free buyer peering, but our gain is only about **$31–$39** per buyer. With the **paid upstream owner** at $0.50/Mb, our gain over the all-free-peering baseline is **−$7.75, −$12.27 and +$7.95** in Worlds 2, 3 and 4 respectively. The buyer gains in that variant, but the upstream payment consumes most or all of our margin.

The maximum modeled resale price at which the buyer is at least as well off as under free peering is approximately **$0.53/Mb in World 2, $0.51/Mb in World 3, and $0.35/Mb in World 4**. This is a ceiling in the local model, not a recommended live quote. Rival routes or a different traffic mix could lower it.

To isolate what the *crossing* adds, we removed the owner's far-side peer while keeping its hardware and the buy/sell contracts. At the $1/Mb resale price, the crossing's far-side peer raises our utility only **$35.34 in World 2 and $32.66 in World 3**. In World 4 the increase is **$0.16** because the modeled far AS 17 can already peer with us directly at IXP 5. Most of the apparently large selling gain is selling access to ordinary local ASes through us, not monetizing the across-ring link.

## Decision

The broad “buy any cross-ring transit, then resell” plan does **not** meet the team's goal of a very large utility jump on this evidence. In particular, do not buy RedBull transit in World 4 merely because RedBull owns a microwave link that lands at our own IXP. First identify specific destinations RedBull exports that we cannot get through local peering.

A smaller, plausible transit service is available when an adjacent buyer cannot reach one of our peers, the buyer demonstrably benefits beyond free peering, and our charge remains below that buyer's value of the extra routes. The best pilot candidates in these maps are World 2 AS 27 or World 3 AS 11. Before live offers, verify the owner's actual advertised prefixes/AS paths from our switch log and ask the buyer what routes it lacks. If our seller must buy paid upstream transit, negotiate a price low enough to preserve a margin. Keep free peering for routes that do not require paid transit.

Reproduction: run `python3 worlds/practice/build-cross-ring-pilot.py`, then `target/release/competitive_net_sim run-a4 worlds/practice/cross-ring-w2-peer_and_sell_low.toml --program switch.wasm --as 23 --json <output>` (substitute world and variant as needed). The generator itself records all modeled endpoints, prices and counterparties.
