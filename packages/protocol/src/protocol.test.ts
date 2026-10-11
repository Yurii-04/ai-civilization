import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';
import {
  PROTOCOL_VERSION,
  Payload,
  Ping,
  ProtocolError,
  decodeEnvelope,
  encodePing,
  encodeSubscribe,
  readHeader,
  subscribeChannels,
} from './index.js';

describe('envelope', () => {
  it('round-trips a ping', () => {
    const bytes = encodePing(7, 42);
    expect(readHeader(bytes)).toEqual({ version: PROTOCOL_VERSION, kind: Payload.Ping, tick: 7 });
    const ping = decodeEnvelope(bytes).payload(new Ping()) as Ping;
    expect(ping.nonce()).toBe(42);
  });

  it('round-trips a subscribe', () => {
    const env = decodeEnvelope(encodeSubscribe(['strategic', 'island:vesper']));
    expect(subscribeChannels(env)).toEqual(['strategic', 'island:vesper']);
  });

  it('rejects garbage', () => {
    expect(() => readHeader(new Uint8Array())).toThrow(ProtocolError);
    expect(() => readHeader(new TextEncoder().encode('definitely not flatbuffers'))).toThrow(
      ProtocolError,
    );
  });

  it('decodes the Rust-encoded fixture', () => {
    // Written by `UPDATE_FIXTURES=1 cargo test -p protocol`.
    const fixture = readFileSync(new URL('../../../schemas/fixtures/ping.bin', import.meta.url));
    const bytes = new Uint8Array(fixture);
    expect(readHeader(bytes)).toEqual({ version: PROTOCOL_VERSION, kind: Payload.Ping, tick: 7 });
    expect((decodeEnvelope(bytes).payload(new Ping()) as Ping).nonce()).toBe(42);
  });
});
