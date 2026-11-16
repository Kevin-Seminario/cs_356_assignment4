# Regional bridge pilot for World 3

## Decision and service

Our World 3 seat is AS 15. Claude 2 is AS 9 in the neighboring sector. AS 15 can connect to sector-4 ASes 17–20 while AS 9 cannot connect to them directly from its seat. Offer Claude 2 a **one-round transit purchase** from jknetwork, rather than a second free-peering agreement on the same border. The service is reachability to AS 15's own, customer-learned, and peer-learned routes. This is valuable only while we actually have useful routes to export and enough capacity to carry the added load.

The candidate agreement is `examples/agreements/regional_bridge/src/lib.rs`: Claude 2 pays jknetwork **$1/Mb crossing their border in either direction**, capped at **$400 per round**. The cap limits payment, not traffic. It does not promise end-to-end on-time delivery, grant access to Tier-1 routes, or grant access to routes we bought from a provider. The source and readable clauses say the same thing. These terms are an opening offer for team review, not proof that the buyer will accept or benefit in the live round.

## Agreements to preserve the product

The bridge requires active, mutually signed routes into sector 4. For Round 3, verify which existing World 3 peer agreements actually continue under the new agreement-file rules. Prioritize renewing the direct peer relationships with Claude 1 (AS 17), Lucky Comets (AS 18), spatulameow (AS 19), and networkarena (AS 20) where each party wants to continue. A bilateral free-peering file for each is easier to sign than a four-party bundle. Confirm the current Worlds mapping before sending any file; these names come from the earlier supplied map, not a live roster. The AS 9 transit file does not itself open those other borders.

Before offering the AS 9 transit file, confirm there is no overlapping free peering between AS 9 and AS 15 for the same round. The added service must be genuine. The new file needs both parties' signatures and begins at the next cutoff. An offer or unsigned file changes nothing.

### Prepared World 3 peer renewals

Three separate one-round, $0-payment peer files are ready under `examples/agreements/w3_peer_lucky_comets/`, `w3_peer_spatulameow/`, and `w3_peer_networkarena/`. Each names jknetwork and the exact partner pseudonym from the Round 2 border readings. The one-round term renews for one unless either party declines; it avoids committing to several rounds before the bridge's first live result. Claude 1 has already offered a separate peer file, so review that offer instead of proposing a duplicate.

The compiled peer files were tested together in the same World 3 full-ring practice scenario as the bridge contract. Replacing the three old-form peer templates with the new files left AS 9, AS 15, and the three peers' modeled utility exactly unchanged; each new file settled at $0 with no error, and no switch program stopped. This verifies the file format and the modeled route relationship, not future signatures or live traffic. None of these files affects Round 3 until the human uploader proposes it and the other party signs.

## Switch policy

`examples/a4_regional_bridge/src/policy.rs` changes only AS 15's announcement to AS 9 when AS 9 is a customer. It exports routes learned from ourselves, customers, or peers. It withholds routes learned from providers or backup relationships, whose marginal cost may exceed the pilot price. Ranking, every other export, and packet forwarding remain as in the submitted switch. A route that is announced is a promise to carry traffic; the switch does not discriminate by traffic class after making that promise.

No data-plane routing change is needed for the first bridge pilot: the A4 route-settlement phase supplies the selected routes to our switch, which already selects a physical exit. Our submitted root `switch.wasm` is deliberately unchanged. The candidate wasm is `examples/a4_regional_bridge/target/wasm32-unknown-unknown/release/a4_regional_bridge.wasm`; it must be dry-run and reviewed before replacing the upload.

## Local matched evidence, November 15

Using the same 32-AS full-ring practice world and starter programs for other ASes, with our AS 15 running either the submitted switch or the candidate:

- With Claude 2 as a transit customer and current peer routes, the candidate and submitted switch were **identical** on every AS's utility, delivery, Tier-1 fee, and settlement. AS 15 utility was $2,976.10 and AS 9 utility was $1,913.61 in both. The $400 payment cap bound. This proves only that the export filter is dormant in this model; it does not make the switch candidate a current earnings improvement.
- Against the same model with no AS 9 agreement, the transit scenario improved AS 15 by $476.52 and AS 9 by $2,033.85. These large modeled gains are **not live predictions**: the model omits their real programs, routes, links, agreements, and workload. The actual World 2 GPT 2 transit paid us only $89.28, whereas this model reached $400.
- In a deliberately altered case where AS 15 **buys** transit from AS 18, the candidate withholds that provider-learned route from AS 9. AS 15 utility rose $13.09 versus the submitted switch because it paid AS 18 less; AS 9 utility fell $33.10 because it used more Tier-1 and received slightly less on-time Video/Web. This is a cost-control tradeoff, not an unqualified improvement. The deal must still be attractive to AS 9 under its real alternatives.
- Both programs completed all matched runs with zero stopped programs. The new settlement file returned no error and paid $400 to AS 15 in the synthetic traffic.

## Next measurement

Use a matched dry run and the next live statement to compare AS 15 and AS 9 utility, bilateral bytes/payments, AS 15's own Calls on-time, Tier-1 fees, and any queue or link drops. If the bridge takes capacity from our $35/Mb Calls, transit revenue may be outweighed by home loss. If direct peer renewal fails, stop marketing routes we no longer have. Do not infer a live profit from the synthetic $400 cap hit.
