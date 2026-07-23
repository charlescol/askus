import { NestFactory } from '@nestjs/core';
import { FastifyAdapter, NestFastifyApplication } from '@nestjs/platform-fastify';
import { ValidationPipe } from '@nestjs/common/pipes';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';
import AppModule from './app.module';

async function bootstrap(): Promise<void> {
  /* AppConfiguration */
  const app = await NestFactory.create<NestFastifyApplication>(AppModule, new FastifyAdapter({ bodyLimit: 50048576 }));

  /* Pipes Configuration */
  app.useGlobalPipes(new ValidationPipe({ whitelist: true, transform: true }));
  app.setGlobalPrefix('api');

  /* Swagger configuration */
  const swaggerConfig = new DocumentBuilder()
    .setTitle('Project')
    .setDescription('Project API')
    .setVersion('1.0')
    .addBearerAuth()
    .build();

  if (process.env.DISABLE_SWAGGER !== 'true') {
    const document = SwaggerModule.createDocument(app, swaggerConfig);
    SwaggerModule.setup('swagger', app, document);
  }

  /* Port Configuration */
  const port = Number(process.env.PORT) || 3000;
  await app.listen(port, '0.0.0.0');
}

bootstrap();
