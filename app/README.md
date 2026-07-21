# Marketaux

This project is a template for NestJS. It offers a robust starting point and includes fundamental unit tests and end-to-end tests, exemplified through a simple "Hello World" function located in the main application module.

## Key Commands

This project utilizes several `yarn` scripts for efficient development, testing, and deployment:

- `yarn build`: Compiles the application using Nest's build command. This is essential for preparing the application for production deployment.
- `yarn start:dev`: Launches the application in development mode with live reloading, useful for immediate feedback during development.
- `yarn start:prod`: Starts the application in production mode, using the compiled files from the `dist` directory.
- `yarn lint`: Executes ESLint to identify and fix problems in your JavaScript code, promoting code quality and consistency.
- `yarn test`: Runs the Jest test suite, a crucial step for ensuring application functionality.

## Environment Variable Management

The project uses environment variables for configuration, stored in the `/env` folder. Follow these guidelines for setting up and maintaining these variables:

- **.env.local**: For development settings.
- **.env.test**: Configurations for the testing environment.
- **.env.prod**: Environment variables for production.
- **.env.staging**: Settings for the staging environment.

Base your configuration on the `.env.example` provided. Ensure any changes to environment variables are accompanied by corresponding updates in the Joi validation schema within the `config` folder. This step is crucial for maintaining application configuration integrity.

## Project Structure

The organization of the project's source code directories is defined as below:

- `src/configs/*` : This directory contains all the configuration files related to the project. It includes configurations for the tests, and various environment files. 
- `src/common/*`: Here, you will find common shared files and modules. These are utility files and modules that are used across different parts of the application, providing a centralized location for code that is used frequently throughout the project.
- `test/*`: This directory is set up for configuring and recreating the test environment by the pipeline.


