//! Joint bid for a content cache on jknetwork's World 1 access switch.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 1,
    parties: ["jknetwork", "Rapid Otters"],
    links: [],
    cap: 0,
    rounds: 1,
    terms: [
        ("bid", "cdn_cache", 21),
        ("share_1", "cr", 1),
        ("share_2", "cr", 20),
    ],
    reads: [
        "The teams jointly bid 21 World 1 credits for one content cache. jknetwork contributes 1 credit and Rapid Otters contributes 20 credits.",
        "If we win, jknetwork owns and installs the cache on its own access switch. It serves eligible Web and Video for jknetwork homes only.",
        "Neither team owes the other any settlement payment. Both signatures are required to enter the bid.",
        "A winning agreement lasts through the end of the season.",
    ],
}
