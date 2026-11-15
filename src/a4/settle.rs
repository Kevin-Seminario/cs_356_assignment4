//! Running an agreement's settlement program (A4).
//!
//! # ABI
//!
//! The module exports a linear `memory` and two functions:
//!
//! ```text
//! // A buffer of `len` bytes the host may write the input into.
//! settle_buffer(len: i32) -> i32
//!
//! // Read a postcard `SettleInputW` at ptr..ptr+len; return what AS `a`
//! // pays AS `b`, in millionths of a dollar (negative: `b` pays `a`).
//! settle(ptr: i32, len: i32) -> i64
//! ```
//!
//! A trap, running out of fuel or memory, or a missing export is a failed
//! settlement: the agreement pays nothing that round.
//!
//! # ABI 2
//!
//! A module that exports `settle2` is an agreement program: two to four
//! parties, a payment for each, and up to `MAX_STATE` bytes kept between
//! rounds.
//!
//! ```text
//! // Read a postcard `SettleInput2W` at ptr..ptr+len; return where the
//! // postcard `SettleOutputW` is: its address in the high 32 bits, its
//! // length in the low 32.
//! settle2(ptr: i32, len: i32) -> i64
//! ```
//!
//! The payments must be one per party and sum to zero. The deal it
//! carries is in the custom section `na.deal` (see `a4::deal`).

use switch_program_types as wire;
use wasmtime::{Instance, Memory, Store, TypedFunc};

use crate::detect::{start, trapped, Host};

#[derive(Debug, Clone)]
pub struct SettleLimits {
    /// Fuel for both calls together.
    pub fuel: u64,
    pub max_memory_bytes: usize,
}

impl Default for SettleLimits {
    fn default() -> Self {
        // A round's readings for one pair are kilobytes. Not yet measured
        // against real programs.
        Self { fuel: 50_000_000, max_memory_bytes: 32 * 1024 * 1024 }
    }
}

/// Run the settlement program in `module_bytes` on `input`.
pub fn run(module_bytes: &[u8], input: &wire::SettleInputW, limits: &SettleLimits) -> Result<i64, String> {
    let (mut store, instance, memory) = start(module_bytes, limits.fuel, limits.max_memory_bytes)?;
    run1(&mut store, &instance, &memory, input)
}

fn run1(store: &mut Store<Host>, instance: &Instance, memory: &Memory, input: &wire::SettleInputW) -> Result<i64, String> {
    let bytes = postcard::to_allocvec(input).map_err(|e| e.to_string())?;
    let buffer: TypedFunc<i32, i32> =
        instance.get_typed_func(&mut *store, "settle_buffer").map_err(|_| "missing export: settle_buffer")?;
    let settle: TypedFunc<(i32, i32), i64> =
        instance.get_typed_func(&mut *store, "settle").map_err(|_| "missing export: settle")?;
    let len = i32::try_from(bytes.len()).map_err(|_| "input too large")?;
    let ptr = buffer.call(&mut *store, len).map_err(|e| trapped(store, e))?;
    memory
        .write(&mut *store, ptr as u32 as usize, &bytes)
        .map_err(|_| format!("settle_buffer gave {ptr}, which does not hold {len} bytes"))?;
    settle.call(&mut *store, (ptr, len)).map_err(|e| trapped(store, e))
}

/// The most state an agreement program may keep between rounds, in bytes.
pub const MAX_STATE: usize = 4096;

/// 2 if the module exports `settle2`, else 1.
pub fn abi(module_bytes: &[u8]) -> Result<u32, String> {
    let module = wasmtime::Module::new(&wasmtime::Engine::default(), module_bytes).map_err(|e| format!("not a wasm module: {e}"))?;
    Ok(if module.get_export("settle2").is_some() { 2 } else { 1 })
}

/// Settle one agreement: what each party pays, in dollars (negative:
/// receives), and the state to hand back next round. `caps[i]` is the most
/// party `i` pays; payments over a cap are all scaled down by one factor.
/// A two-party (ABI 1) module settles only two parties and keeps the state
/// as it is. On `Err` nobody pays and the state stays what it was.
pub fn settle(
    module_bytes: &[u8],
    input: &wire::SettleInput2W,
    caps: &[f64],
    limits: &SettleLimits,
) -> Result<(Vec<f64>, Vec<u8>), String> {
    let n = input.parties.len();
    if caps.len() != n {
        return Err(format!("{} caps for {n} parties", caps.len()));
    }
    let (mut store, instance, memory) = start(module_bytes, limits.fuel, limits.max_memory_bytes)?;
    if instance.get_func(&mut store, "settle2").is_none() {
        if n != 2 {
            return Err(format!("a two-party program in an agreement of {n} parties"));
        }
        let old = wire::SettleInputW {
            a: input.parties[0],
            b: input.parties[1],
            terms: input.terms.clone(),
            epoch_ms: input.epoch_ms,
            round_ms: input.round_ms,
            readings: input.readings.clone(),
            outages: input.outages.clone(),
        };
        let amount = (run1(&mut store, &instance, &memory, &old)? as f64 / 1e6).min(caps[0]).max(-caps[1]);
        return Ok((vec![amount, 0.0 - amount], input.state.clone()));
    }
    let bytes = postcard::to_allocvec(input).map_err(|e| e.to_string())?;
    let buffer: TypedFunc<i32, i32> =
        instance.get_typed_func(&mut store, "settle_buffer").map_err(|_| "missing export: settle_buffer")?;
    let settle2: TypedFunc<(i32, i32), i64> =
        instance.get_typed_func(&mut store, "settle2").map_err(|_| "settle2 is not (i32, i32) -> i64")?;
    let len = i32::try_from(bytes.len()).map_err(|_| "input too large")?;
    let ptr = buffer.call(&mut store, len).map_err(|e| trapped(&store, e))?;
    memory
        .write(&mut store, ptr as u32 as usize, &bytes)
        .map_err(|_| format!("settle_buffer gave {ptr}, which does not hold {len} bytes"))?;
    let at = settle2.call(&mut store, (ptr, len)).map_err(|e| trapped(&store, e))?;
    let (ptr, len) = ((at >> 32) as u32 as usize, at as u32 as usize);
    let out: wire::SettleOutputW = memory
        .data(&store)
        .get(ptr..ptr.saturating_add(len))
        .and_then(|b| postcard::from_bytes(b).ok())
        .ok_or("settle2 did not return a SettleOutputW")?;
    if out.pay.len() != n {
        return Err(format!("{} payments for {n} parties", out.pay.len()));
    }
    if out.pay.iter().map(|&p| p as i128).sum::<i128>() != 0 {
        return Err("the payments do not sum to zero".into());
    }
    if out.state.len() > MAX_STATE {
        return Err(format!("{} bytes of state, over the {MAX_STATE} allowed", out.state.len()));
    }
    let mut pay: Vec<f64> = out.pay.iter().map(|&p| p as f64 / 1e6).collect();
    let f = pay.iter().zip(caps).filter(|(p, _)| **p > 0.0).map(|(p, cap)| cap / p).fold(1.0, f64::min);
    if f < 1.0 {
        pay.iter_mut().for_each(|p| *p = *p * f + 0.0); // + 0.0: no -0
    }
    Ok((pay, out.state))
}
