# A4 read-only game harness

This folder sits inside the A4 kit so the world and pit-wall workspaces can reach the simulator and rules two directories above them. Read `AGENTS.md` and `common/Website-read-API.md` before using it.

The team must replace any key previously pasted into chat. Put the new key in the `NA4_KEY` environment variable in your own terminal, outside the repository. Never commit or paste the key into chat. This session cannot verify live access until that variable is available to the process running the harness.

From `pitwall/`, run `python3 ../tools/na4.py summary` for a read-only overview. From `W1/` through `W4/`, run `python3 ../tools/na4.py state --world N` for a world-specific view. The script only performs GET requests; every game action remains a person's click on the website.
