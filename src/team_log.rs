//! The team's log, in a run where the harness forwards: what the program
//! chose to write down, on the one switch it named for it. The detector
//! reads this and nothing else.
//!
//! A controller names its switch the logging switch with
//! `ControllerAction::NameLoggingSwitch`. The first switch to ask gets it
//! for the rest of the run. Every switch's first timer fires at time zero
//! in switch-id order, so among those asking then the lowest id wins. Only the logging switch's
//! `ControllerAction::LogWrite`s count, and each is kept whole or lost
//! whole: it is lost if it would take the log past its cap. The log is the
//! kept writes' bytes back to back, with nothing added. Framing and
//! timestamps are the program's to write.

use crate::types::SwitchId;

/// Why a write was not kept, as `LogEvent::TeamLogRefused` records it.
pub const REFUSED_NOT_LOGGING_SWITCH: u8 = 0;
pub const REFUSED_FULL: u8 = 1;

#[derive(Debug, Clone, Default)]
pub struct TeamLog {
    pub cap_bytes: usize,
    /// The logging switch, once one is named.
    pub switch: Option<SwitchId>,
    pub bytes: Vec<u8>,
}

impl TeamLog {
    pub fn new(cap_bytes: usize) -> Self {
        Self { cap_bytes, ..Default::default() }
    }

    /// `switch` asks to be the logging switch. True if it is, now or
    /// already.
    pub fn name(&mut self, switch: SwitchId) -> bool {
        *self.switch.get_or_insert(switch) == switch
    }

    /// `switch` writes `bytes`. `Ok` if kept, else why not.
    pub fn write(&mut self, switch: SwitchId, bytes: &[u8]) -> Result<(), u8> {
        if self.switch != Some(switch) {
            return Err(REFUSED_NOT_LOGGING_SWITCH);
        }
        if self.bytes.len() + bytes.len() > self.cap_bytes {
            return Err(REFUSED_FULL);
        }
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }
}

/// A4: one log per AS, with no logging switch. Any switch's write goes to
/// its AS's log as one line, `<t_ms> <switch> <message>`, the message
/// escaped byte by byte (see `escape`). A write that would take the AS's
/// message bytes past the cap is lost whole and counted.
#[derive(Debug, Clone, Default)]
pub struct AsLogs {
    pub cap_bytes: usize,
    pub owner: std::collections::HashMap<SwitchId, u32>,
    logs: std::collections::BTreeMap<u32, AsLog>,
}

#[derive(Debug, Clone, Default)]
struct AsLog {
    text: String,
    used: usize,
    dropped: u64,
}

impl AsLogs {
    pub fn new(cap_bytes: usize, owner: std::collections::HashMap<SwitchId, u32>) -> Self {
        Self { cap_bytes, owner, logs: Default::default() }
    }

    /// `switch` writes `bytes` at `t_ms`. True if kept.
    pub fn write(&mut self, t_ms: u64, switch: SwitchId, bytes: &[u8]) -> bool {
        let Some(&as_id) = self.owner.get(&switch) else { return false };
        let log = self.logs.entry(as_id).or_default();
        if log.used + bytes.len() > self.cap_bytes {
            log.dropped += 1;
            return false;
        }
        log.used += bytes.len();
        log.text += &format!("{t_ms} {} {}\n", switch.raw(), escape(bytes));
        true
    }

    /// Each AS that wrote, with its log's text.
    pub fn texts(&self) -> impl Iterator<Item = (u32, String)> + '_ {
        self.logs.iter().map(|(&id, l)| {
            let mut t = l.text.clone();
            if l.dropped > 0 {
                t += &format!("log full: {} later writes dropped\n", l.dropped);
            }
            (id, t)
        })
    }
}

/// Printable ASCII as it is, `\` as `\\`, newline, CR and tab as `\n`,
/// `\r`, `\t`, any other byte as `\xNN`. Lossless.
fn escape(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len());
    for &b in bytes {
        match b {
            b'\\' => s += "\\\\",
            b'\n' => s += "\\n",
            b'\r' => s += "\\r",
            b'\t' => s += "\\t",
            0x20..=0x7e => s.push(b as char),
            _ => s += &format!("\\x{b:02x}"),
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_first_switch_to_ask_logs_and_writes_are_whole_or_lost() {
        let (a, b) = (SwitchId::new(3), SwitchId::new(1));
        let mut log = TeamLog::new(10);
        assert_eq!(log.write(a, b"x"), Err(REFUSED_NOT_LOGGING_SWITCH), "nobody named yet");
        assert!(log.name(a));
        assert!(!log.name(b));
        assert!(log.name(a), "asking again changes nothing");
        assert_eq!(log.write(b, b"x"), Err(REFUSED_NOT_LOGGING_SWITCH));
        assert_eq!(log.write(a, b"123456"), Ok(()));
        assert_eq!(log.write(a, b"78901"), Err(REFUSED_FULL), "one byte over: all of it lost");
        assert_eq!(log.write(a, b"7890"), Ok(()), "a write that fits still goes in");
        assert_eq!(log.bytes, b"1234567890");
        assert_eq!(log.write(a, b""), Ok(()));
        assert_eq!(log.write(a, b"!"), Err(REFUSED_FULL));
    }

    #[test]
    fn each_as_gets_its_own_log_and_a_full_log_says_so() {
        let owner = [(SwitchId::new(11), 1), (SwitchId::new(12), 1), (SwitchId::new(21), 2)].into_iter().collect();
        let mut logs = AsLogs::new(8, owner);
        assert!(logs.write(5, SwitchId::new(11), b"a\nb"));
        assert!(logs.write(7, SwitchId::new(21), b"\\\xff"));
        assert!(!logs.write(9, SwitchId::new(12), b"123456"), "3 + 6 > 8: lost whole");
        assert!(logs.write(9, SwitchId::new(12), b"12345"), "3 + 5 fits");
        assert!(!logs.write(9, SwitchId::new(99), b"x"), "no AS");
        let t: Vec<_> = logs.texts().collect();
        assert_eq!(t[0], (1, "5 11 a\\nb\n9 12 12345\nlog full: 1 later writes dropped\n".to_string()));
        assert_eq!(t[1], (2, "7 21 \\\\\\xff\n".to_string()));
    }

    #[test]
    fn every_byte_round_trips() {
        let all: Vec<u8> = (0..=255).collect();
        let e = escape(&all);
        assert!(e.bytes().all(|b| (0x20..=0x7e).contains(&b)), "one printable line");
        assert!(e.starts_with("\\x00\\x01") && e.contains("\\t\\n\\x0b\\x0c\\r") && e.contains("[\\\\]"));
        assert!(e.contains("}~\\x7f\\x80") && e.ends_with("\\xfe\\xff"));
        // Decode: `\\`, `\n`, `\r`, `\t`, `\xNN`, else the byte.
        let (mut out, mut i, b) = (Vec::new(), 0, e.as_bytes());
        while i < b.len() {
            if b[i] != b'\\' { out.push(b[i]); i += 1; continue; }
            match b[i + 1] {
                b'n' => out.push(b'\n'), b'r' => out.push(b'\r'), b't' => out.push(b'\t'), b'\\' => out.push(b'\\'),
                b'x' => { out.push(u8::from_str_radix(&e[i + 2..i + 4], 16).unwrap()); i += 2; }
                c => panic!("unexpected escape {c}"),
            }
            i += 2;
        }
        assert_eq!(out, all);
    }
}
