//! World 3 joint bid for one shared cache at IXP 4.
use switch_program_sdk::wire::{SettleInput2W, SettleOutputW};

fn settle(input: &SettleInput2W) -> SettleOutputW {
    input.out()
}

switch_program_sdk::agreement_program! {
    settle: settle,
    world: 3,
    parties: ["jknetwork", "JalebiStreet", "Stray Otters", "Lucky Comets"],
    links: [],
    cap: 0,
    rounds: 1,
    terms: [
        ("bid", "shared_cache", 52),
        ("share_1", "cr", 13),
        ("share_2", "cr", 13),
        ("share_3", "cr", 13),
        ("share_4", "cr", 13),
    ],
    reads: [
        "We jointly bid 52 World 3 credits for one shared cache. Each of the four teams contributes 13 credits.",
        "If we win, jknetwork owns the cache and will install it at IXP 4 for the homes of all four named teams.",
        "There are no hosting fees, usage charges, or other payments between teams.",
        "The last signature enters the bid. A winning agreement lasts through the end of the season.",
    ],
}
