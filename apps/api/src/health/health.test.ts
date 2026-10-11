import 'reflect-metadata';
import { NestFactory } from '@nestjs/core';
import { PROTOCOL_VERSION } from '@relight/protocol';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';
import type { INestApplication } from '@nestjs/common';
import { AppModule } from '../app.module.js';

let app: INestApplication;
let base: string;

beforeAll(async () => {
  app = await NestFactory.create(AppModule, { logger: false });
  await app.listen(0, '127.0.0.1');
  base = await app.getUrl();
});

afterAll(async () => {
  await app.close();
});

describe('GET /health', () => {
  it('reports ok with the protocol version', async () => {
    const res = await fetch(`${base}/health`);
    expect(res.status).toBe(200);
    expect(await res.json()).toEqual({
      status: 'ok',
      service: 'api',
      protocolVersion: PROTOCOL_VERSION,
    });
  });
});
