import { Controller, Get } from '@nestjs/common';
import { PROTOCOL_VERSION } from '@relight/protocol';

export interface HealthStatus {
  status: 'ok';
  service: 'api';
  protocolVersion: number;
}

@Controller('health')
export class HealthController {
  @Get()
  check(): HealthStatus {
    return { status: 'ok', service: 'api', protocolVersion: PROTOCOL_VERSION };
  }
}
