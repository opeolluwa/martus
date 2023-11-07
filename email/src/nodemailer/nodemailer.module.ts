import { Module } from '@nestjs/common';
import { NodemailerService } from './nodemailer.service';
import { MailerModule } from '@nestjs-modules/mailer';

@Module({
  providers: [NodemailerService],
  exports: [NodemailerService],
  imports: [MailerModule],
})
export class NodemailerModule {}

