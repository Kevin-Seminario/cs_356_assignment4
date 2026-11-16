# World 3 peer candidates

This folder puts three prepared bilateral agreements in one place. These are
**candidates, not a directive to renew all three**. For each partner, first
check whether its route gives a paying transit customer useful reachability
or saves enough Tier-1 cost and on-time value to justify the link. Also check
whether that partner is itself a better buyer. If selected, upload its `.wasm`
file as a **separate** agreement on the game's World 3 Agreements page and
attach the matching `.rs` source file. Every other party must sign.
Check the live page first: an already signed or pending file need not be sent
again.

- `lucky-comets.wasm` and `lucky-comets.rs` — Lucky Comets.
- `spatulameow.wasm` and `spatulameow.rs` — spatulameow.
- `networkarena.wasm` and `networkarena.rs` — networkarena.

Each agreement opens one peer link with jknetwork, has no payment, and lasts
one round before renewing unless either party declines. Claude 1 has already
proposed a separate peer agreement; evaluate its route value and terms before
signing. Claude 2's paid regional-transit offer is separate and has already
been sent. Do not maintain a free peer to a proposed buyer merely to preserve
the old relationship.

The source crates remain under `examples/agreements/`. If their source changes, rebuild the wasm and refresh this folder before uploading anything.
