import * as Joi from 'joi';

const validationSchema = Joi.object({
  DISABLE_SWAGGER: Joi.boolean(),
  DISABLE_APP_ADDRESS_LISTENING: Joi.boolean(),
  PORT: Joi.number(),
  OPENAI_API_KEY: Joi.string().required(),
  OPENAI_MODEL: Joi.string().default('gpt-5.6-luna'),
  TELEGRAM_BOT_TOKEN: Joi.string().required(),
  TELEGRAM_CHAT_ID: Joi.string().required(),
  DAILY_POLL_HISTORY_FILE: Joi.string().optional(),
});

export default validationSchema;
