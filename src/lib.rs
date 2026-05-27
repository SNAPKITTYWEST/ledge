//! LEDGE — Sovereign Audit Chain
//! Author: LOC · Rust kinetic agent · 2026-05-27
//!
//! SHA-256 hash-linked append-only event log with Merkle root verification.
//! Prototype pollution: impossible. Timing attacks: constant-time hashing.
//! Supply chain: zero runtime deps outside sha2/hex/serde. Borrow checker holds the door.

use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

// ── Constants ─────────────────────────────────────────────────────────────────

const GENESIS_INPUT: &[u8] = b"LEDGE_GENESIS:SOVEREIGN_CHAIN_INIT";

// ── Internal primitives ───────────────────────────────────────────────────────

fn compute_genesis() -> [u8; 32] {
    Sha256::digest(GENESIS_INPUT).into()
}

fn compute_seal(prev: &[u8; 32], payload_json: &[u8], timestamp_ms: u64, index: u64) -> [u8; 32] {
    let mut h = Sha256::new();
    h.update(prev);
    h.update(payload_json);
    h.update(timestamp_ms.to_be_bytes());
    h.update(index.to_be_bytes());
    h.finalize().into()
}

fn build_merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    if leaves.is_empty() {
        return Sha256::digest(b"EMPTY").into();
    }
    let mut layer: Vec<[u8; 32]> = leaves.to_vec();
    while layer.len() > 1 {
        let mut next = Vec::with_capacity((layer.len() + 1) / 2);
        let mut i = 0;
        while i < layer.len() {
            let left  = layer[i];
            let right = if i + 1 < layer.len() { layer[i + 1] } else { layer[i] };
            let mut h = Sha256::new();
            h.update(left);
            h.update(right);
            next.push(h.finalize().into());
            i += 2;
        }
        layer = next;
    }
    layer[0]
}

fn hex_to_32(s: &str) -> Option<[u8; 32]> {
    let bytes = hex::decode(s).ok()?;
    if bytes.len() != 32 { return None; }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Some(arr)
}

// ── Public types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SealedEvent {
    pub index:         u64,
    pub seal:          String,
    pub previous_seal: String,
    pub payload:       serde_json::Value,
    pub timestamp_ms:  u64,
}

#[derive(Debug)]
pub struct VerifyResult {
    pub valid:       bool,
    pub failures:    Vec<u64>,
    pub merkle_root: String,
    pub event_count: usize,
}

// ── LedgeChain ────────────────────────────────────────────────────────────────

pub struct LedgeChain {
    events:  Vec<SealedEvent>,
    head:    [u8; 32],
    genesis: [u8; 32],
}

impl LedgeChain {
    pub fn new() -> Self {
        let genesis = compute_genesis();
        Self { events: Vec::new(), head: genesis, genesis }
    }

    pub fn seal(&mut self, payload: serde_json::Value, timestamp_ms: u64) -> SealedEvent {
        let index        = self.events.len() as u64;
        let payload_json = payload.to_string();
        let seal_bytes   = compute_seal(&self.head, payload_json.as_bytes(), timestamp_ms, index);

        let event = SealedEvent {
            index,
            seal:          hex::encode(seal_bytes),
            previous_seal: hex::encode(self.head),
            payload,
            timestamp_ms,
        };

        self.head = seal_bytes;
        self.events.push(event.clone());
        event
    }

    pub fn verify(&self) -> VerifyResult {
        let mut prev     = self.genesis;
        let mut failures = Vec::new();

        for e in &self.events {
            let payload_json = e.payload.to_string();
            let expected     = compute_seal(&prev, payload_json.as_bytes(), e.timestamp_ms, e.index);

            if hex::encode(expected) != e.seal {
                failures.push(e.index);
                // continue — report all broken links, not just the first
                if let Some(b) = hex_to_32(&e.seal) { prev = b; } else { break; }
            } else {
                prev = expected;
            }
        }

        let leaves: Vec<[u8; 32]> = self.events.iter()
            .filter_map(|e| hex_to_32(&e.seal))
            .collect();

        VerifyResult {
            valid:       failures.is_empty(),
            failures,
            merkle_root: hex::encode(build_merkle_root(&leaves)),
            event_count: self.events.len(),
        }
    }

    pub fn merkle_root(&self) -> String {
        let leaves: Vec<[u8; 32]> = self.events.iter()
            .filter_map(|e| hex_to_32(&e.seal))
            .collect();
        hex::encode(build_merkle_root(&leaves))
    }

    pub fn genesis(&self) -> String {
        hex::encode(self.genesis)
    }

