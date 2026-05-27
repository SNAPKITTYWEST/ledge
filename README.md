# LEDGE — Sovereign Audit Chain

**Cryptographic append-only event ledger for AI agents and financial systems.**

[![MIT License](https://img.shields.io/badge/license-MIT-00ff88)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/ledge)](https://crates.io/crates/ledge)
[![npm](https://img.shields.io/npm/v/@snapkitty/ledge)](https://www.npmjs.com/package/@snapkitty/ledge)

---

## What it does

Every event you seal is SHA-256 chained to the previous. If anyone tampers with any event — one byte — every subsequent hash breaks. Tamper-evident. Mathematically provable. No trust required.

```
Event 0: SHA256(genesis || payload || timestamp || 0)
Event 1: SHA256(seal_0  || payload || timestamp || 1)
Event 2: SHA256(seal_1  || payload || timestamp || 2)
         ...
Merkle root fingerprints the entire chain.
```

**The security model:** Rust borrow checker. Zero JavaScript crypto.

---

## Install

```bash
npm install @snapkitty/ledge
```

```toml
# Cargo.toml
[dependencies]
ledge = "0.1"
```

---

## Usage

### JavaScript / TypeScript

```typescript
import { initLedge } from '@snapkitty/ledge'

const { createChain } = await initLedge()
const chain = createChain()

chain.seal({ event: 'PAYMENT', amount: 5000, vendor: 'Acme Corp' })
chain.seal({ event: 'APPROVAL', agent: 'VAULT', approved: true })
chain.seal({ event: 'COMMIT', ref: 'abc123', author: 'forge' })

const result = chain.verify()
// { valid: true, eventCount: 3, merkleRoot: '7f3a...', failures: [] }

console.log(chain.merkleRoot()) // 64-char hex fingerprint of the entire chain
```

### Rust

```rust
use ledge::LedgeChain;
use serde_json::json;

let mut chain = LedgeChain::new();

chain.seal(json!({ "event": "PAYMENT", "amount": 5000 }), 1716000000000);
chain.seal(json!({ "event": "APPROVAL", "agent": "VAULT" }), 1716000001000);

let result = chain.verify();
assert!(result.valid);
println!("Merkle root: {}", chain.merkle_root());
```

---

## API

### Rust

| Method | Description |
|--------|-------------|
| `LedgeChain::new()` | Create a new chain seeded from genesis |
| `.seal(payload, timestamp_ms)` → `SealedEvent` | Append a sealed event |
| `.verify()` → `VerifyResult` | Verify all seals; returns all broken links |
| `.merkle_root()` → `String` | Hex Merkle root of the full chain |
| `.genesis()` → `String` | Hex genesis hash (constant per protocol) |
| `.events()` → `&[SealedEvent]` | Read-only slice of all events |

### WASM / JavaScript

| Function | Description |
|----------|-------------|
| `ledge_genesis()` | Return genesis hash hex |
| `ledge_seal(prev, payload_json, timestamp_ms, index)` | Compute one seal |
| `ledge_verify(events_json)` | Verify JSON array of events |
| `ledge_merkle_root(seals_json)` | Build Merkle root from seal array |

---

## How the seal works

```
seal = SHA256(
  prev_seal_bytes  ||  // 32 bytes — previous seal (or genesis)
  payload_json     ||  // UTF-8 JSON string
  timestamp_ms     ||  // u64 big-endian — milliseconds since epoch
  index            ||  // u64 big-endian — event position in chain
)
```

Genesis: `SHA256("LEDGE_GENESIS:SOVEREIGN_CHAIN_INIT")`

The protocol has no secrets. Any party with the events can verify the chain independently.

---

## Build from source

**Rust library:**
```bash
cargo build --release
cargo test
```

**WASM + JavaScript:**
```bash
# requires wasm-pack: cargo install wasm-pack
wasm-pack build --features wasm --target bundler --out-dir pkg
npm run build:js
```

---

## Use cases

- **AI agent audit trails** — seal every decision an AI makes; prove it wasn't altered retroactively
- **Financial transaction logs** — cryptographic proof of payment sequence and approval chain
- **Code deployment records** — immutable record of what was deployed, when, and who approved
- **Compliance evidence** — SOX, GDPR, MiCA all require tamper-evident audit trails

---

## Live demo

[collectivekitty.com/labs/ledge](https://collectivekitty.com/labs/ledge) — seal events in the browser, verify chain integrity, explore the Merkle tree.

---

## License

MIT — see [LICENSE](LICENSE)

---

*Built by LOC — Rust kinetic agent, SnapKitty Sovereign OS*  
*Core infrastructure: [github.com/snapkittywest/DEVFLOW-FINANCE](https://github.com/SNAPKITTYWEST/DEVFLOW-FINANCE) (private)*
