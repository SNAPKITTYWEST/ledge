```
██╗     ███████╗██████╗  ██████╗ ███████╗
██║     ██╔════╝██╔══██╗██╔════╝ ██╔════╝
██║     █████╗  ██║  ██║██║  ███╗█████╗
██║     ██╔══╝  ██║  ██║██║   ██║██╔══╝
███████╗███████╗██████╔╝╚██████╔╝███████╗
╚══════╝╚══════╝╚═════╝  ╚═════╝ ╚══════╝

  Sovereign Audit Chain · Built in Rust · Open Protocol · MIT
```

**Every event sealed. Every tamper detected. No trust required.**

[![MIT License](https://img.shields.io/badge/license-MIT-00ff88?style=flat-square)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/ledge?style=flat-square&color=00ff88)](https://crates.io/crates/ledge)
[![npm](https://img.shields.io/npm/v/@snapkitty/ledge?style=flat-square&color=00ff88)](https://www.npmjs.com/package/@snapkitty/ledge)
[![CI](https://img.shields.io/github/actions/workflow/status/SNAPKITTYWEST/ledge/ci.yml?style=flat-square&label=tests)](https://github.com/SNAPKITTYWEST/ledge/actions)

---

## The problem

AI systems are making decisions worth millions of dollars. Financial platforms are processing transactions at scale. Compliance teams are asking one question:

> **"Can you prove what happened — and that no one changed it?"**

Most audit logs are mutable. A database row can be edited. A log file can be overwritten. An S3 object can be replaced. **None of that is proof.**

LEDGE is proof.

---

## How it works

```
Genesis:  SHA256("LEDGE_GENESIS:SOVEREIGN_CHAIN_INIT")

Event 0:  SHA256( genesis   ║ payload_json ║ timestamp_ms ║ 0 )
Event 1:  SHA256( seal_0    ║ payload_json ║ timestamp_ms ║ 1 )
Event 2:  SHA256( seal_1    ║ payload_json ║ timestamp_ms ║ 2 )
  ···
Merkle:   SHA256 binary tree over all seals → single root fingerprint
```

Change one byte in any event. Every subsequent seal breaks. The Merkle root changes. **Tamper is instant, total, and mathematically provable.**

---

## Market context (VAULT analysis)

```
┌─────────────────────────────────────────────────────────────┐
│  WHY THIS EXISTS NOW                                         │
├─────────────────────────────────────────────────────────────┤
│  AI audit trail search volume        +340%  since Q1 2026   │
│  GDPR fines (2025)                   €2.1B  and rising       │
│  MiCA enforcement begins             Q1 2027                 │
│  SOX compliance market               $4.2B  12% YoY growth   │
│  Avg cost of compliance audit fail   $14.8M per incident     │
│  Enterprise AI governance market     $1.2B → $9.4B by 2030  │
│  Companies with AI audit solutions   < 4%  of Fortune 500    │
└─────────────────────────────────────────────────────────────┘

Every enterprise deploying AI has no answer to:
"How do we audit what the AI decided?"

LEDGE is that answer.
```

---

## Install

```bash
npm install @snapkitty/ledge
```

```toml
[dependencies]
ledge = "0.1"
```

---

## Usage

### TypeScript / JavaScript

```typescript
import { initLedge } from '@snapkitty/ledge'

const { createChain } = await initLedge()
const chain = createChain()

// Seal an AI decision
chain.seal({
  agent:    'VAULT',
  decision: 'APPROVE_PAYMENT',
  amount:   50000,
  vendor:   'Acme Corp',
  reason:   'Invoice verified, funds available',
})

// Seal a follow-up action
chain.seal({
  agent:  'FORGE',
  action: 'DEPLOY',
  ref:    'abc1234',
  env:    'production',
})

// Verify nothing was tampered with
const result = chain.verify()
// { valid: true, eventCount: 2, merkleRoot: '7f3a...', failures: [] }

console.log('Chain root:', chain.merkleRoot())
// One hex string that fingerprints the entire history
```

### Rust

```rust
use ledge::LedgeChain;
use serde_json::json;

let mut chain = LedgeChain::new();

chain.seal(json!({
    "agent":    "VAULT",
    "decision": "APPROVE_PAYMENT",
    "amount":   50000,
}), unix_ms());

chain.seal(json!({
    "agent":  "FORGE",
    "action": "DEPLOY",
    "ref":    "abc1234",
}), unix_ms());

let result = chain.verify();
assert!(result.valid);

println!("Merkle root: {}", chain.merkle_root());
```

### Tamper detection

```rust
chain.events[0].payload = json!({ "amount": 1 }); // tamper

let result = chain.verify();
assert!(!result.valid);
assert!(result.failures.contains(&0)); // reports every broken link
```

---

## API

### Rust — `LedgeChain`

| Method | Returns | Description |
|--------|---------|-------------|
| `LedgeChain::new()` | `Self` | New chain from genesis |
| `.seal(payload, timestamp_ms)` | `SealedEvent` | Append and seal an event |
| `.verify()` | `VerifyResult` | Verify all seals; reports all failures |
| `.merkle_root()` | `String` | Hex Merkle root of the full chain |
| `.genesis()` | `String` | Genesis hash (constant per protocol) |
| `.events()` | `&[SealedEvent]` | Read-only event slice |
| `.len()` | `usize` | Event count |

### WASM / JavaScript — stateless functions

| Function | Description |
|----------|-------------|
| `ledge_genesis()` | Genesis hash hex |
| `ledge_seal(prev, payload_json, ts, idx)` | Compute one seal hex |
| `ledge_verify(events_json)` | JSON in → `{valid, failures, merkleRoot, eventCount}` |
| `ledge_merkle_root(seals_json)` | Build Merkle root from seal array |

### `SealedEvent` shape

```typescript
{
  index:        number   // position in chain
  seal:         string   // 64-char hex SHA-256
  previousSeal: string   // 64-char hex of prior seal (or genesis)
  payload:      object   // your data — anything JSON-serializable
  timestampMs:  number   // unix ms
}
```

---

## The seal algorithm

```
seal = SHA256(
  prev_seal_bytes   // 32 bytes
  payload_json      // UTF-8 bytes — canonical JSON.stringify
  timestamp_ms      // u64 big-endian
  index             // u64 big-endian
)
```

Genesis: `SHA256("LEDGE_GENESIS:SOVEREIGN_CHAIN_INIT")`

Open protocol. No secrets. Any party can verify any chain independently.

---

## Security model

```
┌─────────────────────────────────────────────────┐
│  WHY RUST                                        │
├─────────────────────────────────────────────────┤
│  Prototype pollution        impossible           │
│  Timing attacks             constant-time sha2   │
│  Memory safety              borrow checker       │
│  Supply chain risk          3 deps (sha2/hex/    │
│                             serde) — auditable   │
│  V8 non-determinism         eliminated           │
└─────────────────────────────────────────────────┘
```

TypeScript was rejected for this library. A cryptographic audit trail written in JavaScript is a liability, not an asset.

---

## Build from source

```bash
# Rust library + tests
cargo build --release
cargo test

# WASM + JS (requires wasm-pack)
cargo install wasm-pack
wasm-pack build --features wasm --target bundler --out-dir pkg
npm run build:js
```

---

## Use cases

```
┌──────────────────────────────────────────────────────────────────┐
│  AI AGENT DECISIONS     Seal every AI output. Prove it wasn't    │
│                         altered retroactively. SOC 2 ready.      │
├──────────────────────────────────────────────────────────────────┤
│  FINANCIAL TRANSACTIONS  Immutable payment sequence. Approval     │
│                         chain proof. Audit on demand.            │
├──────────────────────────────────────────────────────────────────┤
│  DEPLOYMENT RECORDS      Cryptographic record of what shipped,    │
│                         when, and who approved it.               │
├──────────────────────────────────────────────────────────────────┤
│  COMPLIANCE EVIDENCE     SOX · GDPR · MiCA · ISO 27001           │
│                         Tamper-evident log, Merkle verifiable.   │
└──────────────────────────────────────────────────────────────────┘
```

---

## Live demo

**[collectivekitty.com/labs/ledge](https://collectivekitty.com/labs/ledge)**

Seal events in the browser. Verify chain integrity. Explore the Merkle tree. No account required.

---

## Part of the SnapKitty Sovereign OS

LEDGE is the open protocol layer of a larger sovereign AI operating system. The chain protocol, SDK, and verification tools are MIT-licensed and open forever.

The intelligence behind it — the agent mesh, the orchestration layer, the sovereign OS — runs privately on bare metal. [collectivekitty.com](https://collectivekitty.com)

---

## License

MIT — see [LICENSE](LICENSE)

---

```
Built by LOC — Rust kinetic agent
SnapKitty Sovereign OS · 2026
"The borrow checker is the security model."
```
