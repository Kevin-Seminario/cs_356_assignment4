# Worlds 1, 2, and 4: proposed regional transit buyers

These are five **separate candidate offers**, with upload files and matching source
in the world subfolders. Each buyer pays jknetwork $1/Mb for traffic metered
across their bilateral link in either direction, capped at $400 per round.
The term is one round and renews for one round unless either party declines.
The cap limits payment, not traffic. There is no quality refund. The switch's
routing policy decides which destinations are announced and carried; the
agreement promises only those announced routes.

| World | Buyer | Upload file | Why consider it |
| --- | --- | --- | --- |
| 1 | GPT 2 (AS17) | `world1-conditional/gpt2.wasm` | Convert an existing free peer to paid transit only if a new-format free peer is absent or mutually cancelled and transit opens useful routes. |
| 1 | GPT 1 (AS25) | `world1-conditional/gpt1.wasm` | Same condition; local model preferred this buyer over GPT 2 by seller utility. |
| 2 | Lossless Pilots (AS20) | `world2/lossless-pilots.wasm` | No direct agreement in Round 2; new interconnection and transit reachability. |
| 2 | Silent Falcons (AS27) | `world2/silent-falcons.wasm` | No direct agreement in Round 2; new interconnection and transit reachability. |
| 4 | Silent Falcons (AS27) | `world4/silent-falcons.wasm` | No direct agreement in Round 2; new interconnection and transit reachability. |

**Before offering:** A human must check the live Agreements page for each
world. Do not stack a paid agreement on a signed new-format free peering
agreement with the same buyer. The existing old-format peers may run through
the Round 3 cutoff; verify their renewal status. In World 2 also verify that
GPT 2's existing paid transit is renewed or replaced, without duplicating it.
The World 1 files are deliberately conditional because both buyers were
already free peers in Round 2. They should not be uploaded simply to collect
payment on the same routes. Explain the new reachability to each buyer and
check the website's settlement preview before signing (R18, R23, R33).

**Local check:** All five files compile for `wasm32-unknown-unknown` and
settled without errors in matched full-ring practice runs with the Round 3
storm. Their utility matched the corresponding generic transit template
models exactly. The modeled changes in jknetwork utility, **one candidate at
a time**, were W1 GPT 2 +$172, W1 GPT 1 +$316, W2 Lossless Pilots +$408,
W2 Silent Falcons +$393, and W4 Silent Falcons +$450. These numbers are
directional, not live revenue forecasts: the model uses starter opponents,
does not represent every live agreement or route, and omits RedBull's World 4
cross-ring hardware. The modeled $400 payments hit the cap, while the real
World 2 GPT 2 transit paid only $89.28 in Round 2. Do not add these gains
as if all five simultaneous offers were tested together.
