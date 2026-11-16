# World 1 peer candidates for Round 3

Each `.wasm` is a **separate bilateral offer** from jknetwork to the named partner. Attach the matching `.rs` file so the other team can review the source. All files specify a World 1 peer link, $0 payment, a $0 cap, and a one-round term that renews for one round unless either side declines. A submitted file opens no route until the other team signs it.

## Decide each relationship against the paid-transit plan

These eight files are an inventory of possible renewals, **not a send-all list**.
Round 2's selected routes show that each of these peers supplied its own AS;
none of these eight was observed supplying a further AS. A high bilateral
volume alone does not establish that renewing a peer helps the regional
bridge. Before sending each file, identify whether this route is a useful
destination for a prospective paying customer, or whether the direct path
avoids enough Tier-1 cost or improves enough on-time delivery to justify it.
Check whether the counterparty is instead a credible transit buyer. Compare
the *combined* agreement set, because buyer and supplier gains overlap.
Retain no peer merely because it was a peer last round; likewise, do not
cancel a valuable route merely to create a paid-buyer candidate.

1. `core/dinnyrob19.wasm` and `.rs` — 150.08 Mb in Round 2.
2. `core/packet-lanterns.wasm` and `.rs` — 148.64 Mb.
3. `core/jumbo-comets.wasm` and `.rs` — 145.28 Mb.
4. `core/spatulameow.wasm` and `.rs` — 72.48 Mb.
5. `core/lucky-comets.wasm` and `.rs` — 72.24 Mb.
6. `core/bevo-baddies.wasm` and `.rs` — 69.84 Mb. The team's site pseudonym is `bevo_baddies` inside the file.
7. `core/network-champions.wasm` and `.rs` — 68.80 Mb.
8. `core/fastnetworkteam.wasm` and `.rs` — 34.72 Mb.

## Handle separately

- **GPT 1 and GPT 2:** Both already sent World 1 peer files for our review (site agreements No. 48 and No. 85 as viewed November 15). Inspect their source and exact terms. The alternative paid-transit files are in `../worlds124-transit-candidates/world1-conditional/`. Choose peer or customer only after confirming the extra routes available to that buyer, the status of its old/new peer term, and both parties' incremental utility. Do not sign a free-peer file by reflex or stack it with paid transit to charge for unchanged service.
- **RedBull:** The old $0.80/Mb transit renewal was declined. `optional-redbull/redbull.wasm` and `.rs` offer **free peering**, which gives different reachability from transit. Send only if RedBull agrees to that relationship; do not restart the paid transit merely to keep a border route. This file is deliberately outside `core/`.
- **Packet Lanterns and Jumbo Comets backup offers:** The pending old-form backup-transit offers are separate from these peer renewals. They remain unsigned and do not replace a peer agreement. Our submitted switch does not currently use the backup route, so do not count on these offers for Round 3 recovery.

## Evidence and verification

Round 2 traffic figures come from `docs/report/evidence/round-2/readings-w1-r2.csv`; the team names and local AS numbers were checked against that file and the read-only World 1 Agreements page. The eight core files were built for `wasm32-unknown-unknown` and run together in the kit's 32-AS practice world: all eight settled at $0 with no agreement errors and no stopped switch programs. Adding the optional RedBull file produced the same result for all nine agreements. This checks file execution, not whether each counterparty will sign or how the live Round 3 world will perform.

**Peer versus paid-customer check:** A customer also supplies its own route,
so the bridge strategy does not inherently require these partners to remain
peers. We tested converting Jumbo Comets, Packet Lanterns, and dinnyrob19
individually from free peers to $1/Mb transit customers in the matched storm
model, with the old RedBull transit removed and the two AI seats left as
peers. Each conversion collected the modeled $400 cap but lowered jknetwork
utility by about $493, $493, and $418, respectively, relative to keeping
that partner a peer. The modeled loss came chiefly from lower company value,
not higher Tier-1 fees. In the real Round 2 meter, their bilateral traffic
was only about 145-150 Mb each, far below the model's billed $400. This is
directional evidence to *test* these three as supplier peers before asking
them to buy transit, not proof of their live behavior. The choice remains
per partner and depends on the full signed agreement set.

Source crates live under `examples/agreements/w1_peer_*/`. Rebuild and refresh the copied `.wasm` and `.rs` files if a source crate changes.
