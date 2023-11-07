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
          console.log(`Received message ${value} from topic ${topic}`);
          this.mailer.sendEmail(JSON.parse(value));
          console.log({
            value: message.value.toString(),
            topic: topic.toString(),
            partition: partition.toString(),
          });
        },
      },
    );
  }
}
