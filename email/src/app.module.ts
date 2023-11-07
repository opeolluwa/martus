import { Module } from '@nestjs/common';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { MailerModule } from '@nestjs-modules/mailer';
import { EjsAdapter } from '@nestjs-modules/mailer/dist/adapters/ejs.adapter';
import { KafkaModule } from './kafka/kafka.module';
import { NodemailerModule } from './nodemailer/nodemailer.module';
import { join } from 'path';
import { ConfigModule, ConfigService } from '@nestjs/config';

@Module({
  imports: [
    ConfigModule.forRoot({ isGlobal: true }),
    MailerModule.forRootAsync({
      imports: [ConfigModule],
      useFactory: async (configService: ConfigService) => ({
        transport: configService.get<string>('SMTP_TRANSPORTER'),
        defaults: {
          from: configService.get<string>('SMTP_USERNAME'),
        },
        template: {
          dir: join(__dirname, '..', 'templates'),
          adapter: new EjsAdapter(),
          options: {
            strict: true,
          },
        },
      }),
      inject: [ConfigService],
    }),
    KafkaModule,
    NodemailerModule,
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
