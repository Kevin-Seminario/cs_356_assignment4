//! Transit: the buyer (party 0) pays the seller (party 1) `price` per Mb
//! crossing between them, in either direction.
//!
//! Refund, off unless set: if less than `min_on_time` (a fraction) of that
//! traffic arrives on time end to end, the seller pays `refund` back.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let (up, down) = (input.sum(0, 1, |_| true), input.sum(1, 0, |_| true));
    let bytes = up.bytes + down.bytes;
    let mut owed = wire::per_mb(input.term("price"), bytes);
    let on_time = (up.on_time + down.on_time) as i128;
    if bytes > 0 && on_time * 1_000_000 < input.term("min_on_time") as i128 * bytes as i128 {
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
    links: [("AAA", "BBB", "transit")],
    cap: 60,
    rounds: 2,
    terms: [("price", "$/Mb", 0.30), ("min_on_time", "fraction", 0.90), ("refund", "$", 10.00)],
    reads: [
        "AAA buys transit from BBB.",
        "AAA pays BBB the price for every Mb that crosses between them, either way.",
        "In a round where less than min_on_time of that traffic arrives on time, BBB pays the refund back.",
    ],
}
