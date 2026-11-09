//! Routing policy: Gao-Rexford with a latency tie-break.
//!
//! The policy sees only its input (no memory, no agreements), so anything
//! that depends on a deal must show up as the neighbour's relationship
//! (`rel`). Signing transit instead of "peering with leaks" is therefore
//! how you export more: a transit customer is `A4_CUSTOMER` and the usual
//! announce rule already tells customers everything.
use switch_program_sdk::wire::{self, AnnounceInputW, RankInputW};

/// ASSUMPTION (check in the practice world): AS ids run 1.. in sector order,
/// four to a sector, so AS 1-4 are sector 0.
fn sector(a: u32) -> u32 {
    a.saturating_sub(1) / 4
}

/// 0 = neighbour is in my own sector (reached with no extra crossing),
/// 1 = in a neighbouring sector.
fn lat_hint(me: u32, neighbour: u32) -> u8 {
    (sector(me) != sector(neighbour)) as u8
}

pub fn rank(input: &RankInputW) -> usize {
    let cost = |rel: u8| match rel {
        wire::A4_CUSTOMER => 1,
        wire::A4_PEER => 2,
        _ => 3,
    };
    (0..input.routes.len())
        .min_by_key(|&i| {
            let r = &input.routes[i];
            (
                r.rel == wire::A4_TIER1, // Tier-1 last: $1.5/Mb, 40 Mb/s, 32 ms
                cost(r.rel),             // customer pays you, peer is free, provider costs
                r.as_path.len(),         // fewer ASes, fewer hops
                lat_hint(input.as_id, r.neighbour),
                r.as_path.clone(),       // deterministic, order-independent
            )
        })
        .unwrap_or(0)
}

/// Usual export rule. `LEAK_TO` is empty on purpose: add an AS id only when
/// a signed agreement pays you for carrying that neighbour's traffic.
const LEAK_TO: &[u32] = &[];

pub fn announce(input: &AnnounceInputW) -> bool {
    if input.route.as_path.contains(&input.to) {
        return false; // it would only be dropped as a loop
    }
    matches!(input.route.rel, wire::A4_OWN | wire::A4_CUSTOMER)
        || input.to_rel == wire::A4_CUSTOMER
        || LEAK_TO.contains(&input.to)
}