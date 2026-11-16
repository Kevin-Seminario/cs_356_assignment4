# World 3 same-sector peering renewals

These two independent agreement files preserve our direct peer routes to the
other World 3 sector-3 teams. They are ready for human upload on the Agreements
page. Attach the matching `.rs` source with each `.wasm` so the partner can
review what it signs.

- `jalebistreet.wasm` and `jalebistreet.rs`: jknetwork with JalebiStreet (AS 13).
- `stray-otters.wasm` and `stray-otters.rs`: jknetwork with Stray Otters (AS 16).

Each agreement opens a peer link, pays $0 in both directions, runs one round,
and renews for one unless either side declines. It starts only after both
parties sign. The old-form agreements end at their term; check the website
before uploading to avoid a duplicate new-format agreement.

Local check: both programs built for `wasm32-unknown-unknown` and settled at
$0 without error in the World 3 six-buyer storm scenario. Replacing the two
old-format practice peer entries with these files left modeled utility exactly
unchanged. This validates file behavior in the kit, not the other teams'
signatures or future traffic.
