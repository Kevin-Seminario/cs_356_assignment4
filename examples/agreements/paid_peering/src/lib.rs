//! Paid peering: if one side sends more than `ratio` times what it
//! receives, it pays `price` per Mb for the excess.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let a_sends = input.sum(0, 1, |_| true).bytes as i128;
    let b_sends = input.sum(1, 0, |_| true).bytes as i128;
    let ratio = input.term("ratio") as i128;
    // Excess over ratio x received; the ratio is in millionths.
    let excess = |sent: i128, got: i128| ((sent * 1_000_000 - ratio * got) / 1_000_000).max(0) as u64;
    let mut out = input.out();
    out.pay(0, 1, wire::per_mb(input.term("price"), excess(a_sends, b_sends)));
    out.pay(1, 0, wire::per_mb(input.term("price"), excess(b_sends, a_sends)));
    out
}

// A starter: put your own team codes, world, numbers and words in.
switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["AAA", "BBB"],
    links: [("AAA", "BBB", "peer")],
    cap: 60,
    rounds: 2,
    terms: [("ratio", "times", 2.0), ("price", "$/Mb", 0.30)],
    reads: [
        "AAA and BBB peer: each carries traffic to the other's customers.",
        "A side may send up to ratio times what it receives, unpaid.",
        "A side that sends more pays the other the price for every Mb over that.",
    ],
}
