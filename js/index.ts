// @snapkitty/ledge — JavaScript/TypeScript interface
// Crypto: Rust (ledge-open/src/lib.rs) compiled to WASM via wasm-pack
// This file is a typed wrapper. Zero crypto runs here.
//
// Build WASM first:
//   cd ledge-open && wasm-pack build --features wasm --target bundler --out-dir pkg

export interface SealedEvent {
  index:        number
  seal:         string
  previousSeal: string
  payload:      Record<string, unknown>
  timestampMs:  number
}

export interface VerifyResult {
  valid:       boolean
  failures:    number[]
  merkleRoot:  string
  eventCount:  number
  error?:      string
}

// ── WASM module interface ─────────────────────────────────────────────────────

interface LedgeWasm {
  ledge_genesis():                                              string
  ledge_seal(prev: string, payload: string, ts: bigint, idx: bigint): string
  ledge_verify(events_json: string):                            string
  ledge_merkle_root(seals_json: string):                        string
}

// ── In-memory chain (state lives in JS, crypto lives in Rust) ─────────────────

export class LedgeChain {
  private events: SealedEvent[] = []
  private head:   string
  private genesis: string
  private wasm:   LedgeWasm

  constructor(wasm: LedgeWasm) {
    this.wasm    = wasm
    this.genesis = wasm.ledge_genesis()
    this.head    = this.genesis
  }

  seal(payload: Record<string, unknown>, timestampMs = Date.now()): SealedEvent {
    const index       = this.events.length
    const payloadJson = JSON.stringify(payload)
    const seal        = this.wasm.ledge_seal(
      this.head,
      payloadJson,
      BigInt(timestampMs),
      BigInt(index),
    )

    const event: SealedEvent = {
      index,
      seal,
      previousSeal: this.head,
      payload,
      timestampMs,
    }

    this.head = seal
    this.events.push(event)
    return event
  }

  verify(): VerifyResult {
    const result = JSON.parse(this.wasm.ledge_verify(JSON.stringify(this.events)))
    return result as VerifyResult
  }

  merkleRoot(): string {
    const seals = this.events.map(e => e.seal)
    return this.wasm.ledge_merkle_root(JSON.stringify(seals))
  }

  getEvents(): SealedEvent[] {
    return [...this.events]
  }

  getGenesis(): string {
    return this.genesis
  }
}

// ── Factory ────────────────────────────────────────────────────────────────────

/**
 * Load the WASM module and return a factory for creating chains.
 *
 * Browser / bundler:
 *   import { initLedge } from '@snapkitty/ledge'
 *   const { createChain } = await initLedge()
 *   const chain = createChain()
 *
 * Node.js (after wasm-pack --target nodejs):
 *   const wasm = require('../pkg/ledge')
 *   const { initLedge } = require('@snapkitty/ledge')
 *   const { createChain } = await initLedge(wasm)
 */
export async function initLedge(wasmModule?: LedgeWasm): Promise<{
  createChain: () => LedgeChain
  genesis:     () => string
}> {
  let wasm: LedgeWasm

  if (wasmModule) {
    wasm = wasmModule
  } else {
    // Dynamic import — bundler resolves to pkg/ledge_bg.wasm
    const mod = await import('../pkg/ledge.js' as string) as LedgeWasm & { default?: () => Promise<void> }
    if (mod.default) await mod.default()
    wasm = mod
  }

  return {
    createChain: () => new LedgeChain(wasm),
    genesis:     () => wasm.ledge_genesis(),
  }
}
