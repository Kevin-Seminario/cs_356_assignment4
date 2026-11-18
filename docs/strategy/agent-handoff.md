# jknetwork handoff for Round 4 and Auction 2

Start with the root AGENTS.md for rules. Round 3 has finished. The earlier Round 3 execution and decision notes describe experiments and pre-cutoff assumptions. The website's read API gives the current signatures, renewals, bids and statements.

## First session

From the repo root, run `python3 harness/tools/na4.py summary` with NA4_KEY in the environment. On the team's existing Mac, source ~/.na4.env in the same shell first. On a new machine the team supplies a read-only key locally. Keep credentials out of Git. The team makes all game-site clicks.

Auction 2 is scheduled to close Wednesday November 18 at 8 p.m. Central. Round 4 cuts off Thursday November 19 at 10 p.m. Central. Check both clocks when resuming.

## Where we left off

Read-only API, November 18 at 9:50 a.m. Central. Round 3 scores were W1 73.7, W2 89.6, W3 46.2 and W4 86.8. Free auction credits were 179, 200, 200 and 197 respectively. W1 also had one credit held.

- W1 private content cache #868, jknetwork 1 credit and Rapid Otters 20, was bidding. Shared cache #867 at IXP5 still needed Jumbo Comets, dinnyrob19 and spatulameow.
- W2 shared cache #871 at IXP5 still needed Bmobile and Lucky Comets. 123455 had signed.
- W3 shared cache #850 at IXP4 still needed JalebiStreet and Lucky Comets. Stray Otters had signed.
- W4 shared cache #872 at IXP5 still needed Toyotathon and Rapid Otters. 123455 had signed. A separate incoming GPT 1 offer #955 awaited our signature. Read its terms before recommending action.
- The four shared-cache proposals each split 52 auction credits equally among four teams. An incomplete signature set leaves the proposal open. Check the minimum bid again after all parties sign.

## Strategy and next decisions

The team is pursuing on-time web/video for home customers and testing whether caches improve utility by avoiding external traffic through Tier 1. Compare a world with and without the item under the same agreements, traffic and failure schedule. Separate auction credits from round utility. Test shared-cache host load and the overlap with a private cache before adding their estimated gains.

The team also tested selling transit where a customer wanted specific additional destinations. In W1 Jumbo Comets asked about reaching AS22 (Packet Lanterns). For each candidate compare against free peering, identify the newly reachable prefixes and check whether the contract charges for existing direct traffic. In W4, RedBull's microwave reaches our side. The last pre-R3 check showed free peer #819 signed and transit #642 withdrawn. Refresh before quoting any offer. The team declined GPT 1 backup transit in W4. Assess any new offer on its own terms.

First pull the R3 statement and readings for each world using `python3 harness/tools/na4.py statement 3 1` and `python3 harness/tools/na4.py readings 3 1` (change the final number for W2–W4). Compare Tier 1 costs, on-time service and actual settlements against R2. Investigate W3's lower score before attributing it to hardware. Use `state --world N` for individual agreement terms and renewals and `auctions N` for current lot details.

## Files

- r3w1 through r3w4 contain agreement source, compiled uploads, and any accompanying messages. They include drafts, fallbacks and superseded versions. Match the live agreement ID, parties and terms before reusing one.
- switch.wasm is the repository's switch artifact. Verify the live program version before running comparisons.
- docs/report/final-report-draft.md and docs/report/evidence/ hold the report and supporting work. The round-3 cross-ring transit pilot is a model, not the official R3 statement.
- docs/assignment/Round-3-update-agent-pickup.md and a4-round3-update.tar.gz (same directory) contain the official updated kit, storm scenario and hardware test instructions. Extract into a separate kit copy for testing to avoid overwriting team changes.
