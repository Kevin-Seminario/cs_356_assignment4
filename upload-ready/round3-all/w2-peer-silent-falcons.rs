//! World 2 optional new free peer with Silent Falcons; verify seat and IXP before offering.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 2,
    parties: ["jknetwork", "Silent Falcons"],
    links: [("jknetwork", "Silent Falcons", "peer")],
    cap: 0,
    rounds: 1,
    terms: [],
    reads: [
        "World 2: jknetwork and Silent Falcons peer; each advertises destinations allowed by its routing policy.",
        "Neither party pays the other. This runs for one round, then renews for one unless either party declines.",
    ],
}
