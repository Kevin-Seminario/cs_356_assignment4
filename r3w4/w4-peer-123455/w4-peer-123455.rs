//! World 4 free-peering renewal with 123455. No settlement payment.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 4,
    parties: ["jknetwork", "123455"],
    links: [("jknetwork", "123455", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 4: jknetwork and 123455 peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
