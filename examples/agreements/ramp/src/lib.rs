//! Ramp: transit whose price falls with a good record. The buyer (party 0)
//! pays the seller (party 1) per Mb crossing between them, either way. The
//! price starts at `price` and is `step` lower for every round in a row,
//! before this one, in which at least `min_on_time` (a fraction) of that
//! traffic arrived on time, but never under `floor`. A round that misses
//! starts the count again.
//!
//! The count is the program's memory: one byte of `state`, handed back
//! every round.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let (up, down) = (input.sum(0, 1, |_| true), input.sum(1, 0, |_| true));
    let bytes = up.bytes + down.bytes;
    let on_time = (up.on_time + down.on_time) as i128;
    let met = on_time * 1_000_000 >= input.term("min_on_time") as i128 * bytes as i128;
    let good_rounds = input.state.first().copied().unwrap_or(0);
    let price = (input.term("price") - good_rounds as i64 * input.term("step")).max(input.term("floor"));
    let mut out = input.out();
    out.pay(0, 1, wire::per_mb(price, bytes));
    out.state = vec![if met { good_rounds.saturating_add(1) } else { 0 }];
    out
}

// A starter: put your own team codes, world, numbers and words in.
switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["AAA", "BBB"],
    links: [("AAA", "BBB", "transit")],
    cap: 60,
    rounds: 3,
    terms: [("price", "$/Mb", 0.40), ("step", "$/Mb", 0.05), ("floor", "$/Mb", 0.20), ("min_on_time", "fraction", 0.90)],
    reads: [
        "AAA buys transit from BBB.",
        "AAA pays BBB per Mb that crosses between them, either way.",
        "The rate starts at price, and is one step lower for every good round in a row before this one, but never under floor.",
        "A good round is one where at least min_on_time of that traffic arrives on time; a round that misses starts the count again.",
    ],
}
