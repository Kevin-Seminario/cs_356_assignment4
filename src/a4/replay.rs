//! Each AS's own replay, cut from the whole world's: its switches, links,
//! apps and their events in full, and of other ASes only what crossed its
//! border links (the link, its failures, packets sent onto it, and the far
//! switch so a viewer can draw it).

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read;
use std::path::Path;

use crate::sim_log::writer::ReadError;
use crate::sim_log::{EventLogger, LogEvent, NodeRefW};
use crate::world::World;

/// Split the whole-world log `whole` into `dir/as-N.simlog` for every AS
/// of `world`, the Tier-1 included.
pub fn split(world: &World, whole: &Path, dir: &Path) -> Result<(), ReadError> {
    let spec = world.a4.as_ref().expect("an A4 world");
    let switch_as: BTreeMap<u32, u32> = world.switches.iter().map(|s| (s.id, s.as_id)).collect();
    let app_as: BTreeMap<u32, u32> = world.apps.iter().map(|a| (a.id, switch_as[&a.switch])).collect();
    // The ASes that see each switch: its own, and those it is a far end for.
    let mut seen_by: BTreeMap<u32, BTreeSet<u32>> = switch_as.iter().map(|(&s, &a)| (s, BTreeSet::from([a]))).collect();
    for l in &world.links {
        let (x, y) = (switch_as[&l.a], switch_as[&l.b]);
        seen_by.get_mut(&l.a).unwrap().insert(y);
        seen_by.get_mut(&l.b).unwrap().insert(x);
    }
    let version = header_version(whole)?;
    let mut out: BTreeMap<u32, EventLogger> = BTreeMap::new();
    for n in spec.ases.iter().map(|a| a.id).chain([spec.tier1_as]) {
        out.insert(n, EventLogger::create_version(&dir.join(format!("as-{n}.simlog")), version)?);
    }
    let node_as = |r: &NodeRefW| match r {
        NodeRefW::Switch(s) => switch_as.get(s).copied(),
        NodeRefW::App(a) => app_as.get(a).copied(),
    };
    let mut link_as: BTreeMap<u32, [Option<u32>; 2]> = BTreeMap::new();
    for_each(whole, |at_ns, event| {
        let one = |s: &u32| switch_as.get(s).copied();
        let to: Vec<u32> = match &event {
            LogEvent::Header { .. } | LogEvent::BgpConfigured { .. } | LogEvent::CustomerNamed { .. } => vec![],
            LogEvent::SwitchAdded { id, .. } => seen_by.get(id).into_iter().flatten().copied().collect(),
            LogEvent::AppAdded { id, .. } | LogEvent::PacketDelivered { app: id, .. } => {
                app_as.get(id).copied().into_iter().collect()
            }
            LogEvent::LinkAdded { id, a, b, .. } => {
                let ends = [node_as(a), node_as(b)];
                link_as.insert(*id, ends);
                ends.into_iter().flatten().collect()
            }
            LogEvent::LinkFailed { id } | LogEvent::LinkRestored { id } => {
                link_as.get(id).into_iter().flatten().flatten().copied().collect()
            }
            LogEvent::PacketEgress { switch, link, .. } => {
                one(switch).into_iter().chain(link_as.get(link).into_iter().flatten().flatten().copied()).collect()
            }
            LogEvent::PacketDropped { at, .. } => node_as(at).into_iter().collect(),
            LogEvent::SwitchFailed { id } | LogEvent::CpuFailed { id } => one(id).into_iter().collect(),
            LogEvent::PacketIngress { switch, .. }
            | LogEvent::PacketPunted { switch, .. }
            | LogEvent::ProgramInstalled { switch, .. }
            | LogEvent::TableEntryInstalled { switch, .. }
            | LogEvent::TableEntryDeleted { switch, .. }
            | LogEvent::QueueConfigChanged { switch, .. }
            | LogEvent::PortQueuesConfigured { switch, .. }
            | LogEvent::LinkDeclared { switch, .. }
            | LogEvent::ObservationDropped { switch, .. }
            | LogEvent::PacketSampled { switch, .. }
            | LogEvent::LoggingSwitchNamed { switch, .. }
            | LogEvent::TeamLogWritten { switch, .. }
            | LogEvent::TeamLogRefused { switch, .. }
            | LogEvent::DroppedAtPort { switch, .. } => one(switch).into_iter().collect(),
        };
        for n in to.into_iter().collect::<BTreeSet<u32>>() {
            if let Some(log) = out.get_mut(&n) {
                log.log(at_ns, event.clone());
            }
        }
    })
}

fn for_each(path: &Path, mut f: impl FnMut(u64, LogEvent)) -> Result<(), ReadError> {
    crate::sim_log::writer::for_each_frame(path, |fr| f(fr.at_ns, fr.event))
}

/// The version the log's header declares.
fn header_version(path: &Path) -> Result<u32, ReadError> {
    let mut file = std::fs::File::open(path)?;
    let mut len = [0u8; 4];
    file.read_exact(&mut len)?;
    let mut body = vec![0u8; u32::from_le_bytes(len) as usize];
    file.read_exact(&mut body)?;
    match postcard::from_bytes::<crate::sim_log::LogFrame>(&body) {
        Ok(fr) if crate::sim_log::writer::is_header(&fr.event) => match fr.event {
            LogEvent::Header { version, .. } => Ok(version),
            _ => unreachable!(),
        },
        _ => Err(ReadError::BadHeader),
    }
}
