import { Module } from '@nestjs/common';
import { NodemailerService } from './nodemailer.service';

@Module({
  providers: [NodemailerService]
})
export class NodemailerModule {}
