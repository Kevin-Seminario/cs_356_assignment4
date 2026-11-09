# A4 rule book — Section 12

Source: `a4-handout-2026-11-07.pdf`, pages 10–12. This is the text of Section 12; consult the PDF if a symbol or line break matters. Numbers are keyed to the root `rules.json`.

This section is the A4 rule book: the whole set of rules. It is written for you and for your AI agent. Give it to your agent as standing context: save the text of this section, with the hardware handout and rules.json, where your agent’s instructions file (for example AGENTS.md or CLAUDE.md) points, so it is loaded at the start of every session. Each rule stands alone; terms are defined in R1; every number is named by its key in rules.json, written as key.

## Terms

R1. Definitions. AS: one team’s network in one world. World: one copy of the game map (world.worlds of them). IXP: an exchange point; world.ixps form a ring. Sector: the world.ases_per_sector ASes between two neighbouring IXPs. Hub: your switch in each of your sector’s two towns, with your IXP port and your Tier-1 link. Crossing: your link between your two hubs; it runs through one of the sector’s two passes, shared with one other AS (your pass-mate). Seat: an AS’s place on the map. Round: one run of all four worlds at a cutoff. Utility: your dollars in one world and round (R8). Agreement: a signed settlement program between two ASes. Usual rule: the default routing policy (R22). Ledger: the public record of every agreement, payment, bid and hardware sale. Mb: megabit.

## The world

R2. Seats. Each world has world.ases_per_world ASes: the teams of this class and world.ai_seats ASes played by AI, marked AI on the map. AI seats deal only through agreement offers (no chat) and act after each round’s results and 8:00–9:00 CT each morning; an AI seat that cannot act keeps its last settings. A seat with no team is run by us with our starter; a team that stops playing keeps its last settings, and its agreements run to the end of their term. Every seat is equivalent. The worlds are fixed for the game; you share a sector with any other team in at most one world.

R3. Neighbours. You can interconnect with the world.possible_neighbours other ASes at your two IXPs: 3 in your sector and 4 in each neighbouring sector. Traffic between two ASes at an IXP flows only while they have an active agreement. Only links sold at auction cross the middle of the ring; one lets you interconnect with the ASes at the far IXP too.

R4. The Tier-1 has a link to each of your hubs and reaches every destination, including external content nothing else reaches. It never fails; its capacity is fixed and cannot be bought. We announce your own address blocks to it on both links, each as nearer the link it is closer to; you do not set this.

R5. Your network is the same for every AS: two hubs, two access switches under each (your homes), a hosting site in the west town (the companies) and your crossing. Capacities and latencies: network; buffers: buffer_bytes_per_port.

R6. What you can see. The website shows every world’s map and the hardware on it, never live traffic or other ASes’ routes. You see what your own switches measure and log, your statements, the meter readings on your own border links (every meter_epoch_s), and your round’s replay: your own AS in full, other ASes only as what crossed your borders.

## Money and score

R7. Two kinds of money never mix: dollars are your score; credits (cr) buy hardware.

R8. Utility in each world and round = value + settlements received − settlements paid − Tier-1 fee. Value is what your homes and companies pay you (R9, R10).

R9. Homes pay classes[].home_price_per_mb for each Mb, sent or received, that arrives within its class’s classes[].target_ms; late or lost traffic pays nothing. When your home talks to a company through another AS, your home pays you.

R10. Companies each pay classes[].company_contract_per_sector per round, split among the sector’s four ASes. If q_A is the fraction of the company’s traffic through AS A that arrived within the target (both directions), A earns contract × q_A²/(q_A² + q_B² + q_C² + q_D²); the whole contract is always paid. Example: B, C, D deliver 95%, A 60%: A earns 0.36/3.07 ≈ 12%, the others about 29% each. You may slow other ASes’ company traffic into your homes: it lowers their q, costs you what those Mb would have paid your homes, and shows on their meter readings.

R11. The Tier-1 fee is tier1_fee_per_mb for every Mb on your Tier-1 links, in each direction.

R12. Score. In each world and round, score = 100×(utility−floor)/(par−floor), where floor is the run in which every AS runs our reference program and uses only the Tier-1, and par the run in which every AS runs our reference program with our reference agreements, on the same world, traffic and failures, then clamped to scoring.score_clamp. Your season score is the sum over the four worlds and all scoring.rounds_counted rounds.

R13. Grade: season score 75%, graded from the leaderboard after the last round with a scoring function announced on Ed; final report 25% (scoring.grade_weights).

R14. Credits are separate in each world: credits.at_open when the game opens and credits.per_monday every Monday after that, spent in auctions and on hardware bought from other teams; unspent credits carry over up to credits.carry_max (credits held in bids do not count); they are worth nothing at the end.

## Agreements

R15. Only agreements bind. Chat is logged and binds no one. Chat and pseudonyms follow the course conduct policy; we rename an offensive pseudonym.

R16. A settlement program runs after each round on the readings for the links between the two ASes (per direction, class and meter_epoch_s: bytes crossed; how many arrived on time end to end, late or lost; and how many came straight back into the AS they left, which is one AS carrying the other’s traffic between its own towns) and the round’s failure log. It returns one amount and who pays it, never more than the maximum both teams signed. It can do nothing else; limits: settlement_program. If it traps or runs out of fuel, it pays nothing that round and the ledger records it.

R17. Templates. Transit: the customer pays the provider a price per Mb crossing between them, in either direction. Peering: no payment. Paid peering: if one side sends more than an agreed multiple of what it receives, it pays a price per Mb for the excess (what it sent minus that multiple of what it received). Backup transit: a fixed fee each round plus a price per Mb of the buyer’s traffic carried between the buyer’s towns. Transit and backup may add a refund, off unless set: an amount paid back if the partner does not carry the traffic when promised, or delivers too little of it on time.

