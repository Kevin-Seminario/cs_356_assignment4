//! World 3 bilateral peering renewal with networkarena (AS 20).
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 3,
    parties: ["jknetwork", "networkarena"],
    links: [("jknetwork", "networkarena", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 3: jknetwork and networkarena peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
