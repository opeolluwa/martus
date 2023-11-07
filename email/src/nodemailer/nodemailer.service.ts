import { Injectable } from '@nestjs/common';
import { MailerService } from '@nestjs-modules/mailer';

@Injectable()
export class NodemailerService {
  constructor(private readonly mailerService: MailerService) {}
  public async sendEmail(payload: any) {
    const { email: to, subject, template, data: context } = payload;
    await this.mailerService
      .sendMail({
        to, // the receiver email, parsed from the job.data
        from: '"Martus Team" <info@martus.io>', // sender address
        subject, // Subject line
        template, // the email template to use  eg welcome, sign up, forgotten-password
        context, // data to be used to saturate the template
      })
      .catch((error) => {
        console.log('error', error.message);
        throw new Error(error);
      });
  }
}