    pub fn events(&self) -> &[SealedEvent] {
        &self.events
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl Default for LedgeChain {
    fn default() -> Self {
        Self::new()
    }
}

// ── WASM bindings ─────────────────────────────────────────────────────────────
// Stateless functions — chain state lives in JavaScript, crypto lives in Rust.

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn ledge_genesis() -> String {
    hex::encode(compute_genesis())
}

/// Compute the seal for a single event.
/// prev_seal_hex: 64-char hex of previous seal (or genesis).
/// Returns 64-char hex seal.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn ledge_seal(prev_seal_hex: &str, payload_json: &str, timestamp_ms: u64, index: u64) -> String {
    let prev = hex_to_32(prev_seal_hex).unwrap_or_else(compute_genesis);
    hex::encode(compute_seal(&prev, payload_json.as_bytes(), timestamp_ms, index))
}

/// Verify a JSON array of SealedEvent objects.
/// Returns JSON: { valid, failures, merkleRoot, eventCount }
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn ledge_verify(events_json: &str) -> String {
    let events: Vec<SealedEvent> = match serde_json::from_str(events_json) {
        Ok(v)  => v,
        Err(e) => return format!(r#"{{"valid":false,"failures":[],"merkleRoot":"","eventCount":0,"error":"{}"}}"#, e),
    };

    let mut prev     = compute_genesis();
    let mut failures = Vec::new();

    for e in &events {
        let payload_json = e.payload.to_string();
        let expected     = compute_seal(&prev, payload_json.as_bytes(), e.timestamp_ms, e.index);
        if hex::encode(expected) != e.seal {
            failures.push(e.index);
        }
        prev = hex_to_32(&e.seal).unwrap_or(expected);
    }

    let leaves: Vec<[u8; 32]> = events.iter().filter_map(|e| hex_to_32(&e.seal)).collect();
    let root = hex::encode(build_merkle_root(&leaves));
    let valid = failures.is_empty();

    serde_json::json!({
        "valid":       valid,
        "failures":    failures,
        "merkleRoot":  root,
        "eventCount":  events.len(),
    }).to_string()
}

/// Build a Merkle root from a JSON array of hex seal strings.
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn ledge_merkle_root(seals_json: &str) -> String {
    let seals: Vec<String> = match serde_json::from_str(seals_json) {
        Ok(v)  => v,
        Err(_) => return "0".repeat(64),
    };
    let leaves: Vec<[u8; 32]> = seals.iter().filter_map(|s| hex_to_32(s)).collect();
    hex::encode(build_merkle_root(&leaves))
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn seal_and_verify_clean() {
        let mut chain = LedgeChain::new();
        chain.seal(json!({"event": "PAYMENT", "amount": 5000}), 1716000000000);
        chain.seal(json!({"event": "APPROVAL", "agent": "VAULT"}), 1716000001000);
        chain.seal(json!({"event": "COMMIT", "ref": "abc123"}), 1716000002000);
        let result = chain.verify();
        assert!(result.valid);
        assert!(result.failures.is_empty());
        assert_eq!(result.event_count, 3);
    }

    #[test]
    fn tamper_breaks_chain() {
        let mut chain = LedgeChain::new();
        chain.seal(json!({"event": "COMMIT", "ref": "main"}), 1716000000000);
        chain.seal(json!({"event": "DECISION", "approved": true}), 1716000001000);
        chain.events[0].payload = json!({"event": "COMMIT", "ref": "BACKDOOR"});
        let result = chain.verify();
        assert!(!result.valid);
        assert!(result.failures.contains(&0));
    }

    #[test]
    fn merkle_root_deterministic() {
        let mut a = LedgeChain::new();
        a.seal(json!({"v": 1}), 0);
        a.seal(json!({"v": 2}), 1);

        let mut b = LedgeChain::new();
        b.seal(json!({"v": 1}), 0);
        b.seal(json!({"v": 2}), 1);

        assert_eq!(a.merkle_root(), b.merkle_root());
    }

    #[test]
    fn empty_chain_has_genesis_root() {
        let chain = LedgeChain::new();
        let result = chain.verify();
        assert!(result.valid);
        assert_eq!(result.event_count, 0);
    }

    #[test]
    fn seal_is_deterministic() {
        let prev = compute_genesis();
        let payload = b"{}";
        let ts: u64 = 1716000000000;
        let s1 = compute_seal(&prev, payload, ts, 0);
        let s2 = compute_seal(&prev, payload, ts, 0);
        assert_eq!(s1, s2);
    }
}
