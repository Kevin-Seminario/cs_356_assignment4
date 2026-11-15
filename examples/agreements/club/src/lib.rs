//! Club: two to four ASes peer with one another and share the cost by
//! use. Each member pays `price` per Mb it sent to the other members into
//! a pot, and the pot is split equally among all of them: a member that
//! sends more than the average pays, one that sends less is paid.
use switch_program_sdk::wire::{self, SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    let n = input.parties.len();
    let mut out = input.out();
    for i in 0..n {
        let sent: u64 = (0..n).filter(|&j| j != i).map(|j| input.sum(i, j, |_| true).bytes).sum();
        // Member i's share of the pot goes to each other member (it keeps its own).
        let share = wire::per_mb(input.term("price"), sent) / n as i64;
        for j in (0..n).filter(|&j| j != i) {
            out.pay(i, j, share);
        }
    }
    out
}

// A starter: put your own team codes, world, numbers and words in.
switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["AAA", "BBB", "CCC"],
    links: [("AAA", "BBB", "peer"), ("AAA", "CCC", "peer"), ("BBB", "CCC", "peer")],
    cap: 60,
    rounds: 2,
    terms: [("price", "$/Mb", 0.30)],
    reads: [
        "AAA, BBB and CCC all peer with one another.",
        "Every Mb a member sends to the others costs price, paid into a pot.",
        "The pot is split equally among the members.",
    ],
}
