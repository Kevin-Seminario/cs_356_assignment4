//! The deal an agreement program carries (A4, settlement ABI 2): who signs,
//! the links it makes, its cap, its length, its numbers and its words. It
//! is the wasm custom section `na.deal`, written by
//! `switch_program_sdk::agreement_program!`: UTF-8, one item per line,
//! fields separated by tabs.
//!
//! ```text
//! abi    2
//! world  N                     1 to 4
//! party  CODE                  two to four, in party order
//! link   CODEX  CODEY  KIND    peer, transit or backup (X buys from Y)
//! cap    DOLLARS               whole dollars, the most one party pays a round
//! rounds N                     1 to 9
//! term   NAME  UNIT  VALUE     up to 16; VALUE as its author wrote it
//! reads  TEXT                  one to eight clauses, 200 characters each
//! ```

use serde::Serialize;

use super::{link_kind, Agreement};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Deal {
    pub abi: u32,
    pub world: u32,
    pub parties: Vec<String>,
    /// (code, code, kind).
    pub links: Vec<(String, String, String)>,
    pub cap: u64,
    pub rounds: u32,
    /// (name, unit, the value as written, the value in millionths).
    pub terms: Vec<(String, String, String, i64)>,
    pub reads: Vec<String>,
}

/// A wasm module's custom section of this name.
fn section<'a>(module: &'a [u8], name: &str) -> Option<&'a [u8]> {
    fn leb(bytes: &[u8], at: &mut usize) -> Option<usize> {
        let (mut n, mut shift) = (0usize, 0);
        loop {
            let b = *bytes.get(*at)?;
            *at += 1;
            n |= ((b & 0x7f) as usize).checked_shl(shift)?;
            if b & 0x80 == 0 {
                return Some(n);
            }
            shift += 7;
        }
    }
    let mut at = 8; // past the magic and the version
    while at < module.len() {
        let id = module[at];
        at += 1;
        let len = leb(module, &mut at)?;
        let body = module.get(at..at.checked_add(len)?)?;
        at += len;
        if id == 0 {
            let mut k = 0;
            let n = leb(body, &mut k)?;
            if body.get(k..k.checked_add(n)?)? == name.as_bytes() {
                return Some(&body[k + n..]);
            }
        }
    }
    None
}

/// A plain decimal ("0.30", "12") in millionths.
fn millionths(text: &str) -> Option<i64> {
    let (whole, part) = text.split_once('.').unwrap_or((text, ""));
    let digits = |s: &str| s.bytes().all(|b| b.is_ascii_digit());
    if whole.is_empty() || !digits(whole) || !digits(part) || part.len() > 6 || (part.is_empty() && text.contains('.')) {
        return None;
    }
    whole.parse::<i64>().ok()?.checked_mul(1_000_000)?.checked_add(format!("{part:0<6}").parse().ok()?)
}

impl Deal {
    /// The deal in an agreement program's file.
    pub fn read(module: &[u8]) -> Result<Deal, String> {
        let bytes = section(module, "na.deal").ok_or("no deal in this file (the section na.deal is missing)")?;
        Deal::parse(std::str::from_utf8(bytes).map_err(|_| "the deal is not text")?)
    }

