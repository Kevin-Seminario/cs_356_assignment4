//! Backup transit: the buyer (party 0) pays the seller (party 1) `fee`
//! each round, plus `price` per Mb of the buyer's own traffic the seller
//! carried between the buyer's two towns (traffic that crossed to the
//! seller and came straight back).
//!
//! Refund, off unless set: if less than `min_on_time` (a fraction) of that
//! traffic arrives on time end to end, the seller pays `refund` back.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let carried = input.sum(0, 1, |_| true);
    let mut owed = input.term("fee") + wire::per_mb(input.term("price"), carried.returned);
    let (n, ok) = (carried.returned as i128, carried.returned_on_time as i128);
    if n > 0 && ok * 1_000_000 < input.term("min_on_time") as i128 * n {
        owed -= input.term("refund");
    }
    let mut out = input.out();
    out.pay(0, 1, owed);
    out
}

// A starter: put your own team codes, world, numbers and words in.
switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["AAA", "BBB"],
    links: [("AAA", "BBB", "backup")],
    cap: 60,
    rounds: 2,
    terms: [("fee", "$", 5.00), ("price", "$/Mb", 0.50), ("min_on_time", "fraction", 0.90), ("refund", "$", 10.00)],
    reads: [
        "AAA buys backup transit from BBB.",
        "AAA pays BBB the fee every round, used or not.",
        "AAA also pays the price for every Mb of its own traffic that BBB carries between AAA's two towns.",
        "In a round where less than min_on_time of that traffic arrives on time, BBB pays the refund back.",
    ],
}
