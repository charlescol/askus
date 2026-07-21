import { Controller, Headers, HttpCode, HttpStatus, Post, UnauthorizedException } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { DailyPollService } from './daily-poll.service';

@Controller('daily-poll')
export class DailyPollController {
  private readonly debugToken: string;

  constructor(
    private readonly dailyPollService: DailyPollService,
    config: ConfigService,
  ) {
    this.debugToken = config.getOrThrow<string>('POLL_DEBUG_TOKEN');
  }

  @Post('debug')
  @HttpCode(HttpStatus.OK)
  async publishNow(@Headers('authorization') authorization?: string): Promise<void> {
    if (authorization !== `Bearer ${this.debugToken}`) {
      throw new UnauthorizedException('Jeton de debug invalide.');
    }

    return this.dailyPollService.publishDailyPoll();
  }
}
