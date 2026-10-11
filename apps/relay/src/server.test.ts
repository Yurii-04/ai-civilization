import { Payload, Pong, decodeEnvelope, encodePing, readHeader } from '@relight/protocol';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import { startRelay, type Relay } from './server.js';

let relay: Relay;

beforeAll(async () => {
  relay = await startRelay({ port: 0, host: '127.0.0.1' });
});

afterAll(() => relay.close());

describe('relay', () => {
  it('serves /health', async () => {
    const res = await fetch(`http://127.0.0.1:${relay.port}/health`);
    expect(res.status).toBe(200);
    expect(await res.json()).toMatchObject({ status: 'ok' });
  });

  it('answers a Ping with a Pong carrying the same nonce', async () => {
    const ws = new WebSocket(`ws://127.0.0.1:${relay.port}/ws`);
    ws.binaryType = 'arraybuffer';
    const reply = await new Promise<Uint8Array>((resolve, reject) => {
      ws.onopen = () => ws.send(encodePing(0, 99));
      ws.onmessage = (e) => resolve(new Uint8Array(e.data as ArrayBuffer));
      ws.onerror = () => reject(new Error('websocket error'));
    });
    ws.close();
    expect(readHeader(reply).kind).toBe(Payload.Pong);
    expect((decodeEnvelope(reply).payload(new Pong()) as Pong).nonce()).toBe(99);
  });
});
