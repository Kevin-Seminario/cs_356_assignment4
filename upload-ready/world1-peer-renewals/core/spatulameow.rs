//! World 1 bilateral peering with spatulameow.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["jknetwork", "spatulameow"],
    links: [("jknetwork", "spatulameow", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 1: jknetwork and spatulameow peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
