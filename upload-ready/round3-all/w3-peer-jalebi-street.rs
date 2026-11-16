//! World 3 bilateral peering renewal with JalebiStreet (AS 13).
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 3,
    parties: ["jknetwork", "JalebiStreet"],
    links: [("jknetwork", "JalebiStreet", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 3: jknetwork and JalebiStreet peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
