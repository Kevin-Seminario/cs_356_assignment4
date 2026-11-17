//! World 3 discounted transit pilot: the buyer pays jknetwork for bilateral
//! transit traffic. Route export is controlled by the switch program, not
//! by this settlement program; both must be reviewed together.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let outbound = input.sum(0, 1, |_| true);
    let inbound = input.sum(1, 0, |_| true);
    let owed = wire::per_mb(input.term("price"), outbound.bytes + inbound.bytes);
    let mut result = input.out();
    result.pay(0, 1, owed);
    result
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 3,
    parties: ["Claude 2", "jknetwork"],
    links: [("Claude 2", "jknetwork", "transit")],
    cap: 200,
    rounds: 1,
    terms: [("price", "$/Mb", 0.80)],
    reads: [
        "World 3: the buyer purchases regional transit from jknetwork for one round.",
        "jknetwork advertises its own, customer-learned and peer-learned routes to the buyer, but not provider-learned or backup-learned routes.",
        "The buyer pays $0.80 per Mb crossing the link in either direction, subject to the $200 per-round payment cap.",
        "The cap limits payment, not traffic; both parties can decline renewal after the first round.",
    ],
}
