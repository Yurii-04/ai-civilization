import 'reflect-metadata';
import { NestFactory } from '@nestjs/core';
import { AppModule } from './app.module.js';

const port = Number(process.env.API_PORT ?? 4000);
const app = await NestFactory.create(AppModule);
app.enableShutdownHooks();
await app.listen(port);
console.info(`[api] listening on :${port}`);
