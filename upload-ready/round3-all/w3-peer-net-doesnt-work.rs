//! World 3 optional new free peer with Net_doesnt_work; verify seat and IXP before offering.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 3,
    parties: ["jknetwork", "Net_doesnt_work"],
    links: [("jknetwork", "Net_doesnt_work", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 3: jknetwork and Net_doesnt_work peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
