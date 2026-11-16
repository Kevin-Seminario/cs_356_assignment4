//! World 1 transit candidate: GPT 2 (AS 17) buys from jknetwork (AS 23).
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let buyer_to_seller = input.sum(0, 1, |_| true);
    let seller_to_buyer = input.sum(1, 0, |_| true);
    let owed = wire::per_mb(input.term("price"), buyer_to_seller.bytes + seller_to_buyer.bytes);
    let mut result = input.out();
    result.pay(0, 1, owed);
    result
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["GPT 2", "jknetwork"],
    links: [("GPT 2", "jknetwork", "transit")],
    cap: 400,
    rounds: 1,
    terms: [("price", "$/Mb", 1.00)],
    reads: [
        "World 1: GPT 2 buys transit from jknetwork for one round.",
        "jknetwork carries traffic for destinations it announces to GPT 2; the routing policy selects those destinations.",
        "GPT 2 pays $1 per Mb crossing the link in either direction, up to $400 per round.",
        "The cap limits payment, not traffic. The agreement renews for one round unless either party declines.",
    ],
}
