# AskUs Daily Poll

An automated NestJS bot that generates a fun daily question with OpenAI and publishes it as a native Telegram poll.
Built for private friend groups, with rotating themes, anonymous voting, and duplicate-question prevention.

## Repository structure

```
.
├── app/        # Main NestJS application
├── infra/      # Infrastructure as code (Terraform)
```

## Setup and start locally

```bash
cd app && yarn install
yarn run start:dev
```
