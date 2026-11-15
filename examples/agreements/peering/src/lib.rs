//! Peering: no payment. Each side carries traffic to the other's customers.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

// A starter: put your own team codes, world, numbers and words in.
switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["AAA", "BBB"],
    links: [("AAA", "BBB", "peer")],
    cap: 0,
    rounds: 2,
    terms: [],
    reads: ["AAA and BBB peer: each carries traffic to the other's customers.", "Nobody pays."],
}
