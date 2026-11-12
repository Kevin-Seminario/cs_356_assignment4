# Round 2 replay event audit

The four original `.simlog` files were downloaded from the site's Round 2 results and remain in `/Users/denjaylu/Downloads/`. They are not copied into Git because they total about 15 MB. The hashes below identify the exact replay files used for this audit. Event times are simulation time, not wall-clock time. Switch and link identifiers come from each replay's topology declarations. Packet-drop counts are replay events at switches owned by our AS, not unique customer-Mb losses. Meter Mb are scaled game readings; the replay's packet bytes are one tenth of those readings.

| World | Replay SHA-256 | Physical event | Our-switch link drops | Other relevant event |
| --- | --- | --- | ---: | --- |
| 1 | `e99421d81d29f9725842b8e4428266d1c6575d9735ddbe87a67f46bed79f0876` | Link 112, our hub 231 port 1 to hub 232 port 1, failed 5.4–7.2 s | 883: 285 in second 5, 484 in second 6, 114 in second 7 | Static routes remained installed; no failure declaration or reroute was logged. |
| 2 | `5e63319a48f4974bef533d5ac905d323710667d379d2d6f82177176e3e572399` | Link 282, our switch 232 port 7 to Rapid Otters' switch 261 port 5, failed 2.0–3.0 s | 50 in second 2 | Border route continued pointing at the failed peer link. |
| 3 | `7be87a81bfb08feb1738fe64b3b9c7d61d5fc7731e5730acbf6ccb1c99b91053` | None logged | 0 | 400 TTL drops, exactly 40 each second, were protocol 91 packets with source `0x30000` and destination `10.15.0.1`; these were not customer packets. |
| 4 | `92b5af18d479c1ad5a057eebf2286ed16350df8b3bab1cac2f858a83` | None logged | 0 | No egress queue drops in the team's switch log. |

In World 1, the only route provided by RedBull beyond RedBull's own AS 18 was AS 14. Packet snapshots crossing our RedBull border show 14.56 game Mb outbound to AS 14 and 11.04 game Mb inbound from AS 14, totaling **25.60 Mb** of the **94.48 Mb** that the bilateral meter billed. The other **68.88 Mb** was direct RedBull traffic. At $1.50 per Tier-1 Mb, the most the 25.60 Mb could save in avoided Tier-1 fees is $38.40, less than the $75.584 transit payment. That is a fee-only bound, not a counterfactual utility score; a faster path can also change on-time home and company revenue. The replay and statement cannot isolate that effect without a matched rerun using free peering.

The same Round 2 switch logs confirm RedBull's World 4 free peering offered ASes 1 and 3 via RedBull's AS 6, and World 2 peer 123455 offered ASes 2 and 29. World 2 also classified GPT 2, AS 25, as our customer and recorded its $89.28 transit payment to us in the statement.
