# Round 3 execution plan — paid-transit market test

**Current plan, November 16, 2026.** This supersedes `round-3-final-decision.md`'s earlier instruction to sign every incoming free peer immediately. Round 3 cuts off Tuesday, November 17 at 10:00 a.m. Central. The November 16 screenshots show offers, not today's guaranteed signatures. A person must make every website click under R34. The four reviewable agreement files and their sources are in [`upload-ready/round3-paid-offers/`](../../upload-ready/round3-paid-offers/README.md).

## Central decision

Use **paid transit as a one-round market test** where we can give the buyer peer-learned routes that a free peer would not receive. Keep free-peer offers that are already signed by the counterparty as cutoff fallbacks. This strategy is more active than the earlier passive peer-renewal plan, but it is **not** a demonstrated $2,000 utility opportunity. The one live paid sale, World 2 GPT 2, paid $89.28 in Round 2. Our old practice models repeatedly hit a $400 cap; treating that as expected income was wrong.

## Per-world actions

1. **World 1 — pitch GPT 1, preserve the fallback.** Before signing its free-peer offer **48**, send the new `w1-gpt1.wasm`: GPT 1 buys transit from us at **$0.80/Mb**, capped at **$200**, for **one round**. It can then learn our peer routes; do not stack the paid relationship on a signed free peer. Sign the *other* valid zero-payment incoming peer offers and restore useful route suppliers. If GPT 1 does not sign paid transit by the internal 8:30 a.m. decision point, sign its already-offered free peer 48 before cutoff. If 48 has already been mutually signed, skip this paid pilot for Round 3.
2. **World 2 — preserve the existing GPT 2 customer and test Lossless Pilots.** Verify that the GPT 2 transit renewal is mutually signed; if not, use `upload-ready/round3-all/w2-transit-gpt-2.wasm`. Its live Round 2 payment was **$89.28**. Send `w2-lossless.wasm` to Lossless Pilots only as a genuine route-export sale: **$0.60/Mb**, **$200 cap**, **one round**. We had no selected direct route to them in Round 2. If they reject or cannot use our peer-learned routes, prefer their zero-payment peer file to losing reachability. Unlike Worlds 1, 3 and 4, there is no pre-signed incoming peer fallback visible in the screenshot; approach them promptly.
3. **World 3 — replace the pending Claude 2 quote if it is still unsigned.** The older offer **234** asks $1/Mb up to $400; the richer buyer test shows that price leaves little or no buyer surplus. If 234 has not already been mutually signed, withdraw it and send `w3-claude2.wasm` at **$0.80/Mb**, **$200 cap**, **one round**. Hold Claude 2's incoming free-peer offer **167** as fallback; it otherwise locks in R3–R5. Restore Lucky Comets and other peer route suppliers, and seek the missing Net_doesnt_work route (modeled +$578 for our utility even as a free peer). Sign 167 before cutoff if no paid agreement arrives.
4. **World 4 — pitch Claude 1, preserve the fallback.** Hold its already-offered free peer **147** while sending `w4-claude1.wasm`: Claude 1 buys at **$0.80/Mb**, **$200 cap**, **one round**. Restore the other peer route suppliers, especially RedBull if its free far-side routes are still offered. If no paid signature by the internal decision point, sign peer 147. If 147 was already mutually signed, skip this paid pilot for Round 3. Keep the separate Silent Falcons free-peer offer pending; do not withdraw it merely to sell an unproven transit deal. Decline dinnyrob19's $1.20/Mb *purchase* offer 564 unless new route evidence establishes value.

## Evidence and practical limits

We compared free peer versus paid transit on the same full-ring practice workload and published Round 3 sample storm. Our switch was current `switch.wasm`; other ASes used kit starters. To challenge the earlier isolated-buyer model, we gave each buyer direct peer agreements to **all other adjacent ASes**. With the actual compiled discounted files, paid transit still opened some peer-learned routes and produced these matched utility changes **relative to free peering**:

- World 1 GPT 1: **+$141.96 for us, +$35.75 for buyer**.
- World 2 Lossless Pilots: **+$81.96 for us, +$22.34 for buyer**.
- World 3 Claude 2: **+$130.14 for us, +$33.52 for buyer**.
- World 4 Claude 1: **+$141.66 for us, +$32.32 for buyer**.

The model still omits the counterparties' real agreements, programs, hidden routes, and installed crossing hardware. In the World 2 Lossless comparison, the existing GPT 2 transit price was the same in both matched cases, so it cancels from the reported delta; the absolute modeled utilities are not a live forecast. Other modeled payments can also exceed likely live receipts: World 1 GPT 1 carried **71.92 Mb** across our bilateral link in Round 2, worth only **$57.54 at $0.80/Mb** if volume stays unchanged; World 4 Claude 1 carried **72 Mb**, worth **$57.60** on the same assumption. The modeled $142 seller gain in those worlds must therefore **not** be promised as a Round 3 gain. The model shows a plausible service and negotiable price; buyer signatures and live meters decide actual profit. The four model deltas sum to about **$496**, already below the earlier $2,000 target even before accounting for acceptance and live differences.

## Cutoff sequence

1. Check whether any of the free-peer alternatives or the older Claude 2 transit file became mutually signed after the screenshots. Never overlay paid transit on an existing same-service peer simply to collect settlement.
2. Upload only the still-valid paid proposals above, then contact the buyers about the specific extra routes. The website's payment preview and exact file terms must be checked before signature under R18.
3. Sign the independent route-supplier renewals; a buyer cannot purchase routes we have not retained. Follow up existing Silent Falcons peer offers rather than duplicating them.
4. Around **8:30 a.m. Central**, resolve unsigned paid pitches and sign the already-offered free-peer fallbacks where available. By **10:00 a.m.**, verify *both signatures* and the start-round state on every essential agreement. A pending proposal does not open a route.
5. Retain the current submitted `switch.wasm`: on the sample storm, the tested failover candidates lost utility versus it. Keep Auction 2 cache decisions separate because its hardware first operates in Round 4.
