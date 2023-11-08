import { Injectable, OnModuleInit } from '@nestjs/common';
import { KafkaService } from './kafka/kafka.service';
import { NodemailerService } from './nodemailer/nodemailer.service';

@Injectable()
export class AppService implements OnModuleInit {
  constructor(
    private readonly kafkaService: KafkaService,
    private readonly mailer: NodemailerService,
  ) {}

  async onModuleInit() {
    await this.kafkaService.consume(
      { topics: ['email-queue'] },
      {
        eachMessage: async ({ topic, partition, message }) => {
          const value = message.value.toString();
          this.mailer.sendEmail(JSON.parse(value));
        },
      },
    );
  }
}
