//! World 2 renewal of GPT 2 buying transit from jknetwork at the Round 2 price.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let buyer_to_seller = input.sum(0, 1, |_| true);
    let seller_to_buyer = input.sum(1, 0, |_| true);
    let bytes = buyer_to_seller.bytes + seller_to_buyer.bytes;
    let mut out = input.out();
    out.pay(0, 1, wire::per_mb(input.term("price"), bytes));
    out
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 2,
    parties: ["GPT 2", "jknetwork"],
    links: [("GPT 2", "jknetwork", "transit")],
    cap: 400,
    rounds: 1,
    terms: [("price", "$/Mb", 1.00)],
    reads: [
        "World 2: GPT 2 buys transit from jknetwork for one round.",
        "jknetwork carries traffic for the destinations it advertises to GPT 2.",
        "GPT 2 pays $1 per Mb crossing the link in either direction, at most $400 per round.",
        "The cap limits payment, not traffic. The deal renews for one round unless either party declines.",
    ],
}
