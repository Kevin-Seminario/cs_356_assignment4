# BGP routing paper: what applies to our A4 network

Source: Hari Balakrishnan, *Wide-Area Internet Routing* (MIT 6.033, 2009), supplied by Jayden as `fcf7a1c903920c1f03b485ee143bdeaa_MIT6_033s18_bgp.pdf`. Relevant sections are 2.1-2.4 (business relationships and policy), 3.4-3.5 (route attributes and selection), 4.2-4.3 (failures and multihoming), and Appendix B (MED and multiple exits). This is a research-to-project mapping, not a change to the submitted program.

## 1. The paper's main economic lesson already matches our policy

An AS's business relationship determines which routes it imports and exports. Customer routes usually outrank peer routes, which outrank provider routes. Own and customer routes can be announced broadly; peer and provider routes go to paying customers, not to unrelated peers. Our `examples/a4_round1/src/policy.rs` implements that ordering and export rule. A4's `rank` and `announce` calls operate once while routes settle before the round, per destination **AS**. They cannot select a separate BGP path for Calls versus Video during the round. The switch receives only the chosen neighbor's routes, although it may choose among that neighbor's physical exits.

Implication for our niche: a transit customer can receive routes to our peers under the usual export rule. That is a genuine service we might sell. A peer cannot normally receive our other peers' routes for free. Announcing them anyway would promise unpaid transit and risks congestion, as the paper warns and A4 R23 expressly says.

## 2. The concrete unused lever: multiple-exit hints (MED)

The paper's Appendix B distinguishes **hot potato** routing (handoff at the nearest exit, leaving the neighbor to carry the long internal path) from **cold potato** routing (carry the packet toward the neighbor's destination before handoff). MED is a neighbor's hint about which entry point it prefers for a prefix, but honoring it may shift transport cost to us. The paper notes that peers often ignore MED without an incentive.

A4 exposes `med` on `A4RouteW`: lower means the neighbor's own block is nearer that link. The simulator derives it from hop distance within that neighbor; we cannot set an arbitrary outbound MED through the policy API. For routes the neighbor learned from another AS, A4 sets MED to zero, so zero is *not* evidence of a nearby destination on those routes. Our switch's `router::table` currently ranks exits by AS-path length, Tier-1 status, then hop distance inside our AS; it ignores MED. The policy's `rank` function does not receive MED at all.

**Safe experiment:** for a directly connected neighbor's own prefix, compare its two exits using our internal path to each exit plus the neighbor's MED, with a deterministic tie-break. Test an alternative that favors the neighbor's low-MED exit only when it improves a measured Calls path. Keep the relationship and AS path fixed. Do not prioritize MED across different ASes or on re-exported routes. Measure our crossing load, border Calls on-time, Zoom share, and utility, with and without a crossing or IXP-port failure. A lower MED can simply move the costly crossing from the neighbor's network to ours; a raw MED-first rule is therefore not justified.

**Controlled probe, November 15:** An isolated copy of the submitted switch ranked directly connected neighbors' own prefixes by our internal hop count plus their MED, favoring lower MED on ties. The probe was built in `/private/tmp/a4-med-probe`; no submitted or tracked switch code changed. In the existing World 3-like full-ring model with AS 14 peering, the baseline and probe both earned **$3,077.91**, delivered **29.52 Mb** of on-time home Calls, and paid **$1,524.84** to Tier-1. Startup logs confirmed that the probe actually changed some /17 exits: for example, switch 151 sent AS 14's `10.14.128.0/17` through its own crossing to switch 152 instead of handing it to AS 14 at switch 151. In the same model with **our crossing** (151–152) down from 5.4 to 7.2 seconds, baseline utility was **$2,920.31** and on-time Calls **27.84 Mb**; the probe fell to **$2,852.36** and **26.64 Mb**. Both ran with zero stopped programs. The simple MED-aware preference is therefore **rejected for submission**: no measured normal gain and a $67.95 failure loss. These are local practice-model results, not live-round forecasts. Revisit only with failure-aware exit selection or evidence that a neighbor's preferred entry yields enough benefit to offset our additional crossing risk.

The existing `examples/a4_backup_seller_probe` is a related proof of concept: in a local model a cooperative backup seller used the buyer's MED to carry returned traffic to the correct IXP. That model gained utility only when both the buyer's failover and the seller's exit selection worked; an agreement alone did not.

## 3. Why the World 3 Calls opportunity comes before MED tuning

In Round 2, World 3 had no selected direct route to same-sector AS 14. Its direct peer-border Calls were on time, while its Tier-1-border Calls were late or lost. A route to AS 14 cannot be improved by changing how we pick between two exits **until an agreement exposes it**. A matched local model adding free peering with AS 14 gained $578.33 utility and raised on-time home Calls from 22.24 to 29.52 Mb. This is directional model evidence, not a live-round prediction. First verify current agreements and obtain the missing route; then test MED-aware exits on that route and existing two-exit peers.

## 4. Failures and multiple exits

The paper treats multihoming as useful for resilience but warns that route changes can be slow or disruptive. A4 does not run full BGP updates during a round: the platform settles AS routes before the simulation, and our switch can react locally using timers, table changes, and offered backup exits. The live Round 2 World 1 crossing outage caused 883 drops at our switches because the submitted switch did not fail over. A local selective-failover candidate detected a silent crossing failure, but its site practice landslide was slightly worse than the submitted program. The next improvement must be tested against the specific failure class and route availability.

Do **not** copy Internet-scale mechanisms into this small simulator without evidence: route reflectors, AS-path prepending, arbitrary prefix origination, BGP keepalive sessions, and route-flap damping are not exposed by the A4 policy interface or address the current measured gap. Damping could delay useful local failover. A4 supplies each switch with the same local AS picture, and the simulator fixes each AS's own prefixes.

## Test order

1. Secure and verify World 3's missing same-sector route; compare matched runs with and without it.
2. Do not adopt the simple MED-aware exit probe. Its matched no-failure run tied the baseline, and its own-crossing-failure run lost $67.95. A future candidate would need to avoid the failed crossing and show a clear net gain under controlled tests.
3. If a neighbor's preferred exit benefits it while burdening our crossing, use a real, metered agreement incentive rather than promising unpaid carriage. Any payment rule should be tied to traffic that actually crosses and the observed on-time result.
4. Pursue backup transit only with both buyer failover and seller forwarding behavior verified. The agreement's payment cap bounds settlement, not packet volume or latency.
