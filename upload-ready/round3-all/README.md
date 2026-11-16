# Round 3 agreement files — human upload checklist

Prepared November 16, 2026 for the **Tuesday, November 17, 10:00 a.m. Central** cutoff. This folder keeps every compiled `.wasm` beside its source `.rs`. The files are drafts, not website submissions. Under A4 R34, a person must make every upload and signature. The other party must also sign before the cutoff.

**Assumption for this batch:** treat every Round 2 relationship as ended. Still check the live Agreements page before uploading: an already-active new-format file needs no duplicate. The `manifest.json` records each artifact SHA-256. All 44 files compiled offline; their embedded `na.deal` sections were checked for the right world, parties, relationship, one-round term, and payment cap. This does not prove any counterpart will sign or any live score.

A local full-ring smoke run with one new peer file and the World 2 GPT 2 transit file finished with zero stopped programs and no agreement errors. The peer paid $0; the transit buyer paid $106.48 in that synthetic run. That is a format and settlement check, **not** a Round 3 payment forecast.

## Priority review: preserve proven routes

- World 4 RedBull: preserve the free peer that advertised far-side ASes 1 and 3 in Round 2. Use `w4-peer-redbull.wasm`.
- World 3 Lucky Comets: preserve the free peer that advertised a route to AS 2. Use `w3-peer-lucky-comets.wasm`.
- World 2 GPT 2: renew the **existing paid transit sale**, not a free peer. Use `w2-transit-gpt-2.wasm`; GPT 2 buys from us at $1/Mb, cap $400 per round, one round then renewal unless declined. Round 2 paid us $89.28; buyer acceptance for Round 3 remains unknown.
- Review same-sector local peers next, then other direct peers. Losing a signed peer can push traffic back to Tier-1. The files below are **options**, not an instruction to upload all 39 renewals. Do not infer a signature from an uploaded file.

## Where paid transit could beat free peering

Our usual routing policy exports routes learned from peers to a **transit customer**, but not to a **peer**. A neighboring-sector team may therefore gain routes across our sector by buying transit from us. A same-sector team often has many of those routes already and is a weaker buyer. The buyer must gain destinations it can use, and our AS must have those routes signed and enough capacity to carry the traffic.

- **Already demonstrated live:** World 2 GPT 2 bought transit and paid us $89.28 in Round 2. Its renewal file is in this package.
- **Pending decision:** World 3 Claude 2 has our $1/Mb, $400-cap transit offer and its own free-peer offer, each with only one signature. The transit model hit the $400 cap, but that is not a live forecast. Choose one relationship; do not stack them.
- **Possible replacements for an expiring free peer:** World 1 GPT 1 or GPT 2; World 2 Claude 2; World 4 GPT 1 or Claude 1. Matched practice models showed a genuine route-export difference, but the seller gain mostly came from a synthetic payment at the cap. These are **negotiation candidates**, not proven Round 3 wins. World 3 Claude 1's new peer file was already reported in force for R3–R5, so it is not a quick conversion.

A transit template charges for **every Mb crossing the bilateral link**, including traffic to the buyer's own AS that may already cross under free peering. For example, our World 1 RedBull purchase cost $75.58 on 94.48 Mb, while only 25.60 Mb concerned its one extra advertised destination; even the maximum fee saving on that extra traffic was $38.40. A buyer needs enough *new* route value to justify paying for *all* metered bytes. Do not send a paid-transit file on top of a free-peer file for unchanged service. Use the specific buyer's current routes and a matched model before proposing a conversion.

For a conversion candidate, decide between paid transit and free peering **before** signing a replacement. If the paid offer is not mutually signed before the cutoff, a signed free-peer renewal is the fallback for preserving direct reachability. An unsigned paid offer and an unsigned peer offer both leave the route absent.

## Candidate renewal files

Each peer file below has **$0 payment**, one round, and a renewal unless either side declines. The World 1 RedBull file deliberately changes its earlier paid-transit relationship to free peering; RedBull must agree. World 3 Claude 1 may already have a new file in force for R3–R5; check before proposing another.

### World 1

- [ ] **GPT 2** — `w1-peer-gpt-2.wasm` (free peer)
- [ ] **RedBull** — `w1-peer-redbull.wasm` (free peer)
- [ ] **spatulameow** — `w1-peer-spatulameow.wasm` (free peer)
- [ ] **fastnetworkteam** — `w1-peer-fastnetworkteam.wasm` (free peer)
- [ ] **Jumbo Comets** — `w1-peer-jumbo-comets.wasm` (free peer)
- [ ] **Packet Lanterns** — `w1-peer-packet-lanterns.wasm` (free peer)
- [ ] **dinnyrob19** — `w1-peer-dinnyrob19.wasm` (free peer)
- [ ] **GPT 1** — `w1-peer-gpt-1.wasm` (free peer)
- [ ] **Lucky Comets** — `w1-peer-lucky-comets.wasm` (free peer)
- [ ] **Network Champions** — `w1-peer-network-champions.wasm` (free peer)
- [ ] **bevo_baddies** — `w1-peer-bevo-baddies.wasm` (free peer)

