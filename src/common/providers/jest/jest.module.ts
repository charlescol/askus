import { Global, Module } from "@nestjs/common";
import JestService from "./jest.service";

@Global()
@Module({
  providers: [JestService],
  exports: [JestService],
})
class JestModule {}

export default JestModule;
