'use client';

import {
  Payload,
  Pong,
  decodeEnvelope,
  encodePing,
  isSupportedVersion,
  readHeader,
} from '@relight/protocol';
import { MonoLabel } from '@relight/ui';
import { useEffect, useState } from 'react';

const RELAY_URL = process.env.NEXT_PUBLIC_RELAY_URL ?? 'ws://localhost:9001/ws';
const PING_INTERVAL_MS = 2_000;

type Status = { kind: 'connecting' } | { kind: 'offline' } | { kind: 'online'; rttMs: number };

/** Pings the relay over the FlatBuffers envelope and shows the round trip. */
export function RelayStatus() {
  const [status, setStatus] = useState<Status>({ kind: 'connecting' });

  useEffect(() => {
    const ws = new WebSocket(RELAY_URL);
    ws.binaryType = 'arraybuffer';
    const sent = new Map<number, number>();
    let nonce = 0;
    let timer: ReturnType<typeof setInterval> | undefined;

    const ping = () => {
      nonce = (nonce + 1) >>> 0;
      sent.set(nonce, performance.now());
      ws.send(encodePing(0, nonce));
    };

    ws.onopen = () => {
      ping();
      timer = setInterval(ping, PING_INTERVAL_MS);
    };
    ws.onmessage = (event: MessageEvent<ArrayBuffer>) => {
      const bytes = new Uint8Array(event.data);
      try {
        const header = readHeader(bytes);
        if (!isSupportedVersion(header.version) || header.kind !== Payload.Pong) return;
        const pong = decodeEnvelope(bytes).payload(new Pong()) as Pong;
        const start = sent.get(pong.nonce());
        if (start === undefined) return;
        sent.delete(pong.nonce());
        setStatus({ kind: 'online', rttMs: Math.round(performance.now() - start) });
      } catch {
        // Ignore malformed frames.
      }
    };
    ws.onclose = () => setStatus({ kind: 'offline' });

    return () => {
      clearInterval(timer);
      ws.close();
    };
  }, []);

  const text =
    status.kind === 'online'
      ? `Relay · ${status.rttMs} ms`
      : status.kind === 'offline'
        ? 'Relay · offline'
        : 'Relay · connecting';
  return <MonoLabel tone={status.kind === 'online' ? 'secondary' : 'timestamp'}>{text}</MonoLabel>;
}
