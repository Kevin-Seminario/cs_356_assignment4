# Round 3 paid-transit offers — review before upload

These four compiled files are **offers**, not signed agreements. A person must upload each file and check the website preview; the other party must sign before the Tuesday, November 17, 10:00 a.m. Central cutoff. Every file opens a one-round transit relationship in which the named buyer pays jknetwork for **all bilateral Mb in either direction**, subject to a $200 round cap; it renews for one round unless declined. Source files are beside the `.wasm` files, and `manifest.json` records SHA-256 hashes. Builds used the installed `wasm32-unknown-unknown` target and compiled offline. The actual compiled agreements produced the same matched-simulation utility as generic transit with the same price/cap. None has been uploaded by this agent.

## The four candidates

- **World 1, GPT 1:** `w1-gpt1.wasm`, $0.80/Mb. GPT 1's free-peer offer **48** is already signed by GPT 1 and is our fallback. Do not sign 48 while pursuing transit. If a paid file is mutually signed, leave 48 unsigned. If no paid signature arrives by the pre-cutoff decision time, sign 48. If 48 was already signed, do **not** stack this transit file on it for Round 3.
- **World 2, Lossless Pilots:** `w2-lossless.wasm`, $0.60/Mb. This was a missing direct route in Round 2. It has no pre-signed free-peer fallback in the November 16 screenshot. Offer paid transit only if the buyer can use our peer-learned routes; otherwise pursue the prepared free-peer file in `../round3-all/w2-peer-lossless-pilots.wasm`. The existing World 2 GPT 2 paid sale is a separate proven relationship and must be verified signed independently.
- **World 3, Claude 2:** `w3-claude2.wasm`, $0.80/Mb. The earlier $1/Mb, $400-cap offer **234** is still pending in the November 16 screenshot. **Check if it has been signed before acting.** If not, withdraw it before offering this discounted replacement so only one transit proposal is live. Claude 2's free-peer offer **167** lasts R3–R5 and is our fallback; do not sign it while pursuing transit. Retain real route suppliers, including Lucky Comets and Net_doesnt_work, so the customer obtains useful reachability.
- **World 4, Claude 1:** `w4-claude1.wasm`, $0.80/Mb. Claude 1's free-peer offer **147** is already signed by Claude 1 and is our fallback. Do not sign 147 while pursuing transit. If no paid signature arrives by the pre-cutoff decision time, sign 147. If 147 was already signed, do **not** stack this transit file on it for Round 3. Keep the separate Silent Falcons free-peer offer pending; it is not this buyer.

## Why these prices

We reran matched free-peer and paid-transit comparisons with our current switch, the published sample Round 3 storm, and **buyers given direct peers to all other adjacent ASes**. That is a stricter buyer-side test than the earlier isolated-buyer model. At the prices above, modeled utility changes **versus free peering** were:

- World 1 GPT 1: jknetwork **+$141.96**, buyer **+$35.75**.
- World 2 Lossless Pilots: jknetwork **+$81.96**, buyer **+$22.34**.
- World 3 Claude 2: jknetwork **+$130.14**, buyer **+$33.52**.
- World 4 Claude 1: jknetwork **+$141.66**, buyer **+$32.32**.

These are *model deltas*, not live forecasts. We do not know the buyers' actual agreements or route selection, and the model omits installed cross-ring hardware. All other teams use the kit starter. The model's buyers gain by replacing some Tier-1 traffic with peer-learned routes through us; if their live routes already cover those destinations, a transit offer may not benefit them. We should inspect the site payment preview and get a buyer-side route rationale before signing. The four model deltas cannot safely be added into a Round 3 score prediction.

**Fallback timing:** decide the unsigned paid offers by about **8:30 a.m. Central Tuesday** so the pre-signed World 1, World 3, and World 4 free-peer fallbacks can be signed before 10:00 a.m. World 2 has no such safe fallback; contact Lossless Pilots early and avoid losing a useful direct route solely to wait on an unanswered paid offer.
