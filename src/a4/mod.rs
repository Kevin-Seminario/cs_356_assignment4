//! A4: several ASes in one world, each running its own program, a Tier-1
//! run by the course, and money computed from what each AS delivered.
//!
//! The world file's `[a4]` section names the ASes, the signed agreements and
//! every app's traffic. Before programs are installed, `prepare` tells each
//! switch its AS, its border ports and the routes its neighbours offer (each
//! team's routing policy, `policy`, run over the signed agreements to a
//! fixed point), and sets up the border meters. After the run, each agreement's settlement program
//! reads the meters (`settle_all`), and `statement` turns deliveries and
//! settlements into money.

pub mod deal;
pub mod meter;
pub mod policy;
pub mod replay;
#[cfg(feature = "wasm")]
pub mod settle;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use switch_program_types as wire;

use crate::app::{App, TrafficPattern};
use crate::sim::Simulator;
use crate::types::AppId;
use crate::IpAddr;
use crate::world::{Prefix, World, WorldError, WorldHandles};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A4Spec {
    /// The Tier-1's AS id and its program.
    pub tier1_as: u32,
    pub tier1_program: String,
    #[serde(rename = "as")]
    pub ases: Vec<AsSpec>,
    #[serde(rename = "agreement", default)]
    pub agreements: Vec<Agreement>,
    #[serde(rename = "app", default)]
    pub apps: Vec<A4App>,
    #[serde(rename = "class")]
    pub classes: Vec<ClassSpec>,
    /// Tier-1 price per Mb, each direction, each end.
    pub tier1_price: f64,
    /// Simulated Mb per real Mb: a world may run at a fraction of the real
    /// rates, and money is computed on real Mb.
    #[serde(default = "one")]
    pub mb_scale: f64,
    /// Length of a meter epoch.
    #[serde(default = "five_s")]
    pub epoch_ms: u64,
    /// Bytes per second each switch's program may send itself (hellos,
    /// probes), with a one-second burst. Such packets use capacity but
    /// are never metered, so they are rationed instead.
    #[serde(default = "ten_k")]
    pub inject_bytes_per_s: u64,
    /// Bytes of program log each AS may write per run; a write past it is
    /// lost whole.
    #[serde(default = "sixty_four_k")]
    pub log_cap_bytes: usize,
}

fn one() -> f64 {
    1.0
}

fn sixty_four_k() -> usize {
    64 << 10
}

fn ten_k() -> u64 {
    10_000
}

fn five_s() -> u64 {
    5000
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsSpec {
    pub id: u32,
    pub sector: u32,
    pub slot: u32,
    pub program: String,
    /// The AS's address blocks, each announced as near `switch`.
    pub blocks: Vec<Block>,
    /// Memory limit of each of this AS's switch programs, in bytes, if not
    /// the usual one: a bought program-memory upgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub prefix: String,
    pub switch: u32,
}

/// `kind` is the label the switch programs see: `peer`, `transit` (`a`
/// buys from `b`) or `backup` (`a` buys from `b`). The money is whatever
/// `program` (a settlement program, relative to the world file) returns on
/// the meter readings, given `terms` (in dollars or plain numbers), and
/// never more than `max` dollars either way.
///
/// An agreement of two to four ASes names them in `parties` instead, with
/// the `links` it makes (`[x, y, kind]`, AS ids; x buys from y) and `caps`,
/// the most dollars each party pays in a round; its program is told how
/// many rounds it has settled (`rounds_run`) and the `state` (hex) it
/// returned in the last one. Without `parties` the agreement is the two
/// ASes `a` and `b`, one link of `kind`, both capped at `max`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Agreement {
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub a: u32,
    #[serde(default)]
    pub b: u32,
    pub program: String,
    #[serde(default)]
    pub terms: BTreeMap<String, f64>,
    #[serde(default)]
    pub max: f64,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parties: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<(u32, u32, String)>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub caps: Vec<f64>,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub rounds_run: u32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub state: String,
}

fn is_zero(n: &u32) -> bool {
    *n == 0
}

