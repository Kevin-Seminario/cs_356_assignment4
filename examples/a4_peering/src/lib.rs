//! A4 peering + latency switch program.
//!
//! Built on the starter. Changes:
//! - exit ranking uses the neighbour's `med` (nearer entry point first), then
//!   inside hops, before falling back to the Tier-1;
//! - Calls/Trading ride a small strict-priority queue 0 on every port;
//! - hello packets on internal ports detect a dead link (timer 100 ms, down
//!   after 3 missed timers) and the table is rebuilt without it.
mod policy;
mod router;
use std::collections::{BTreeMap, BTreeSet};
use switch_program_sdk::wire::{self, A4InfoW};
use switch_program_sdk::*;

const TIMER_NS: u64 = 100_000_000;
const MISSED: u64 = 3;

pub struct Peering {
    me: u32,
    info: A4InfoW,
    down: BTreeSet<(u32, u16)>,
    installed: Vec<(u64, u32, u8, u16)>,
    next_id: u64,
    tick: u64,
    heard: BTreeMap<u16, u64>,
    internal: Vec<u16>,
}

/// Lower is better. Fewest ASes first (the policy already chose the
/// neighbour), Tier-1 last, then the neighbour's own entry-point hint
/// (`med`), then fewest hops inside our AS.
fn rank(r: &wire::A4RouteW, inside: u32) -> Option<u64> {
    if r.rel == wire::A4_BACKUP {
        return None;
    }
    let tier1 = (r.rel == wire::A4_TIER1) as u64;
    Some(r.as_path.len() as u64 * 1_000_000 + tier1 * 100_000 + (r.med as u64).min(99) * 1_000 + inside as u64)
}

fn pad(routes: &[(u32, u8, u16)]) -> Vec<(u64, u32, u8, u16)> {
    routes.iter().enumerate().map(|(i, &(a, l, p))| (1000 + i as u64, a, l, p)).collect()
}

impl SwitchProgram for Peering {
    fn init(_: u32, _: Vec<u16>) -> (Self, ProgramSetup) {
        let p = Peering {
            me: 0,
            info: A4InfoW::default(),
            down: Default::default(),
            installed: vec![],
            next_id: 100_000,
            tick: 0,
            heard: BTreeMap::new(),
            internal: vec![],
        };
        (p, router::setup(&[], true))
    }

    fn init_with(input: InitInput) -> (Self, ProgramSetup) {
        let Some(info) = input.a4.clone() else { return Self::init(0, vec![]) };
        let me = input.switch_id;
        let down = BTreeSet::new();
        let routes = router::table(&info, me, &down, rank);
        let internal = router::internal_ports(&info, me);
        let p = Peering {
            me,
            installed: pad(&routes),
            info,
            down,
            next_id: 100_000,
            tick: 0,
            heard: internal.iter().map(|&q| (q, 0)).collect(),
            internal,
        };
        (p, router::setup(&routes, true))
    }

    fn on_punt(&mut self, ev: PuntEvent) -> Vec<Action> {
        if ev.ip_proto == router::PROTO_HELLO {
            self.heard.insert(ev.ingress_port, self.tick);
        }
        Vec::new()
    }

    fn on_timer(&mut self, _ev: TimerEvent) -> Vec<Action> {
        let mut out = Vec::new();
        if self.info.switches.is_empty() {
            return out; // not an A4 world; nothing to do
        }
        if self.tick == 0 {
            // Calls + Trading in queue 0 (small, strict priority), the rest in 1.
            let mut ports = self.internal.clone();
            ports.extend(router::edge_ports(&self.info, self.me));
            for p in ports {
                out.push(actions::set_queues(
                    p,
                    Discipline::StrictPriority,
                    vec![QueueSpec::new(5), QueueSpec::new(12)],
                ));
            }
        }
        // Hellos, in the priority queue so a full port cannot hide them.
        for &p in &self.internal {
            out.push(actions::in_queue(
                0,
                actions::inject_packet(p, self.me, router::CTRL_IP, router::PROTO_HELLO, 1, vec![]),
            ));
        }
        self.tick += 1;
        // Which internal ports have gone quiet?
        let mut now_down = BTreeSet::new();
        if self.tick > MISSED {
            for &p in &self.internal {
                let last = self.heard.get(&p).copied().unwrap_or(0);
                if self.tick - last > MISSED {
                    now_down.insert((self.me, p));
                }
            }
        }
        if now_down != self.down {
            for &(_, p) in now_down.difference(&self.down) {
                out.push(actions::link_down(p));
            }
            for &(_, p) in self.down.difference(&now_down) {
                out.push(actions::link_up(p));
            }
            self.down = now_down;
            let routes = router::table(&self.info, self.me, &self.down, rank);
            out.extend(router::reinstall(&mut self.installed, &routes, &mut self.next_id));
        }
        out.push(actions::schedule_timer(TIMER_NS));
        out
    }
}

switch_program!(Peering);
switch_program_sdk::policy_program!(policy::rank, policy::announce);