    pub fn parse(text: &str) -> Result<Deal, String> {
        let mut deal = Deal { abi: 0, world: 0, parties: vec![], links: vec![], cap: 0, rounds: 0, terms: vec![], reads: vec![] };
        let mut seen = std::collections::BTreeSet::new();
        for line in text.lines() {
            let fields: Vec<&str> = line.split('\t').collect();
            let bad = || format!("a line of the deal is not right: {}", line.replace('\t', " | "));
            let once = matches!(fields[0], "abi" | "world" | "cap" | "rounds");
            if once && !seen.insert(fields[0]) {
                return Err(format!("the deal gives {} twice", fields[0]));
            }
            match fields[..] {
                ["abi", n] => deal.abi = n.parse().map_err(|_| bad())?,
                ["world", n] => deal.world = n.parse().map_err(|_| bad())?,
                ["party", code] => deal.parties.push(code.into()),
                ["link", x, y, kind] => deal.links.push((x.into(), y.into(), kind.into())),
                ["cap", n] => deal.cap = n.parse().map_err(|_| bad())?,
                ["rounds", n] => deal.rounds = n.parse().map_err(|_| bad())?,
                ["term", name, unit, value] => {
                    let m = millionths(value).ok_or(format!("the term {name} is not a plain number: {value}"))?;
                    deal.terms.push((name.into(), unit.into(), value.into(), m));
                }
                ["reads", words] => deal.reads.push(words.into()),
                _ => return Err(bad()),
            }
        }
        for key in ["abi", "world", "cap", "rounds"] {
            if !seen.contains(key) {
                return Err(format!("the deal gives no {key}"));
            }
        }
        let mut codes = deal.parties.clone();
        codes.sort();
        codes.dedup();
        let names: Vec<&String> = deal.terms.iter().map(|t| &t.0).collect();
        if deal.abi != 2 {
            return Err(format!("the deal is for ABI {}, not 2", deal.abi));
        } else if !(1..=4).contains(&deal.world) {
            return Err("world is 1 to 4".into());
        } else if !(2..=4).contains(&deal.parties.len()) || codes.len() != deal.parties.len() || codes.iter().any(|c| c.is_empty()) {
            return Err("the deal needs two to four different parties".into());
        } else if !(1..=9).contains(&deal.rounds) {
            return Err("rounds is 1 to 9".into());
        } else if deal.terms.len() > 16 || names.iter().enumerate().any(|(i, n)| n.is_empty() || names[..i].contains(n)) {
            return Err("the deal takes up to 16 terms, each with its own name".into());
        } else if !(1..=8).contains(&deal.reads.len()) || deal.reads.iter().any(|r| r.is_empty() || r.chars().count() > 200) {
            return Err("the deal needs one to eight lines of reads, of 200 characters at most".into());
        }
        for (x, y, kind) in &deal.links {
            if x == y || !deal.parties.contains(x) || !deal.parties.contains(y) {
                return Err(format!("the link {x}-{y} is not between two of its parties"));
            } else if link_kind(kind).is_none() {
                return Err(format!("unknown kind of link: {kind}"));
            }
        }
        Ok(deal)
    }

    /// The deal as a world file's agreement, between these ASes (one for
    /// each party, in order). The program and the state are the caller's.
    pub fn agreement(&self, parties: &[u32]) -> Agreement {
        let id = |code: &String| parties[self.parties.iter().position(|p| p == code).unwrap_or(0)];
        Agreement {
            parties: parties.to_vec(),
            links: self.links.iter().map(|(x, y, kind)| (id(x), id(y), kind.clone())).collect(),
            caps: vec![self.cap as f64; parties.len()],
            terms: self.terms.iter().map(|t| (t.0.clone(), t.3 as f64 / 1e6)).collect(),
            ..Default::default()
        }
    }

    /// The contract in plain text.
    pub fn show(&self) -> String {
        let mut s = format!("Agreement for world {}\n\nParties: {}\n\nLinks:\n", self.world, self.parties.join(", "));
        for (x, y, kind) in &self.links {
            s += &match kind.as_str() {
                "peer" => format!("  {x} and {y} peer\n"),
                "backup" => format!("  {x} buys backup transit from {y}\n"),
                _ => format!("  {x} buys transit from {y}\n"),
            };
        }
        if self.links.is_empty() {
            s += "  none\n";
        }
        s += "\nPayments:\n";
        for (i, words) in self.reads.iter().enumerate() {
            s += &format!("  {}. {words}\n", i + 1);
        }
        s += "\nNumbers:\n";
        for (name, unit, value, _) in &self.terms {
            s += &format!("  {name} = {value} {unit}\n");
        }
        if self.terms.is_empty() {
            s += "  none\n";
        }
        s + &format!("\nCap: no party pays more than ${} in a round\nRounds: {}, then it must be renewed\n", self.cap, self.rounds)
    }
}
