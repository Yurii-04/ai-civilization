/**
 * Relight wire protocol (TypeScript side).
 *
 * `generated/` is produced by `pnpm codegen` from `schemas/*.fbs`; never edit
 * it by hand. Keep PROTOCOL_VERSION in sync with `crates/protocol`.
 */
import * as flatbuffers from 'flatbuffers';
import { Envelope } from './generated/relight/protocol/envelope.js';
import { Payload } from './generated/relight/protocol/payload.js';
import { Ping } from './generated/relight/protocol/ping.js';
import { Pong } from './generated/relight/protocol/pong.js';
import { Subscribe } from './generated/relight/protocol/subscribe.js';

export { Envelope, Payload, Ping, Pong, Subscribe };

/** Bump on every schema change. Clients must tolerate `PROTOCOL_VERSION - 1`. */
export const PROTOCOL_VERSION = 1;

/** Oldest envelope version this build can read. */
export const MIN_SUPPORTED_VERSION = Math.max(1, PROTOCOL_VERSION - 1);

export interface Header {
  version: number;
  kind: Payload;
  tick: number;
}

export class ProtocolError extends Error {
  override name = 'ProtocolError';
}

/** Returns the envelope root after checking the `RLGT` file identifier. */
export function decodeEnvelope(bytes: Uint8Array): Envelope {
  if (bytes.byteLength < 8) throw new ProtocolError('buffer too short');
  const bb = new flatbuffers.ByteBuffer(bytes);
  if (!Envelope.bufferHasIdentifier(bb)) {
    throw new ProtocolError('not a Relight envelope (missing file identifier)');
  }
  return Envelope.getRootAsEnvelope(bb);
}

/** Reads only the routing fields; used by relays. */
export function readHeader(bytes: Uint8Array): Header {
  const env = decodeEnvelope(bytes);
  return { version: env.version(), kind: env.payloadType(), tick: env.tick() };
}

export function isSupportedVersion(version: number): boolean {
  return version >= MIN_SUPPORTED_VERSION && version <= PROTOCOL_VERSION;
}

function finish(
  builder: flatbuffers.Builder,
  tick: number,
  kind: Payload,
  payload: flatbuffers.Offset,
): Uint8Array<ArrayBuffer> {
  const env = Envelope.createEnvelope(builder, PROTOCOL_VERSION, tick, kind, payload);
  Envelope.finishEnvelopeBuffer(builder, env);
  // Copy out of the builder's oversized backing buffer.
  return builder.asUint8Array().slice();
}

export function encodePing(tick: number, nonce: number): Uint8Array<ArrayBuffer> {
  const b = new flatbuffers.Builder(64);
  return finish(b, tick, Payload.Ping, Ping.createPing(b, nonce));
}

export function encodePong(tick: number, nonce: number): Uint8Array<ArrayBuffer> {
  const b = new flatbuffers.Builder(64);
  return finish(b, tick, Payload.Pong, Pong.createPong(b, nonce));
}

export function encodeSubscribe(channels: readonly string[]): Uint8Array<ArrayBuffer> {
  const b = new flatbuffers.Builder(128);
  const names = channels.map((c) => b.createString(c));
  const vec = Subscribe.createChannelsVector(b, names);
  return finish(b, 0, Payload.Subscribe, Subscribe.createSubscribe(b, vec));
}

/** Channel names from a Subscribe envelope. */
export function subscribeChannels(env: Envelope): string[] {
  const sub = env.payload(new Subscribe()) as Subscribe | null;
  if (!sub) return [];
  const out: string[] = [];
  for (let i = 0; i < sub.channelsLength(); i++) {
    const name = sub.channels(i);
    if (name !== null) out.push(name);
  }
  return out;
}
