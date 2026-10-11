import { startRelay } from './server.js';

const port = Number(process.env.RELAY_PORT ?? 9001);
const relay = await startRelay({ port });
console.info(`[relay] listening on :${relay.port} (ws: /ws, health: /health)`);

for (const signal of ['SIGINT', 'SIGTERM'] as const) {
  process.on(signal, () => {
    relay.close();
    process.exit(0);
  });
}
