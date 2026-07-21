# Base image for setting up common environment
FROM node:20 as base
WORKDIR /app
COPY package.json yarn.lock ./

# Stage 1: Testing the Application
FROM base as tester
RUN yarn install --frozen-lockfile
COPY . .
CMD yarn run test

# Stage 2: Building the Application for Production
FROM base as builder
RUN yarn install --frozen-lockfile
COPY . .
RUN yarn build

# Stage 3: Production Image
FROM base as production
RUN yarn install --frozen-lockfile --production
RUN yarn audit
COPY --from=builder /app/dist ./dist

USER node

CMD yarn run start:prod