### World 2

- [ ] **Claude 2** — `w2-peer-claude-2.wasm` (free peer)
- [ ] **Bmobile** — `w2-peer-bmobile.wasm` (free peer)
- [ ] **123455** — `w2-peer-123455.wasm` (free peer)
- [ ] **Lucky Herons** — `w2-peer-lucky-herons.wasm` (free peer)
- [ ] **Lucky Comets** — `w2-peer-lucky-comets.wasm` (free peer)
- [ ] **fastnetworkteam** — `w2-peer-fastnetworkteam.wasm` (free peer)
- [ ] **Rapid Otters** — `w2-peer-rapid-otters.wasm` (free peer)
- [ ] **network** — `w2-peer-network.wasm` (free peer)
- [ ] **GPT 2** — `w2-transit-gpt-2.wasm` (paid transit)

### World 3

- [ ] **Network Champions** — `w3-peer-network-champions.wasm` (free peer)
- [ ] **fastnetworkteam** — `w3-peer-fastnetworkteam.wasm` (free peer)
- [ ] **JalebiStreet** — `w3-peer-jalebi-street.wasm` (free peer)
- [ ] **Stray Otters** — `w3-peer-stray-otters.wasm` (free peer)
- [ ] **Claude 1** — `w3-peer-claude-1.wasm` (free peer)
- [ ] **Lucky Comets** — `w3-peer-lucky-comets.wasm` (free peer)
- [ ] **spatulameow** — `w3-peer-spatulameow.wasm` (free peer)
- [ ] **networkarena** — `w3-peer-networkarena.wasm` (free peer)

### World 4

- [ ] **RedBull** — `w4-peer-redbull.wasm` (free peer)
- [ ] **GPT 1** — `w4-peer-gpt-1.wasm` (free peer)
- [ ] **Rapid Otters** — `w4-peer-rapid-otters.wasm` (free peer)
- [ ] **123455** — `w4-peer-123455.wasm` (free peer)
- [ ] **dinnyrob19** — `w4-peer-dinnyrob19.wasm` (free peer)
- [ ] **Toyotathon** — `w4-peer-toyotathon.wasm` (free peer)
- [ ] **Anycast Badgers** — `w4-peer-anycast-badgers.wasm` (free peer)
- [ ] **networkarena** — `w4-peer-networkarena.wasm` (free peer)
- [ ] **Claude 1** — `w4-peer-claude-1.wasm` (free peer)
- [ ] **Packet Lanterns** — `w4-peer-packet-lanterns.wasm` (free peer)
- [ ] **Lossless Pilots** — `w4-peer-lossless-pilots.wasm` (free peer)

## Optional new peers — separate from renewals

These were absent from our selected direct routes in Round 2. They may add reachability, but the partner identity, current IXP availability, signature state, and utility should be checked before offering. They are **not** required to preserve a Round 2 agreement. The World 3 AS 14 model had the largest estimated gain, but it was not a live forecast.

- [ ] World 2: **Lossless Pilots** — `w2-peer-lossless-pilots.wasm` (free peer candidate)
- [ ] World 2: **Silent Falcons** — `w2-peer-silent-falcons.wasm` (free peer candidate)
- [ ] World 3: **Anycast Badgers** — `w3-peer-anycast-badgers.wasm` (free peer candidate)
- [ ] World 3: **Net_doesnt_work** — `w3-peer-net-doesnt-work.wasm` (free peer candidate)
- [ ] World 4: **Silent Falcons** — `w4-peer-silent-falcons.wasm` (free peer candidate)

## World 3 Claude 2 — choose one, do not stack

Claude 2 had an incoming free-peer file (No. 167, signed by Claude 2) and our outgoing paid-transit file (No. 234, signed by us) when the site was inspected on November 16. Neither had both signatures. Choose the relationship based on actual extra routes and economics. Do not upload a duplicate just because it is absent from this folder. Do not sign both merely to collect payment on unchanged traffic.

## Before a person uploads

1. Check the current Agreements page for an active new-format relationship or pending duplicate.
2. Confirm the counterparty and world, and open the adjacent `.rs` if reviewing a payment.
3. Upload only the chosen `.wasm`; wait for the other party to sign. For incoming files, inspect the sender's source, clauses, cap, and payment preview.
4. Before 10:00 a.m. Central, confirm each required deal says **signed/in force for Round 3**. A sent offer alone does not open a link.
5. Record signed and declined status in the team handoff. Route changes occur only at the cutoff.

**No new switch program is included here.** The Round 3 storm test and any switch upload are separate decisions. Auction 2 caches cannot operate until Round 4.
