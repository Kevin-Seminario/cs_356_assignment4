//! World 4 free-peering renewal with Anycast Badgers. No settlement payment.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 4,
    parties: ["jknetwork", "Anycast Badgers"],
    links: [("jknetwork", "Anycast Badgers", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 4: jknetwork and Anycast Badgers peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
