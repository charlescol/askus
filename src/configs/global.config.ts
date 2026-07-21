import { registerAs } from "@nestjs/config";

export default registerAs("global", () => ({
  environment: process.env.NODE_ENV,
  serverUrl: process.env.SERVER_URL,
  encryptionKey: process.env.ENCRYPTION_SECRET_KEY || "SECRET_KEY",
  disableCorsPolicy: process.env.DISABLE_CORS_POLICY === "true",
}));
