//! World 2 transit candidate: Silent Falcons (AS 27) buys from jknetwork (AS 23).
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
    world: 2,
    parties: ["Silent Falcons", "jknetwork"],
    links: [("Silent Falcons", "jknetwork", "transit")],
    cap: 400,
    rounds: 1,
    terms: [("price", "$/Mb", 1.00)],
    reads: [
        "World 2: Silent Falcons buys transit from jknetwork for one round.",
        "jknetwork carries traffic for destinations it announces to Silent Falcons; the routing policy selects those destinations.",
        "Silent Falcons pays $1 per Mb crossing the link in either direction, up to $400 per round.",
        "The cap limits payment, not traffic. The agreement renews for one round unless either party declines.",
    ],
}
