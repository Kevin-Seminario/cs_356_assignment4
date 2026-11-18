//! W2 transit with one shared two-way allowance per round.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let outbound = input.sum(0, 1, |_| true).bytes;
    let inbound = input.sum(1, 0, |_| true).bytes;
    // Terms are scaled by one million, so a micro-megabit is one bit.
    let free_bytes = input.term("allowance") as u64 / 8;
    let billed_bytes = outbound.saturating_add(inbound).saturating_sub(free_bytes);
    let charge = wire::per_mb(input.term("price"), billed_bytes).min(100_000_000);
    let mut result = input.out();
    result.pay(0, 1, charge);
    result
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 2,
    parties: ["Bmobile", "jknetwork"],
    links: [("Bmobile", "jknetwork", "transit")],
    cap: 100,
    rounds: 1,
    terms: [("price", "$/Mb", 1), ("allowance", "Mb", 75)],
    reads: [
        "Bmobile buys transit from jknetwork in World 2. Routes include Rapid Otters (10.26.0.0/17 and 10.26.128.0/17) and network (10.28.0.0/17 and 10.28.128.0/17) while advertised and available.",
        "No fixed fee. The first 75 Mb each round is free across both directions combined. Bmobile pays jknetwork $1 per additional Mb, up to $100 per round.",
        "The meter includes direct traffic and transit to all destinations, including traffic that arrives late or is lost after crossing.",
        "Runs one round, then renews one round at a time unless either team declines renewal.",
    ],
}
