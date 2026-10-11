import {
  Payload,
  Ping,
  decodeEnvelope,
  encodePong,
  isSupportedVersion,
  readHeader,
  subscribeChannels,
} from '@relight/protocol';
import { createRequire } from 'node:module';
import type * as UWS from 'uWebSockets.js';

// Load the CommonJS entry: the package's ESM wrapper is broken in some
// releases (it imports a missing ./index.js).
const uWS = createRequire(import.meta.url)('uWebSockets.js') as typeof UWS;

/** Per-connection state. Channel routing from Redis arrives in M3. */
export interface ClientData {
  channels: string[];
}

export interface RelayOptions {
  /** 0 picks a free port. */
  port: number;
  host?: string;
}

export interface Relay {
  port: number;
  close(): void;
}

export const MAX_PAYLOAD_BYTES = 16 * 1024;

/** Handles one binary client message; returns the reply bytes, if any. */
export function handleClientMessage(bytes: Uint8Array, client: ClientData): Uint8Array | null {
  const header = readHeader(bytes);
  if (!isSupportedVersion(header.version)) return null;
  switch (header.kind) {
    case Payload.Ping: {
      const ping = decodeEnvelope(bytes).payload(new Ping()) as Ping;
      return encodePong(0, ping.nonce());
    }
    case Payload.Subscribe:
      client.channels = subscribeChannels(decodeEnvelope(bytes));
      return null;
    default:
      return null;
  }
}

export function startRelay({ port, host = '0.0.0.0' }: RelayOptions): Promise<Relay> {
  const app = uWS.App();
  let connections = 0;

  app.get('/health', (res) => {
    res.writeHeader('content-type', 'application/json');
    res.end(JSON.stringify({ status: 'ok', connections }));
  });

  app.ws<ClientData>('/ws', {
    compression: uWS.DISABLED,
    maxPayloadLength: MAX_PAYLOAD_BYTES,
    idleTimeout: 60,
    upgrade: (res, req, context) => {
      res.upgrade<ClientData>(
        { channels: [] },
        req.getHeader('sec-websocket-key'),
        req.getHeader('sec-websocket-protocol'),
        req.getHeader('sec-websocket-extensions'),
        context,
      );
    },
    open: () => {
      connections++;
    },
    message: (ws, message, isBinary) => {
      if (!isBinary) return;
      try {
        // uWS reuses `message` after this callback returns; copy before decoding.
        const reply = handleClientMessage(new Uint8Array(message.slice(0)), ws.getUserData());
        if (reply) ws.send(reply, true);
      } catch {
        ws.end(1003, 'malformed envelope');
      }
    },
    close: () => {
      connections--;
    },
  });

  return new Promise((resolve, reject) => {
    app.listen(host, port, (listenSocket) => {
      if (!listenSocket) {
        reject(new Error(`relay failed to listen on ${host}:${port}`));
        return;
      }
      resolve({
        port: uWS.us_socket_local_port(listenSocket),
        close: () => {
          uWS.us_listen_socket_close(listenSocket);
          app.close();
        },
      });
    });
  });
}