fn link_kind(name: &str) -> Option<u8> {
    match name {
        "peer" => Some(wire::A4_LINK_PEER),
        "transit" => Some(wire::A4_LINK_TRANSIT),
        "backup" => Some(wire::A4_LINK_BACKUP),
        _ => None,
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn unhex(s: &str) -> Option<Vec<u8>> {
    if s.len() % 2 != 0 || !s.is_ascii() {
        return None;
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).ok()).collect()
}

impl Agreement {
    pub fn parties(&self) -> Vec<u32> {
        if self.parties.is_empty() { vec![self.a, self.b] } else { self.parties.clone() }
    }

    /// The links it makes: (x, y, kind), AS ids.
    pub fn links(&self) -> impl Iterator<Item = (u32, u32, &str)> {
        let old = self.parties.is_empty().then(|| (self.a, self.b, self.kind.as_str()));
        old.into_iter().chain(self.links.iter().map(|(x, y, kind)| (*x, *y, kind.as_str())))
    }

    /// The most each party pays in a round, in dollars.
    pub fn caps(&self) -> Vec<f64> {
        if self.parties.is_empty() { vec![self.max, self.max] } else { self.caps.clone() }
    }

    /// Whether the agreement is well formed. One without `parties` is as
    /// it always was.
    pub fn check(&self) -> Result<(), String> {
        let p = &self.parties;
        if p.is_empty() {
            return if self.links.is_empty() && self.caps.is_empty() { Ok(()) } else { Err("links or caps without parties".into()) };
        }
        if !(2..=4).contains(&p.len()) || p.iter().collect::<BTreeSet<_>>().len() != p.len() {
            return Err("needs two to four different parties".into());
        }
        if self.caps.len() != p.len() || self.caps.iter().any(|c| !(*c >= 0.0)) {
            return Err("needs a cap of $0 or more for each party".into());
        }
        for (x, y, kind) in &self.links {
            if x == y || !p.contains(x) || !p.contains(y) {
                return Err(format!("the link {x}-{y} is not between two of its parties"));
            }
            if link_kind(kind).is_none() {
                return Err(format!("unknown kind of link: {kind}"));
            }
        }
        unhex(&self.state).map(|_| ()).ok_or_else(|| "state is not hex".into())
    }

    /// What its program reads, given a run's readings and outages of its
    /// parties. Call `check` first.
    pub fn input(&self, epoch_ms: u64, round_ms: u64, readings: Vec<wire::ReadingW>, outages: Vec<wire::OutageW>) -> wire::SettleInput2W {
        let parties = self.parties();
        let index = |x: u32| parties.iter().position(|p| *p == x).unwrap_or(0) as u32;
        wire::SettleInput2W {
            links: self.links().map(|(x, y, kind)| (index(x), index(y), link_kind(kind).unwrap_or(0))).collect(),
            parties,
            terms: self.terms.iter().map(|(k, v)| (k.clone(), (v * 1e6).round() as i64)).collect(),
            rounds_run: self.rounds_run,
            state: unhex(&self.state).unwrap_or_default(),
            epoch_ms,
            round_ms,
            readings,
            outages,
        }
    }
}

/// One app's traffic: `rate_mbps` spread evenly over `to`, all of class
/// `class`. `role` is `home`, `company` or `external`; a company row names
/// the company, and its sector's contract is split among the ASes its
/// sessions go through.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct A4App {
    pub id: u32,
    pub class: u8,
    pub rate_mbps: f64,
    pub to: Vec<u32>,
    pub role: String,
    #[serde(default)]
    pub company: Option<String>,
    #[serde(default)]
    pub sector: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassSpec {
    pub id: u8,
    pub name: String,
    pub target_ms: f64,
    /// What a home pays per Mb on time.
    pub price: f64,
    /// The contract of the company that is this class, per sector.
    pub contract: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rel {
    Customer,
    Peer,
    Provider,
    BackupProvider,
    BackupCustomer,
    Tier1,
    Tier1Customer,
}

impl Rel {
    fn label(self) -> u8 {
        match self {
            Rel::Customer | Rel::Tier1Customer => wire::A4_CUSTOMER,
            Rel::Peer => wire::A4_PEER,
            Rel::Provider => wire::A4_PROVIDER,
            Rel::BackupProvider | Rel::BackupCustomer => wire::A4_BACKUP,
            Rel::Tier1 => wire::A4_TIER1,
        }
    }
}

/// The traffic seed of app `id` in a world with `[world] seed`: every
/// world and round (the website sets the seed per round) draws its own
/// packets, and the same seed always draws the same.
fn app_seed(world_seed: u64, id: u32) -> u64 {
    let w = world_seed.wrapping_mul(0x9e37_79b9_7f4a_7c15).rotate_left(29);
    0x9e37_79b9_7f4a_7c15u64 ^ w ^ (id as u64).wrapping_mul(0x2545_f491_4f6c_dd1d) | 1
}

impl A4Spec {
    fn class(&self, id: u8) -> Option<&ClassSpec> {
        self.classes.iter().find(|c| c.id == id)
    }

    /// The app `id` as the world builds it: open loop at its rate, round
    /// robin over its destinations, its class in the DSCP field.
    pub fn app(
        &self,
        id: u32,
        prefix: Prefix,
        handles: &WorldHandles,
        size_bytes: u64,
        load_scale: f64,
        world_seed: u64,
    ) -> Result<App, WorldError> {
        let bad = |m: String| WorldError::Invalid(m);
        if id == 0 {
            // The meters tell customer traffic by its app; 0 is no app.
            return Err(bad("app ids start at 1".into()));
        }
        let row = self.apps.iter().find(|a| a.id == id).ok_or_else(|| bad(format!("app {id} has no [[a4.app]] row")))?;
        let dests: Vec<IpAddr> = row
            .to
            .iter()
            .map(|d| handles.app_prefixes.get(d).map(|p| IpAddr(p.first_host())).ok_or_else(|| bad(format!("app {id} sends to unknown app {d}"))))
            .collect::<Result<_, _>>()?;
        if dests.is_empty() || !(row.rate_mbps > 0.0) {
            return Err(bad(format!("app {id} needs a rate and at least one destination")));
        }
        // Poisson, not constant bitrate: synchronised senders would make a
        // destination's place in every round robin decide its losses.
        let lambda_pps = row.rate_mbps * 1e6 * load_scale / (size_bytes * 8) as f64;
        let seed = app_seed(world_seed, id);
        let mut app = App::new(
            AppId::new(id),
            IpAddr(prefix.first_host()),
            prefix.len,
            dests[0],
            32,
            TrafficPattern::Poisson { lambda_pps, size_bytes, seed },
        );
        for d in &dests[1..] {
            app.add_destination(*d);
        }
        app.dscp = row.class;
        Ok(app)
    }

    /// Every agreement's links, in file order.
    fn links(&self) -> impl Iterator<Item = (u32, u32, &str)> {
        self.agreements.iter().flat_map(Agreement::links)
    }

    fn rel(&self, x: u32, y: u32) -> Option<Rel> {
        if x == self.tier1_as {
            return Some(Rel::Tier1Customer);
        }
        if y == self.tier1_as {
            return Some(Rel::Tier1);
        }
        let main = self.links().find_map(|(a, b, kind)| match (kind, a == x && b == y, a == y && b == x) {
            ("peer", true, _) | ("peer", _, true) => Some(Rel::Peer),
            ("transit", true, _) => Some(Rel::Provider),
            ("transit", _, true) => Some(Rel::Customer),
            _ => None,
        });
        main.or_else(|| self.backup(x, y))
    }

    /// A backup agreement between x and y, as x sees it. A pair may hold
    /// one beside a peering or transit agreement.
    fn backup(&self, x: u32, y: u32) -> Option<Rel> {
        self.links().find_map(|(a, b, kind)| match (kind, a == x && b == y, a == y && b == x) {
            ("backup", true, _) => Some(Rel::BackupProvider),
            ("backup", _, true) => Some(Rel::BackupCustomer),
            _ => None,
        })
    }
}

/// Owner of each switch.
fn owners(world: &World) -> BTreeMap<u32, u32> {
    world.switches.iter().map(|s| (s.id, s.as_id)).collect()
}

/// Hop distances inside one AS from `from`.
fn hops_inside(world: &World, owner: &BTreeMap<u32, u32>, from: u32) -> BTreeMap<u32, u32> {
    let me = owner[&from];
    let mut dist = BTreeMap::from([(from, 0)]);
    let mut q = VecDeque::from([from]);
    while let Some(s) = q.pop_front() {
        for l in &world.links {
            let n = if l.a == s { l.b } else if l.b == s { l.a } else { continue };
            if owner[&n] == me && !dist.contains_key(&n) {
                dist.insert(n, dist[&s] + 1);
                q.push_back(n);
            }
        }
    }
    dist
}

/// How the round's routes were computed.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Routing {
    /// Passes until no AS changed its routes, or the cap.
    pub passes: u32,
    pub settled: bool,
    /// ASes whose policy failed and got the usual one, and why.
    pub failed: BTreeMap<u32, String>,
}

/// A cap on passes: policies that never agree stop here, on the last pass.
pub const MAX_PASSES: u32 = 32;

/// The route each AS uses to each destination AS, every AS running its
/// team's policy: each pass, every AS ranks what its neighbours announced
/// in the pass before, until no AS changes or `MAX_PASSES`. Only routes
/// over signed agreements are offered, plus the Tier-1. A policy that fails
/// anywhere is replaced by the usual one and the computation restarts.
fn best_routes(
    spec: &A4Spec,
    nbrs: &BTreeMap<u32, BTreeSet<u32>>,
    policies: &mut BTreeMap<u32, Box<dyn policy::Policy>>,
) -> (BTreeMap<u32, BTreeMap<u32, wire::PolicyRouteW>>, Routing) {
    let mut failed = BTreeMap::new();
    loop {
        match fixed_point(spec, nbrs, policies, &failed) {
            Ok((best, passes, settled)) => return (best, Routing { passes, settled, failed }),
            Err((as_id, why)) => {
                failed.insert(as_id, why);
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn fixed_point(
    spec: &A4Spec,
    nbrs: &BTreeMap<u32, BTreeSet<u32>>,
    policies: &mut BTreeMap<u32, Box<dyn policy::Policy>>,
    failed: &BTreeMap<u32, String>,
) -> Result<(BTreeMap<u32, BTreeMap<u32, wire::PolicyRouteW>>, u32, bool), (u32, String)> {
    let tier1 = spec.tier1_as;
    let ases: Vec<u32> = spec.ases.iter().map(|a| a.id).collect();
    let own = |x: u32| wire::PolicyRouteW { neighbour: x, rel: wire::A4_OWN, as_path: Vec::new() };
    let mut usual = policy::Usual;
    let mut best: BTreeMap<u32, BTreeMap<u32, wire::PolicyRouteW>> =
        ases.iter().map(|&x| (x, BTreeMap::from([(x, own(x))]))).collect();
    for pass in 1..=MAX_PASSES {
        let mut next = BTreeMap::new();
        for &x in &ases {
            let mut offered: BTreeMap<u32, Vec<wire::PolicyRouteW>> = BTreeMap::new();
            for &y in nbrs.get(&x).into_iter().flatten() {
                let Some(r) = spec.rel(x, y).filter(|r| matches!(r, Rel::Customer | Rel::Peer | Rel::Provider)) else { continue };
                let to_rel = spec.rel(y, x).unwrap().label();
                let py: &mut dyn policy::Policy = match policies.get_mut(&y) {
                    Some(p) if !failed.contains_key(&y) => p.as_mut(),
                    _ => &mut usual,
                };
                for (&d, route) in &best[&y] {
                    // Routes through the Tier-1 are never passed on.
                    if d == x || route.rel == wire::A4_TIER1 || route.as_path.contains(&x) {
                        continue;
                    }
                    let input = wire::AnnounceInputW { as_id: y, dest: d, route: route.clone(), to: x, to_rel };
                    if py.announce(&input).map_err(|e| (y, e))? {
                        let mut as_path = vec![y];
                        as_path.extend(&route.as_path);
                        offered.entry(d).or_default().push(wire::PolicyRouteW { neighbour: y, rel: r.label(), as_path });
                    }
                }
            }
            let px: &mut dyn policy::Policy = match policies.get_mut(&x) {
                Some(p) if !failed.contains_key(&x) => p.as_mut(),
                _ => &mut usual,
            };
            let has_tier1 = nbrs.get(&x).is_some_and(|n| n.contains(&tier1));
            let mut mine = BTreeMap::from([(x, own(x))]);
            for &d in ases.iter().filter(|&&d| d != x) {
                let mut routes = offered.remove(&d).unwrap_or_default();
                if has_tier1 {
                    routes.push(wire::PolicyRouteW { neighbour: tier1, rel: wire::A4_TIER1, as_path: vec![tier1, d] });
                }
                if routes.is_empty() {
                    continue;
                }
                let input = wire::RankInputW { as_id: x, dest: d, routes };
                let i = px.rank(&input).map_err(|e| (x, e))?;
                mine.insert(d, input.routes[i].clone());
            }
            next.insert(x, mine);
        }
        if next == best {
            return Ok((best, pass, true));
        }
        best = next;
    }
    Ok((best, MAX_PASSES, false))
}

/// Fill `sim.a4_info` for every switch: its AS, border ports and offered
/// routes, each AS running its policy from `policies` (the usual one where
/// none is given). Call after `build`, before installing programs.
pub fn prepare(
    world: &World,
    handles: &WorldHandles,
    sim: &mut Simulator,
    policies: &mut BTreeMap<u32, Box<dyn policy::Policy>>,
) -> Result<Routing, WorldError> {
    let spec = world.a4.as_ref().ok_or_else(|| WorldError::Invalid("not an A4 world".into()))?;
    for g in &spec.agreements {
        let known = g.parties.iter().all(|p| spec.ases.iter().any(|a| a.id == *p));
        g.check()
            .and_then(|_| if known { Ok(()) } else { Err("a party is not an AS of this world".into()) })
            .map_err(|e| WorldError::Invalid(format!("agreement of {:?}: {e}", g.parties())))?;
    }
    let owner = owners(world);
    let tier1 = spec.tier1_as;
    sim.inject_budget = Some(spec.inject_bytes_per_s);
    let log_owner = owner.iter().map(|(&s, &a)| (crate::types::SwitchId::new(s), a)).collect();
    sim.as_logs = Some(crate::team_log::AsLogs::new(spec.log_cap_bytes, log_owner));

    // Border ports and AS adjacency.
    let mut borders: BTreeMap<u32, Vec<(u32, u16, u32, u32)>> = BTreeMap::new(); // as -> (switch, port, peer_as, peer_switch)
    let mut nbrs: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut internal: BTreeMap<u32, Vec<wire::A4LinkW>> = BTreeMap::new();
    for l in &world.links {
        let (oa, ob) = (owner[&l.a], owner[&l.b]);
        let pa = world.port_toward(l.a, l.b).unwrap();
        let pb = world.port_toward(l.b, l.a).unwrap();
        if oa == ob {
            internal.entry(oa).or_default().push(wire::A4LinkW { a: l.a, a_port: pa, b: l.b, b_port: pb });
        } else {
            borders.entry(oa).or_default().push((l.a, pa, ob, l.b));
            borders.entry(ob).or_default().push((l.b, pb, oa, l.a));
            nbrs.entry(oa).or_default().insert(ob);
            nbrs.entry(ob).or_default().insert(oa);
        }
    }

    // Each AS's own app prefixes, and its blocks.
    let mut own: BTreeMap<u32, Vec<wire::A4PrefixW>> = BTreeMap::new();
    for a in &world.apps {
        let p = handles.app_prefixes[&a.id];
        let class = spec.apps.iter().find(|r| r.id == a.id).map_or(0, |r| r.class);
        own.entry(owner[&a.switch]).or_default().push(wire::A4PrefixW {
            addr: p.addr,
            len: p.len,
            switch: a.switch,
            port: world.app_port(a.id),
            class,
        });
    }
    let mut blocks: BTreeMap<u32, Vec<(Prefix, u32)>> = BTreeMap::new();
    for a in &spec.ases {
        for b in &a.blocks {
            blocks.entry(a.id).or_default().push((Prefix::parse(&b.prefix)?, b.switch));
        }
    }

    let (best, routing) = best_routes(spec, &nbrs, policies);

    // What y offers x on one link (x's switch sx, port px; y's switch sy).
    let offer = |x: u32, sx: u32, px: u16, y: u32, sy: u32| -> Vec<wire::A4RouteW> {
        let mut out = Vec::new();
        let Some(r) = spec.rel(x, y) else { return out };
        let route_as = |rel: Rel, p: &Prefix, path: Vec<u32>, med: u32| wire::A4RouteW {
            addr: p.addr,
            len: p.len,
            switch: sx,
            port: px,
            rel: rel.label(),
            as_path: path,
            med,
        };
        let route = |p: &Prefix, path: Vec<u32>, med: u32| route_as(r, p, path, med);
        let near = |d: u32, from: u32| -> Vec<(Prefix, u32)> {
            let dist = hops_inside(world, &owner, from);
            blocks.get(&d).into_iter().flatten().map(|(p, s)| (*p, *dist.get(s).unwrap_or(&99))).collect()
        };
        match r {
            Rel::Tier1 => out.push(wire::A4RouteW { addr: 0, len: 0, switch: sx, port: px, rel: r.label(), as_path: vec![tier1], med: 0 }),
            Rel::Tier1Customer => {
                // An AS asks the Tier-1 to deliver its own blocks only.
                for (p, med) in near(y, sy) {
                    out.push(route(&p, vec![y], med));
                }
            }
            Rel::BackupProvider | Rel::BackupCustomer => {}
            Rel::Customer | Rel::Peer | Rel::Provider => {
                // Only the routes x's policy chose through y, on every link to y.
                for (&d, chosen) in &best[&x] {
                    if chosen.neighbour != y || chosen.rel == wire::A4_OWN {
                        continue;
                    }
                    let path = chosen.as_path.clone();
                    if d == y {
                        for (p, med) in near(y, sy) {
                            out.push(route(&p, path.clone(), med));
                        }
                    } else {
                        for (p, _) in blocks.get(&d).into_iter().flatten() {
                            out.push(route(p, path.clone(), 0));
                        }
                    }
                }
            }
        }
        match spec.backup(x, y) {
            // The seller carries the buyer's own blocks between its towns.
            Some(Rel::BackupProvider) => {
                for (p, _) in near(x, sx) {
                    out.push(route_as(Rel::BackupProvider, &p, vec![y, x], 0));
                }
            }
            // The seller reaches the buyer's blocks the nearer way.
            Some(Rel::BackupCustomer) if r == Rel::BackupCustomer => {
                for (p, med) in near(y, sy) {
                    out.push(route_as(Rel::BackupCustomer, &p, vec![y], med));
                }
            }
            _ => {}
        }
        out
    };

    let mut ases: Vec<u32> = spec.ases.iter().map(|a| a.id).collect();
    ases.push(tier1);
    for &x in &ases {
        let switches: Vec<u32> = world.switches.iter().filter(|s| s.as_id == x).map(|s| s.id).collect();
        let mut info = wire::A4InfoW {
            as_id: x,
            switches,
            internal: internal.remove(&x).unwrap_or_default(),
            borders: Vec::new(),
            own: own.remove(&x).unwrap_or_default(),
            routes: Vec::new(),
            queue_bytes: 0,
        };
        for &(sx, px, y, sy) in borders.get(&x).into_iter().flatten() {
            let rel = spec.rel(x, y).map_or(0, Rel::label);
            info.borders.push(wire::A4BorderW { switch: sx, port: px, peer_as: y, rel });
            info.routes.extend(offer(x, sx, px, y, sy));
        }
        for s in world.switches.iter().filter(|s| s.as_id == x) {
            let queue_bytes = s.queue_capacity_bytes.unwrap_or(world.params.queue_capacity_bytes);
            sim.a4_info.insert(s.id, wire::A4InfoW { queue_bytes, ..info.clone() });
        }
    }
    let target = spec.classes.iter().map(|c| (c.id, Duration::from_secs_f64(c.target_ms / 1e3))).collect();
    sim.a4_meter = Some(meter::Meter::new(Duration::from_millis(spec.epoch_ms), owner, target));
    Ok(routing)
}

/// What one agreement's program said: what each party pays, in dollars
/// (negative: receives), and the state it keeps for the next round (hex);
/// or why nobody paid, and the state it had.
#[derive(Debug, Clone, Serialize)]
pub struct Settled {
    pub parties: Vec<u32>,
    pub pay: Vec<f64>,
    pub state: String,
    pub error: Option<String>,
}

/// Each agreement's input, in the order of `spec.agreements`: its terms,
/// the readings between its parties at the real rates, and the outages of
/// its parties' links in a run of `round_ms`.
pub fn settle_inputs(
    world: &World,
    sim: &Simulator,
    failures: Option<&crate::failures::Schedule>,
    round_ms: u64,
) -> Vec<wire::SettleInput2W> {
    let spec = world.a4.as_ref().expect("an A4 world");
    let outages = outages(world, failures, round_ms);
    spec.agreements
        .iter()
        .map(|g| {
            let p = g.parties();
            g.input(
                spec.epoch_ms,
                round_ms,
                readings(world, sim, |x, y| x != y && p.contains(&x) && p.contains(&y)),
                outages.iter().filter(|o| p.contains(&o.as_id)).copied().collect(),
            )
        })
        .collect()
}

/// One AS's meter readings and outages, as a settlement program sees them:
/// every reading to or from the AS, and the outages of it and its
/// neighbours.
#[derive(Debug, Clone, Serialize)]
pub struct AsReadings {
    pub as_id: u32,
    pub epoch_ms: u64,
    pub round_ms: u64,
    pub readings: Vec<wire::ReadingW>,
    pub outages: Vec<wire::OutageW>,
}

/// Every AS's readings, the Tier-1 included, by AS id.
pub fn as_readings(
    world: &World,
    sim: &Simulator,
    failures: Option<&crate::failures::Schedule>,
    round_ms: u64,
) -> Vec<AsReadings> {
    let spec = world.a4.as_ref().expect("an A4 world");
    let owner = owners(world);
    let outages = outages(world, failures, round_ms);
    let ids: BTreeSet<u32> = spec.ases.iter().map(|a| a.id).chain([spec.tier1_as]).collect();
    ids.into_iter()
        .map(|n| {
            let mut near = BTreeSet::from([n]);
            for l in &world.links {
                let (x, y) = (owner[&l.a], owner[&l.b]);
                if x == n || y == n {
                    near.extend([x, y]);
                }
            }
            AsReadings {
                as_id: n,
                epoch_ms: spec.epoch_ms,
                round_ms,
                readings: readings(world, sim, |x, y| x == n || y == n),
                outages: outages.iter().filter(|o| near.contains(&o.as_id)).copied().collect(),
            }
        })
        .collect()
}

/// The meter's readings between the pairs `keep` takes, at the real rates.
fn readings(world: &World, sim: &Simulator, keep: impl Fn(u32, u32) -> bool) -> Vec<wire::ReadingW> {
    let scale = world.a4.as_ref().expect("an A4 world").mb_scale;
    let real = |b: u64| (b as f64 / scale).round() as u64;
    sim.a4_meter
        .as_ref()
        .map(|m| &m.readings)
        .into_iter()
        .flatten()
        .filter(|((x, y, _, _), _)| keep(*x, *y))
        .map(|(&(from, to, epoch, class), r)| wire::ReadingW {
            from,
            to,
            epoch,
            class,
            bytes: real(r.bytes),
            on_time: real(r.on_time),
            late: real(r.late),
            lost: real(r.lost),
            returned: real(r.returned),
            returned_on_time: real(r.returned_on_time),
        })
        .collect()
}

/// Each failure in the schedule as an outage of the AS at either end, cut
/// at `round_ms`.
fn outages(world: &World, failures: Option<&crate::failures::Schedule>, round_ms: u64) -> Vec<wire::OutageW> {
    let spec = world.a4.as_ref().expect("an A4 world");
    let owner = owners(world);
    let hubs: BTreeSet<u32> = spec.ases.iter().flat_map(|a| a.blocks.iter().map(|b| b.switch)).collect();
    let mut outages: Vec<wire::OutageW> = Vec::new();
    for f in failures.map(|s| s.failures.as_slice()).unwrap_or_default() {
        let [x, y] = f.link;
        let (ox, oy) = (owner[&x], owner[&y]);
        let (start_ms, end_ms) = (f.down_ms.min(round_ms), f.up_ms.min(round_ms));
        let what = if ox != oy {
            wire::A4_OUT_BORDER
        } else if hubs.contains(&x) && hubs.contains(&y) {
            wire::A4_OUT_CROSSING
        } else {
            wire::A4_OUT_INSIDE
        };
        for as_id in BTreeSet::from([ox, oy]) {
            outages.push(wire::OutageW { as_id, what, start_ms, end_ms });
        }
    }
    outages
}

/// Run every agreement's settlement program. `modules` holds each
/// agreement's program by its `program` path. No party pays more than its
/// cap (see `settle::settle`).
#[cfg(feature = "wasm")]
pub fn settle_all(
    world: &World,
    sim: &Simulator,
    failures: Option<&crate::failures::Schedule>,
    round_ms: u64,
    modules: &BTreeMap<String, Vec<u8>>,
) -> Vec<Settled> {
    let spec = world.a4.as_ref().expect("an A4 world");
    let limits = settle::SettleLimits::default();
    spec.agreements
        .iter()
        .zip(settle_inputs(world, sim, failures, round_ms))
        .map(|(g, input)| {
            let result = g
                .check()
                .and_then(|_| modules.get(&g.program).ok_or_else(|| format!("no program {}", g.program)))
                .and_then(|m| settle::settle(m, &input, &g.caps(), &limits));
            let (pay, state, error) = match result {
                Ok((pay, state)) => (pay, hex(&state), None),
                Err(why) => (vec![0.0; input.parties.len()], g.state.clone(), Some(why)),
            };
            Settled { parties: input.parties, pay, state, error }
        })
        .collect()
}

/// What one AS earned in one run.
#[derive(Debug, Clone, Default, Serialize)]
pub struct Statement {
    pub as_id: u32,
    /// Homes: $ by class name.
    pub homes: BTreeMap<String, f64>,
    /// Companies: $ by company name, and the AS's on-time rate q for it.
    pub companies: BTreeMap<String, f64>,
    pub q: BTreeMap<String, f64>,
    pub value: f64,
    /// Per class: Mb offered and on time, for traffic that starts or ends
    /// at one of this AS's homes.
    pub offered_mb: BTreeMap<String, f64>,
    pub on_time_mb: BTreeMap<String, f64>,
    /// Settlements received and paid, by counterpart AS, and the
    /// agreements whose program failed (it paid nothing), and why.
    pub paid_by: BTreeMap<u32, f64>,
    pub paid_to: BTreeMap<u32, f64>,
    pub failed: BTreeMap<u32, String>,
    /// Mb on this AS's Tier-1 links, both directions, and the fee.
    pub tier1_mb: f64,
    pub tier1_fee: f64,
    /// value + received - paid - Tier-1 fee.
    pub utility: f64,
    /// The round's routes did not settle (policies kept changing them), so
    /// the last pass was used.
    pub routes_unsettled: bool,
    /// This AS's routing policy failed, so it used the usual one; why.
    pub policy_failed: Option<String>,
    /// One of this AS's switch programs stopped during the run; why (the
    /// lowest-numbered switch's reason).
    pub program_failed: Option<String>,
}

/// Every AS's statement from a finished run, with the settlements the
/// agreements' programs returned. The Tier-1 fee is priced on the bytes its
/// links carried, both directions.
pub fn statement(world: &World, sim: &Simulator, settled: &[Settled]) -> Vec<Statement> {
    let spec = world.a4.as_ref().expect("an A4 world");
    let owner = owners(world);
    let size_mb = (world.workload.packet_size_bytes() * 8) as f64 / 1e6 / spec.mb_scale;
    let row = |id: u32| spec.apps.iter().find(|r| r.id == id);
    let app_of_ip: BTreeMap<u32, u32> = sim.apps.iter().map(|(id, a)| (a.ip.0, id.raw())).collect();
    let app_as = |id: u32| owner[&world.apps.iter().find(|a| a.id == id).unwrap().switch];

    // Per (src app, dst app): packets sent, delivered on time.
    let mut sent: BTreeMap<(u32, u32), f64> = BTreeMap::new();
    let mut on_time: BTreeMap<(u32, u32), f64> = BTreeMap::new();
    for (id, a) in &sim.apps {
        let Some(r) = row(id.raw()) else { continue };
        let per = a.metrics.packets_sent as f64 / r.to.len() as f64;
        for d in &r.to {
            *sent.entry((id.raw(), *d)).or_default() += per;
        }
    }
    for (id, a) in &sim.apps {
        for (src, delay) in a.metrics.samples() {
            let Some(&s) = app_of_ip.get(&src) else { continue };
            let Some(c) = row(s).and_then(|r| spec.class(r.class)) else { continue };
            if delay.as_secs_f64() * 1e3 <= c.target_ms {
                *on_time.entry((s, id.raw())).or_default() += 1.0;
            }
        }
    }

    let mut out: BTreeMap<u32, Statement> =
        spec.ases.iter().map(|a| (a.id, Statement { as_id: a.id, ..Default::default() })).collect();
    // Homes pay for what they send and receive on time.
    let mut q_num: BTreeMap<(u32, String, u32), (f64, f64)> = BTreeMap::new(); // (sector, company, via) -> (on time, sent)
    for (&(s, d), &n) in &sent {
        let ok = on_time.get(&(s, d)).copied().unwrap_or(0.0);
        let (rs, rd) = (row(s).unwrap(), row(d));
        let c = spec.class(rs.class).unwrap();
        for (app, r) in [(s, Some(rs)), (d, rd)] {
            let Some(r) = r else { continue };
            if r.role == "home" {
                if let Some(st) = out.get_mut(&app_as(app)) {
                    *st.homes.entry(c.name.clone()).or_default() += ok * size_mb * c.price;
                    *st.offered_mb.entry(c.name.clone()).or_default() += n * size_mb;
                    *st.on_time_mb.entry(c.name.clone()).or_default() += ok * size_mb;
                }
            }
            if r.role == "company" {
                let key = (r.sector.unwrap_or(0), r.company.clone().unwrap_or_default(), app_as(app));
                let e = q_num.entry(key).or_default();
                e.0 += ok;
                e.1 += n;
            }
        }
    }
    // Each company's contract, split by q squared among its sector's ASes.
    let mut by_company: BTreeMap<(u32, String), Vec<(u32, f64)>> = BTreeMap::new();
    for ((sector, name, via), (ok, n)) in &q_num {
        let q = if *n > 0.0 { ok / n } else { 0.0 };
        by_company.entry((*sector, name.clone())).or_default().push((*via, q));
    }
    for ((_, name), qs) in &by_company {
        let contract = spec
            .apps
            .iter()
            .find(|r| r.company.as_deref() == Some(name.as_str()))
            .and_then(|r| spec.class(r.class))
            .map_or(0.0, |c| c.contract);
        let total: f64 = qs.iter().map(|(_, q)| q * q).sum();
        for (via, q) in qs {
            let share = if total > 0.0 { q * q / total } else { 1.0 / qs.len() as f64 };
            if let Some(st) = out.get_mut(via) {
                *st.companies.entry(name.clone()).or_default() += contract * share;
                st.q.insert(name.clone(), *q);
            }
        }
    }
    // Bytes on the links between each pair of ASes, in real Mb.
    let mut pair_mb: BTreeMap<(u32, u32), f64> = BTreeMap::new();
    for l in &world.links {
        let (oa, ob) = (owner[&l.a], owner[&l.b]);
        if oa == ob {
            continue;
        }
        let mut bytes = 0u64;
        for (me, other) in [(l.a, l.b), (l.b, l.a)] {
            let port = world.port_toward(me, other).unwrap() as usize;
            if let Some(sw) = sim.switches.get(&crate::types::SwitchId::new(me)) {
                bytes += sw.egress.get(port).map_or(0, |e| e.bytes_sent);
            }
        }
        *pair_mb.entry((oa.min(ob), oa.max(ob))).or_default() += bytes as f64 * 8.0 / 1e6 / spec.mb_scale;
    }
    let mb = |x: u32, y: u32| pair_mb.get(&(x.min(y), x.max(y))).copied().unwrap_or(0.0);
    for s in settled {
        let each = || s.parties.iter().copied().zip(s.pay.iter().copied());
        if let Some(why) = &s.error {
            for (me, other) in each().flat_map(|(me, _)| each().map(move |(other, _)| (me, other))).filter(|(me, other)| me != other) {
                if let Some(st) = out.get_mut(&me) {
                    st.failed.insert(other, why.clone());
                }
            }
        }
        // Each payer pays each receiver its share of all that is received.
        let received: f64 = each().filter(|(_, p)| *p < 0.0).map(|(_, p)| -p).sum();
        for (payer, paid) in each().filter(|(_, p)| *p > 0.0) {
            for (payee, got) in each().filter(|(_, p)| *p < 0.0) {
                let amount = paid * (-got / received);
                if let Some(st) = out.get_mut(&payer) {
                    *st.paid_to.entry(payee).or_default() += amount;
                }
                if let Some(st) = out.get_mut(&payee) {
                    *st.paid_by.entry(payer).or_default() += amount;
                }
            }
        }
    }
    for st in out.values_mut() {
        st.value = st.homes.values().sum::<f64>() + st.companies.values().sum::<f64>();
        st.tier1_mb = mb(st.as_id, spec.tier1_as);
        st.tier1_fee = spec.tier1_price * st.tier1_mb;
        st.utility = st.value + st.paid_by.values().sum::<f64>() - st.paid_to.values().sum::<f64>() - st.tier1_fee;
    }
    out.into_values().collect()
}

/// Packets dropped at switch egress queues, by where the link goes:
/// `crossing` (a hub-to-hub link inside an AS), `inside`, `ixp`,
/// `tier1_in` (Tier-1 toward an AS), `tier1_out` (an AS toward the Tier-1).
pub fn drops(world: &World, sim: &Simulator) -> BTreeMap<String, u64> {
    let spec = world.a4.as_ref().expect("an A4 world");
    let owner = owners(world);
    let hubs: BTreeSet<u32> = spec.ases.iter().flat_map(|a| a.blocks.iter().map(|b| b.switch)).collect();
    let mut out: BTreeMap<String, u64> = BTreeMap::new();
    for l in &world.links {
        for (me, other) in [(l.a, l.b), (l.b, l.a)] {
            let port = world.port_toward(me, other).unwrap() as usize;
            let Some(sw) = sim.switches.get(&crate::types::SwitchId::new(me)) else { continue };
            let n = sw.egress.get(port).map_or(0, |e| e.packets_dropped);
            let kind = if owner[&me] == spec.tier1_as {
                "tier1_in"
            } else if owner[&other] == spec.tier1_as {
                "tier1_out"
            } else if owner[&me] != owner[&other] {
                "ixp"
            } else if hubs.contains(&me) && hubs.contains(&other) {
                "crossing"
            } else {
                "inside"
            };
            *out.entry(kind.into()).or_default() += n;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::policy::{Policy, Usual};
    use super::*;

    /// ASes 1, 2, 3 and the Tier-1 (9); `deals` are peerings.
    fn world(deals: &[(u32, u32)]) -> (A4Spec, BTreeMap<u32, BTreeSet<u32>>) {
        let mut toml = String::from("tier1_as = 9\ntier1_program = \"t\"\ntier1_price = 1.0\n");
        for i in 1..=3 {
            toml += &format!("[[as]]\nid = {i}\nsector = 0\nslot = {i}\nprogram = \"p\"\nblocks = []\n");
        }
        for (a, b) in deals {
            toml += &format!("[[agreement]]\nkind = \"peer\"\na = {a}\nb = {b}\nprogram = \"s\"\nmax = 1.0\n");
        }
        toml += "[[class]]\nid = 1\nname = \"Web\"\ntarget_ms = 200.0\nprice = 1.0\ncontract = 1.0\n";
        let mut nbrs: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
        for x in 1..=3 {
            nbrs.entry(x).or_default().insert(9);
        }
        for &(a, b) in deals {
            nbrs.entry(a).or_default().insert(b);
            nbrs.entry(b).or_default().insert(a);
        }
        (toml::from_str(&toml).unwrap(), nbrs)
    }

    #[test]
    fn each_world_seed_draws_its_own_traffic() {
        assert_ne!(app_seed(1, 7), app_seed(2, 7), "two rounds, two seeds");
        assert_ne!(app_seed(1, 7), app_seed(1, 8), "two apps");
        assert_eq!(app_seed(5, 7), app_seed(5, 7));
        assert_eq!(app_seed(0, 7) & 1, 1, "odd, so xorshift never sticks at 0");
    }

    /// Prefers routes through `via`, announces everything to everyone.
    struct Via(u32);
    impl Policy for Via {
        fn rank(&mut self, input: &wire::RankInputW) -> Result<usize, String> {
            Ok(input.routes.iter().position(|r| r.neighbour == self.0).unwrap_or_else(|| wire::usual_rank(input)))
        }
        fn announce(&mut self, _: &wire::AnnounceInputW) -> Result<bool, String> {
            Ok(true)
        }
    }

    struct Broken;
    impl Policy for Broken {
        fn rank(&mut self, _: &wire::RankInputW) -> Result<usize, String> {
            Err("trap: unreachable".into())
        }
        fn announce(&mut self, _: &wire::AnnounceInputW) -> Result<bool, String> {
            Ok(true)
        }
    }

    fn run(
        deals: &[(u32, u32)],
        policies: Vec<(u32, Box<dyn Policy>)>,
    ) -> (BTreeMap<u32, BTreeMap<u32, wire::PolicyRouteW>>, Routing) {
        let (spec, nbrs) = world(deals);
        best_routes(&spec, &nbrs, &mut policies.into_iter().collect())
    }

    fn path(best: &BTreeMap<u32, BTreeMap<u32, wire::PolicyRouteW>>, x: u32, d: u32) -> Vec<u32> {
        best[&x][&d].as_path.clone()
    }

    #[test]
    fn the_usual_policy_takes_a_peer_over_the_tier1_and_passes_peer_routes_to_no_peer() {
        let (best, routing) = run(&[(1, 2), (1, 3)], vec![]);
        assert!(routing.settled && routing.failed.is_empty());
        assert_eq!(path(&best, 2, 1), vec![1]);
        assert_eq!(path(&best, 2, 3), vec![9, 3]); // 1 does not pass 3's route to 2
        let (same, _) = run(&[(1, 2), (1, 3)], vec![(1, Box::new(Usual)), (2, Box::new(Usual))]);
        assert_eq!(best, same);
    }

    #[test]
    fn rank_chooses_the_route() {
        // 2 hears 3 directly and, since 1 announces everything, through 1.
        let (best, _) = run(&[(1, 2), (1, 3), (2, 3)], vec![(1, Box::new(Via(3))), (2, Box::new(Via(1)))]);
        assert_eq!(path(&best, 2, 3), vec![1, 3]);
        let (usual, _) = run(&[(1, 2), (1, 3), (2, 3)], vec![(1, Box::new(Via(3)))]);
        assert_eq!(path(&usual, 2, 3), vec![3]);
    }

    #[test]
    fn announcing_a_peer_route_to_a_peer_is_a_leak_the_others_use() {
        let (best, _) = run(&[(1, 2), (1, 3)], vec![(1, Box::new(Via(0)))]);
        assert_eq!(path(&best, 2, 3), vec![1, 3]);
        assert_eq!(path(&best, 3, 2), vec![1, 2]);
    }

    #[test]
    fn policies_that_never_agree_stop_at_the_cap() {
        // 1 and 2 each prefer to reach 3 through the other.
        let (_, routing) = run(&[(1, 2), (1, 3), (2, 3)], vec![(1, Box::new(Via(2))), (2, Box::new(Via(1)))]);
        assert!(!routing.settled);
        assert_eq!(routing.passes, MAX_PASSES);
    }

    #[test]
    fn a_failing_policy_gets_the_usual_one_and_is_reported() {
        let (best, routing) = run(&[(1, 2), (1, 3)], vec![(2, Box::new(Broken))]);
        assert_eq!(routing.failed.get(&2).map(String::as_str), Some("trap: unreachable"));
        assert_eq!(best, run(&[(1, 2), (1, 3)], vec![]).0);
    }
}
