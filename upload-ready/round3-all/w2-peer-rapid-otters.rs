//! World 2 free-peering renewal with Rapid Otters. No settlement payment.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 2,
    parties: ["jknetwork", "Rapid Otters"],
    links: [("jknetwork", "Rapid Otters", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 2: jknetwork and Rapid Otters peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