R18. Your own settlement program may replace a template for agreements starting at the Round 2 cutoff or later. It is run on test readings when uploaded; one that traps or runs out of fuel is rejected. Before you sign one, the website shows what it would have paid on test readings and, if your two ASes have been connected, on your readings from recent rounds; its wasm can be downloaded and run in the kit. Do not sign a program whose payments you have not checked.

R19. Signing. One team offers (a filled-in template or its own program; an offer has no free text) and names the world and the rounds it covers, at most agreements.max_term_rounds; the other signs. An agreement is identified by the hash of its program and its numbers, so what both sign is exactly what runs. It starts at the next cutoff and runs for the rounds named. It ends early only if both sign a cancellation, or after a rule change (R31). At the end of its term it renews for the same number of rounds unless either side declines before the cutoff the new term would start at, so after the last round’s results. An offer can go to the same seat relative to you in every world; each copy is signed separately. Two teams may hold more than one agreement in a world. Each agreement has a label (provider, customer, peer, backup) that your programs see.

R20. Metering. At an IXP, the exchange (run by us) counts the traffic between each pair of members; every other link that leaves your AS carries a meter run by us. Settlements and customer payments use only our measurements. Only customer traffic is metered: packets your programs send (hellos, probes) use capacity, and the Tier-1 fee counts them like any Mb on your Tier-1 links, but they never appear on a meter or a settlement; each switch may send at most switch_program.inject_bytes_per_s of them a second.

## Your programs and routing

R21. Your programs. switch.wasm runs on every switch of your AS (exit, queueing, failover; limits: switch_program) and holds your routing policy (R22; limits: policy_program). Each agreement is a settlement program. All are wasm; the API guide has the interfaces.

R22. Routing policy. For each destination, your switch program’s policy chooses which offered route you use (rank) and whether you tell each neighbour about it (announce). You are offered only routes your agreements cover. If your program leaves the policy out, or it traps, runs out of fuel or answers out of range, the usual rule applies: rank customer over peer over provider, with the Tier-1 last, then the shortest AS path; announce your own and your customers’ routes to everyone, and your peers’ and providers’ routes only to your customers. Every policy call runs on a fresh copy of your program: it keeps nothing from one call to the next and shares nothing with your running switches, so its answers depend only on its input.

R23. Announcing is a promise to carry the traffic. Announcing more than the usual rule is a leak, and you carry that traffic unpaid. Sending a neighbour traffic for a destination it did not announce to you is a sham deal (R33); your switch may drop such traffic.

R24. Settling. Routes are recomputed until no AS changes, at most policy_program.max_iterations times; if they have not settled, the last iteration is used and your statement says so.

R25. Backups are handled by us: the seller’s route between your towns is offered to your switch program for failover; policies do not rank or announce it.

## Rounds

R26. Schedule. The game opens at schedule.game_opens; round cutoffs are schedule.round_cutoffs, auctions close at schedule.auctions_close, all Central Time (the schedule in Section 11 lists them). Results are out by 18:00 on the day of the cutoff (Round 1: by midnight). Every round counts, except one we cancel because our side failed (website guide).

R27. The cutoff. What you have set at a cutoff (programs, installed hardware, signed agreements) is what runs; nothing changes during a round. If the website is down in the 2 hours before a cutoff, we move the cutoff and announce the new time on Ed.

R28. Submissions. Every switch program must pass a dry run on the practice world before it is accepted; resubmitting is free and has no limit, and each team’s dry runs wait in line, one at a time. A program larger than its block’s max_wasm_bytes, or one that imports anything the SDK does not provide, is rejected. If your switch program fails during a round, that world is rerun with your last version that passed; if you never submitted one, with our starter.

R29. Traffic and failures. Each round runs on traffic you have not seen, the same in every repeat of the round. Single links, IXP ports and whole passes fail (the hardware handout says which hardware fails with them), on one schedule in which every sector and every pass is hit equally often over the season. Your programs and settings are never touched. Before the game opens we publish a hash of the season’s traffic and failure schedule, and at each cutoff a hash of what everyone has set; after the last round we publish both, so anyone can check them.

R30. Results. For each world: your utility and score, a statement for each agreement and each customer (the measurements it was paid on), your logs and the replay. The leaderboard shows season scores and your place in each world. Report a suspected game bug before the second cutoff after that round’s results; if a confirmed bug changed scores, we rescore that round for everyone.

R31. Changes. Numbers in rules.json may change until the game opens. After that they change only to close an exploit, fix a broken rule or fix a number that makes the game unplayable. A change is announced on Ed and takes effect at the first cutoff at least 48 hours later; any agreement it affects may then be cancelled by either side.

## Hardware and conduct

R32. Hardware is sold only in the auctions, for credits. Bids are open: everyone sees the winning bids, and every bid goes on the ledger as it is made. Teams may bid jointly, and may trade hardware for credits at any price both agree on, never for dollars or through an agreement. The hardware handout has the auction rules.

R33. Cooperation is allowed; sham deals are not. Deals that only move money or credits between teams can be reversed. Each round we flag agreements whose payments are far from the value of the traffic they metered, review them, and tell both teams our decision privately with reasons; each may appeal once on the course discussion board.

R34. AI is welcome: it may write your programs and help you plan. There is no game API; do not script the website, in code or with an agent driving a browser: a person makes every click. Do not share your switch programs with other teams, or reach the servers or another team’s AS except through the game.
