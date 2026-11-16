//! World 3 free-peering renewal with Claude 1. No settlement payment.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 3,
    parties: ["jknetwork", "Claude 1"],
    links: [("jknetwork", "Claude 1", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 3: jknetwork and Claude 1 peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
