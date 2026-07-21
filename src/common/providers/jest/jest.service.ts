import AppModule from "@src/app.module";
import { Injectable, ValidationPipe } from "@nestjs/common";
import {
  FastifyAdapter,
  NestFastifyApplication,
} from "@nestjs/platform-fastify";
// eslint-disable-next-line import/no-extraneous-dependencies
import { Test, TestingModuleBuilder } from "@nestjs/testing";
import { E2eApp } from "./jest.types";

@Injectable()
class JestService {
  static async getE2EApp(): Promise<E2eApp> {
    const appModuleBuilder: TestingModuleBuilder = Test.createTestingModule({
      imports: [AppModule],
    });

    const appModule = await appModuleBuilder.compile();

    const app = appModule.createNestApplication<NestFastifyApplication>(
      new FastifyAdapter()
    );

    app.useGlobalPipes(
      new ValidationPipe({ whitelist: true, transform: true })
    );
    app.setGlobalPrefix("api");

    await app.init();
    await app.getHttpAdapter().getInstance().ready();

    let e2eApp: E2eApp = {
      app,
      appModule,
    };
    e2eApp = { ...e2eApp };

    return e2eApp;
  }

  static async closeApp(app: NestFastifyApplication): Promise<void> {
    try {
    } catch (error) {
      // eslint-disable-next-line no-console
      console.error("Error during cleanup:", error);
    } finally {
      /* Ensure the application is closed */
      await app.close();
    }
  }
}

export default JestService;
