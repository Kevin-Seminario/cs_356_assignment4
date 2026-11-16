# Round 3 handoff: protect routes, prepare a Web/Video cache service

**For the team. Updated November 16, 2026.** Round 3 cuts off **Tuesday, November 17 at 10:00 a.m. Central**. Auction 2 closes **Wednesday, November 18 at 8:00 p.m. Central**. Hardware won there can first operate in **Round 4, Thursday, November 19**.

## Our strategy

We want to deliver Web and Video to home customers without paying Tier-1 for every Mb. A cache serves 40% of eligible *external* Web/Video locally; a shared cache at one IXP can serve the homes of up to four named teams in a joint bid. Direct peer routes address a different problem: traffic between ASes. Neither peers nor switch policy can replace Tier-1 for the external content specified in the rules.

This is a **candidate business**, not a proven $2,000 gain. In World 3 Round 2, our Tier-1 bill was $1,547.64 on 1,031.76 Mb. The replay suggests roughly 300 Mb was external content. At similar traffic, 40% caching would avoid about 120 Mb, or **$180 of Tier-1 fees per team per round**, before any on-time-delivery change. A shared cache may help other teams, but we must establish their eligibility, IXP location, capacity, willingness to share credits, and payment terms. Eight private caches and two shared caches are auctioned per world; competing teams may prefer their own cache. Credits and utility dollars are different currencies.

The earlier **paid regional-transit idea is not our main plan**. Our only measured live transit sale was $89.28 from GPT 2 in World 2. The $400 World 3 Claude 2 result came from a practice model that hit its cap; Claude 2 has not signed our proposed sale. Treat that pending offer as an experiment, not forecast revenue. World 4's useful far-side routes came from free RedBull peering, with no settlement payment.

## What to do before the Round 3 cutoff

1. **Save the routes we already rely on.** Old-form agreements end when their terms end and do not automatically renew into the new file system. On the World 3 Agreements page, several peers were marked “last ran R2 — ends at the Round 3 cutoff,” including JalebiStreet and Network Champions. Audit **all four worlds**, identify each valuable agreement ending at this cutoff, and get a replacement file signed by both parties. A pending offer does not open a route. Check the site's final signed status, not just files in Git.
2. **Choose one Claude 2 relationship in World 3.** Claude 2's free-peer file and our paid-transit file were both pending when inspected on November 16. Compare the *additional destinations* Claude 2 gains as a customer with the actual expected payment and capacity cost. Do not sign overlapping deals merely to charge traffic already available through a free peer. Human team members decide whether to sign, decline, or withdraw.
3. **Test the live switch against the new failure pattern.** Round 3 has longer and more frequent pass, IXP-port, and crossing failures. Our submitted switch installs static forwarding entries; previous failover candidates helped some practice cases but harmed others. Use the updated kit's `storm.toml` to compare the currently submitted program with any proposed replacement, on the same traffic and agreement set. Review utility, Tier-1 fee, Web/Video and Calls on-time Mb, and program failures. Upload a replacement only if evidence supports it, then confirm the website marks the intended version live and it passes the required dry run.
4. **Verify the route inventory before cutoff.** Confirm which agreements are signed for Round 3 and which selected routes those relationships should provide. Missing peers can shift Web/Video back to Tier-1 even if the switch is unchanged. The website cutoff, not a Git commit, determines what runs.

## Cache work for Auction 2 and Round 4

- Pick a pilot world and an IXP shared by us and prospective partners. World 3 has the highest observed Tier-1 bill and is a reasonable first analysis target; it is not automatically the best bid.
- For each candidate partner, estimate eligible external Web/Video, 40% offload, avoided Tier-1 fees, on-time change, and shared-IXP link load. Compare a shared cache with that team's private-cache alternative.
- Agree on owner, installation IXP, credit shares, and any recurring payments **before** asking all parties to sign a joint bid. The winner owns a fixed-location asset and the joint-bid agreement lasts to season end. Do not infer that a cache serves teams outside the named joint bid or outside its IXP.
- A person makes every website click, upload, signature, bid, and installation. Auction 2's winning hardware cannot improve Round 3.

## Evidence and cautions

- Round 2 results: `docs/report/evidence/round-2/statement-w{1,2,3,4}-r2.json`, corresponding border readings and switch logs.
- Prior controlled experiments: `docs/report/evidence/round-2/agreement-opportunity-sweep.md`, `docs/report/evidence/round-2/selective-internal-failover-test.md`, and `docs/strategy/regional-bridge-pilot.md`. Their practice gains are comparisons, not live forecasts.
- Rules and new mechanics: `docs/assignment/a4-rule-book.md`, `docs/assignment/programmable-agreements-agent-brief.md`, and the downloaded `a4-round3-update.tar.gz` / `Round-3-update-agent-pickup.md`. The repository's `rules.json` and simulator have **not yet been updated** from that archive at the time of writing.

**Decision rule:** preserve proven reachability first; change the switch or sign a paid deal only when matched evidence shows net utility after delivery, Tier-1 charges, settlements, and congestion. Build the cache partnership for Round 4 on measured buyer economics, not a headline utility target.
