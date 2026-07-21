import { NestFastifyApplication } from "@nestjs/platform-fastify";
// eslint-disable-next-line import/no-extraneous-dependencies
import { TestingModule } from "@nestjs/testing";

export type E2eApp = {
  app: NestFastifyApplication;
  appModule: TestingModule;
  versionManagerQueueMock?: jest.SpyInstance;
  bearerToken?: string;
};
