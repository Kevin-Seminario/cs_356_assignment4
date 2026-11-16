# Worlds 1, 2, and 4: proposed regional transit buyers

**Strategy review, November 16:** These compiled files are experiment
artifacts, not a recommendation to send every offer. In particular, hold the
two World 1 AI transit files. World 1 was our strongest live Round 2 world
(utility $2,157.60; score about 82), while the modeled transit gains rely on
each buyer paying the $400 cap. Their actual Round 2 bilateral traffic would
have paid only about $143 combined at the proposed rate. No World 1
agreement currently renews, so restoring worthwhile existing routes is a
more urgent and better-supported task than converting its two AI peers.
Worlds 2 and 4 candidates also need live signed-status and bilateral-volume
checks before use. The regional bridge remains an unproven World 3 pilot,
not a four-world rollout or a demonstrated $2,000 utility opportunity.

These are five **separate candidate offers**, with upload files and matching source
in the world subfolders. Each buyer pays jknetwork $1/Mb for traffic metered
across their bilateral link in either direction, capped at $400 per round.
The term is one round and renews for one round unless either party declines.
The cap limits payment, not traffic. There is no quality refund. The switch's
routing policy decides which destinations are announced and carried; the
agreement promises only those announced routes.

The bridge plan does **not** call for renewing every old free peer. A peer
should stay only when its route supplies a useful destination to a distinct
paying buyer, or when the direct path creates enough other incremental value.
The World 1 peer files are therefore a candidate inventory, not a renewal
checklist. Test a proposed buyer together with the suppliers it needs; gains
from separate one-buyer tests cannot simply be added.

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

**World 1 recheck after the renewal clarification:** None of the old World 1
agreements will renew automatically. In a matched storm model with the old
RedBull transit removed and all eight non-AI peer suppliers present, the
seller's utility was $2,912 with both AI seats as free peers, $3,232 with only
GPT 2 as a transit customer, $3,249 with only GPT 1 as a transit customer,
and $3,496 with both as customers. These comparisons assume $400 from each
customer. The modeled non-payment effect was therefore **-$80**, **-$63**,
and **-$217**, respectively. The actual Round 2 bilateral traffic was just
71.28 Mb with GPT 2 and 71.92 Mb with GPT 1; at $1/Mb, that traffic alone
would pay about $143 combined, below the modeled $217 combined break-even.
Future route exports could increase metered traffic, and the model is not a
live forecast, but this is insufficient evidence to pitch both AI conversions
as a large World 1 gain. Prioritize renewing useful route suppliers and
negotiate a buyer only if it gains specific new reachability and realistic
volume supports the price.
Repeating the baseline, GPT 1, and both-buyer trials with the submitted
`switch.wasm` for jknetwork (instead of the kit starter) produced utilities
of $2,929, $3,266, and $3,513. The incremental buyer economics were almost
identical. The submitted policy does announce peer routes to transit
customers; the remaining uncertainty is demand, signatures, and live route
choices, not whether that export rule exists.

In the same both-buyers/no-RedBull model, removing Jumbo Comets, Packet
Lanterns, or dinnyrob19 one at a time reduced our utility by about $506,
$525, or $498. Removing any one of the other five candidate suppliers reduced
it by about $144-$159. These are marginal **one-at-a-time** model effects,
not additive forecasts, and do not prove that all eight counterparties will
sign new files. They show that selling transit can require free peers as
route inventory, rather than treating every free peer as wasted revenue.
