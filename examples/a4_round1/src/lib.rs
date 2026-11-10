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

const LOG_INTERVAL_NS: u64 = 200_000_000;
// Split a conservative share of the site's 64 KiB per-world log among all
// switches. The remainder covers the site's timestamp/switch line prefixes.
const WORLD_LOG_PAYLOAD_BUDGET: usize = 56_000;

pub struct Starter {
    startup_log: Vec<String>,
    startup_logged: bool,
    runtime_log_bytes: usize,
    runtime_log_budget: usize,
}

/// Condense the route table to one line per destination AS and exit. This
/// makes the site's per-world log useful without spending its 64 KiB budget
/// on one line for every address block at every switch.
fn routing_snapshot(
    input: &InitInput,
    info: &wire::A4InfoW,
    routes: &[(u32, u8, u16)],
) -> Vec<String> {
    let me = input.switch_id;
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
    for (i, port) in input.local_ports.iter().enumerate() {
        let bps = input.port_capacity_bps.get(i).copied().unwrap_or(0);
        lines.push(format!("port={} capacity_bps={}", port, bps));
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
    // The grouped view above is cheap to scan. These lines retain exact
    // prefixes and every offered exit so we can investigate unexpected paths.
    for &(addr, len, out_port) in routes {
        if len == 0 || info.own.iter().any(|p| p.addr == addr && p.len == len) {
            continue;
        }
        if let Some(offer) = info.routes.iter().find(|r| r.addr == addr && r.len == len) {
            let via_as = info
                .borders
                .iter()
                .find(|b| b.switch == offer.switch && b.port == offer.port)
                .map(|b| b.peer_as)
                .unwrap_or(0);
            lines.push(format!(
                "selected={}/{} out_port={} via_as={} as_path={:?}",
                ip(addr),
                len,
                out_port,
                via_as,
                offer.as_path
            ));
        }
    }
    let mut offered: BTreeMap<(u32, u32, u8, u32, u16, u32), u32> = BTreeMap::new();
    for route in &info.routes {
        let dest_as = route.as_path.last().copied().unwrap_or(0);
        let via_as = info
            .borders
            .iter()
            .find(|b| b.switch == route.switch && b.port == route.port)
            .map(|b| b.peer_as)
            .unwrap_or(0);
        *offered
            .entry((
                dest_as,
                via_as,
                route.rel,
                route.switch,
                route.port,
                route.med,
            ))
            .or_default() += 1;
    }
    for ((dest_as, via_as, rel, exit_switch, exit_port, med), count) in offered {
        lines.push(format!(
            "offered_to_as={} via_as={} rel={} exit={}:{} med={} prefixes={}",
            dest_as, via_as, rel, exit_switch, exit_port, med, count
        ));
    }
    let per_switch = WORLD_LOG_PAYLOAD_BUDGET / info.switches.len().max(1);
    cap_log(lines, per_switch * 3 / 5)
}

fn cap_log(lines: Vec<String>, budget: usize) -> Vec<String> {
    let mut kept = Vec::new();
    let mut used = 0;
    let mut omitted = 0;
    for line in lines {
        if used + line.len() + 64 <= budget {
            used += line.len();
            kept.push(line);
        } else {
            omitted += 1;
        }
    }
    if omitted > 0 {
        kept.push(format!(
            "startup_lines_omitted={} budget_bytes={}",
            omitted, budget
        ));
    }
    kept
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
                runtime_log_bytes: 0,
                runtime_log_budget: 0,
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
        let startup_log = routing_snapshot(&input, info, &routes);
        let per_switch = WORLD_LOG_PAYLOAD_BUDGET / info.switches.len().max(1);
        (
            Starter {
                startup_log,
                startup_logged: false,
                runtime_log_bytes: 0,
                runtime_log_budget: per_switch - per_switch * 3 / 5,
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
            let policy: u64 = ev.port_drops.iter().map(|d| d.policy as u64).sum();
            let ports = ev
                .port_drops
                .iter()
                .map(|d| format!("{}:{}:q{:?}", d.port, d.dropped, d.queues))
                .collect::<Vec<_>>()
                .join(",");
            let line = format!(
                "egress_drops_last_200ms={} policy={} by_port=[{}]",
                total, policy, ports
            );
            if self.runtime_log_bytes + line.len() <= self.runtime_log_budget {
                self.runtime_log_bytes += line.len();
                out.push(actions::log_write(line.into_bytes()));
            }
        }
        out.push(actions::schedule_timer(LOG_INTERVAL_NS));
        out
    }
}

switch_program!(Starter);
switch_program_sdk::policy_program!(policy::rank, policy::announce);
