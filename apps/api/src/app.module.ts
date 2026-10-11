import { Module } from '@nestjs/common';
import { HealthController } from './health/health.controller.js';

// Feature modules (auth, wallet, blessings, payments, seasons, admin) are
// added from M6 on; see docs/ARCHITECTURE.md.
@Module({
  controllers: [HealthController],
})
export class AppModule {}
