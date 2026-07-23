import * as path from 'path';
import { Module } from '@nestjs/common';
import { ConfigModule } from '@nestjs/config';
import globalConfig from '@configs/global.config';
import validationSchema from '@configs/joi/validationSchema';
import { DailyPollService } from './daily-poll.service';
import { DailyPollController } from './daily-poll.controller';

@Module({
  controllers: [DailyPollController],
  providers: [DailyPollService],
  imports: [
    ConfigModule.forRoot({
      isGlobal: true,
      load: [globalConfig],
      envFilePath: path.resolve(process.cwd(), 'env', `.env.${process.env.NODE_ENV || 'local'}`),
      validationSchema,
    }),
  ],
})
export default class AppModule {}
