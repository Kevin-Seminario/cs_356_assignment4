//! Round 1 candidate based on the A4 starter. It runs on every switch of your AS.
//!
//! At start-up the simulator tells each switch (in `input.a4`) which AS it
//! is in, the AS's switches and internal links, its border ports, where its
//! own address blocks are, and the routes your routing policy (`policy.rs`,
//! in this same program) chose: for each destination, the one neighbour it uses,
//! offered on every link to that neighbour. The Tier-1 always offers
//! 0.0.0.0/0, which carries everything your policy sent to the Tier-1.
//!
//! What the starter does with that:
//! - traffic for your own blocks goes to them inside the AS;
//! - everything else takes an offered route through the nearest exit
//!   (fewest hops inside your AS), and the Tier-1 only where nothing more
//!   specific is offered. With no agreements that is the Tier-1, at the hub
//!   nearest the switch.
//! - Trading packets whose only selected route is Tier-1 are dropped before
//!   incurring its fee; Trading on a more specific peer route still forwards.
//!
//! What it does not do: notice a failed link (no failover), use backup
//! routes, or queue by class (one plain queue per port). Those are yours.
//! `router.rs` has the helpers: shortest paths inside the AS, the
//! forwarding table, the data-plane program, and table updates.
//! `policy.rs` is the routing policy: the usual rule.
mod policy;
mod router;
use std::collections::BTreeMap;
use switch_program_sdk::*;

const LOG_INTERVAL_NS: u64 = 1_000_000_000;

pub struct Starter {
    startup_log: Vec<String>,
    startup_logged: bool,
}

/// Condense the route table to one line per destination AS and exit. This
/// makes the site's per-world log useful without spending its 64 KiB budget
/// on one line for every address block at every switch.
fn routing_snapshot(info: &wire::A4InfoW, me: u32, routes: &[(u32, u8, u16)]) -> Vec<String> {
    let mut lines = vec![format!(
        "a4_round1 as={} switch={} installed_routes={} offers={} buffer_bytes={} failover=off",
        info.as_id,
        me,
        routes.len(),
        info.routes.len(),
        info.queue_bytes
    )];
    for link in &info.internal {
        if link.a == me {
            lines.push(format!(
                "port={} internal_to_switch={}",
                link.a_port, link.b
            ));
        } else if link.b == me {
            lines.push(format!(
                "port={} internal_to_switch={}",
                link.b_port, link.a
            ));
        }
    }
    for border in info.borders.iter().filter(|b| b.switch == me) {
        lines.push(format!(
            "port={} border_to_as={} rel={}",
            border.port, border.peer_as, border.rel
        ));
    }
    let mut grouped: BTreeMap<(u32, u32, u8, u16), u32> = BTreeMap::new();
    for &(addr, len, port) in routes {
        if len == 0 {
            lines.push(format!("default=0.0.0.0/0 out_port={} trading=drop", port));
        } else if let Some(own) = info.own.iter().find(|p| p.addr == addr && p.len == len) {
            lines.push(format!(
                "own={}/{} class={} out_port={}",
                ip(addr),
                len,
                own.class,
                port
            ));
        } else if let Some(offer) = info.routes.iter().find(|r| r.addr == addr && r.len == len) {
            let dest_as = offer.as_path.last().copied().unwrap_or(0);
            let via_as = info
                .borders
                .iter()
                .find(|b| b.switch == offer.switch && b.port == offer.port)
                .map(|b| b.peer_as)
                .unwrap_or(0);
            *grouped
                .entry((dest_as, via_as, offer.rel, port))
                .or_default() += 1;
        } else {
            lines.push(format!("unmapped={}/{} out_port={}", ip(addr), len, port));
        }
    }
    for ((dest_as, via_as, rel, port), count) in grouped {
        lines.push(format!(
            "route_to_as={} via_as={} rel={} out_port={} prefixes={}",
            dest_as, via_as, rel, port, count
        ));
    }
    lines
}

fn ip(addr: u32) -> String {
    format!(
        "{}.{}.{}.{}",
        addr >> 24,
        (addr >> 16) & 255,
        (addr >> 8) & 255,
        addr & 255
    )
}

/// Lower is better; `None` refuses the route. `inside` is the number of
/// hops inside your AS from this switch to the border switch of the route.
fn rank(r: &wire::A4RouteW, inside: u32) -> Option<u64> {
    if r.rel == wire::A4_BACKUP {
        return None; // backup routes are for failover, which the starter lacks
    }
    let tier1 = (r.rel == wire::A4_TIER1) as u64;
    Some(r.as_path.len() as u64 * 1_000_000 + tier1 * 1_000 + inside as u64)
}

impl SwitchProgram for Starter {
    fn init(_: u32, _: Vec<u16>) -> (Self, ProgramSetup) {
        (
            Starter {
                startup_log: vec![],
                startup_logged: false,
            },
            router::setup(&[], false),
        )
    }
    fn init_with(input: InitInput) -> (Self, ProgramSetup) {
        let Some(info) = input.a4.as_ref() else {
            return Self::init(0, vec![]);
        };
        let no_failed_links = Default::default();
        let routes = router::table(info, input.switch_id, &no_failed_links, rank);
        let startup_log = routing_snapshot(info, input.switch_id, &routes);
        (
            Starter {
                startup_log,
                startup_logged: false,
            },
            router::setup(&routes, false),
        )
    }

    fn on_timer(&mut self, ev: TimerEvent) -> Vec<Action> {
        let mut out = Vec::new();
        if !self.startup_logged {
            out.extend(
                self.startup_log
                    .drain(..)
                    .map(|line| actions::log_write(line.into_bytes())),
            );
            self.startup_logged = true;
        } else {
            let total: u64 = ev.port_drops.iter().map(|d| d.dropped as u64).sum();
            let ports = ev
                .port_drops
                .iter()
                .map(|d| format!("{}:{}", d.port, d.dropped))
                .collect::<Vec<_>>()
                .join(",");
            out.push(actions::log_write(
                format!("egress_drops_last_s={} by_port=[{}]", total, ports).into_bytes(),
            ));
        }
        out.push(actions::schedule_timer(LOG_INTERVAL_NS));
        out
    }
}

switch_program!(Starter);
switch_program_sdk::policy_program!(policy::rank, policy::announce);
