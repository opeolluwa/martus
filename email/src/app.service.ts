import { MailerService } from '@nestjs-modules/mailer';
import {
  Injectable,
  OnApplicationShutdown,
  OnModuleInit,
} from '@nestjs/common';
import {
  Consumer,
  ConsumerRunConfig,
  ConsumerSubscribeTopics,
  Kafka,
} from 'kafkajs';
import { KafkaService } from './kafka/kafka.service';

@Injectable()
export class AppService implements OnModuleInit {
  constructor(
    private readonly kafkaService: KafkaService,
    private readonly mailerService: MailerService,
  ) {}

  async onModuleInit() {
    await this.kafkaService.consume(
      { topics: ['email-queue'] },
      {
        eachMessage: async ({ topic, partition, message }) => {
          console.log({
            value: message.value.toString(),
            topic: topic.toString(),
            partition: partition.toString(),
          });

          // await this.mailerService
          // .sendMail({
          // to: recipient, // the receiver email, parsed from the job.data
          // from: '"Easepay" <info@easepay.io>', // sender address
          // subject: emailSubject, // Subject line
          // template: emailTemplate, // the email template to use  eg welcome, sign up, forgotten-password
          // context: {
          // fullname: data?.fullname || data?.name, // if the email require email
          // token: data?.token, // if the email contain JWT or OTP
          // unsubscribeLink, // link to unsubscribe from emails
          // contactUsPage, // contact up page link
          // data, // make the rest of the data accessible in the template as data[field]
          // }, // data to be used to saturate the template
          // })
          // .catch((error) => {
          // throw new Error(error);
          // });
        },
      },
    );
  }
}
