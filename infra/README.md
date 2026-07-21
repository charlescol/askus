# Market Streaming Infrastructure Terraform

## Scope

This repository provisions the infrastructure layer of the system on Google Cloud.

## Repository structure

```
.
├── bootstrap/   # One-time setup for Terraform state storage
├── core/        # Privileged resources such as service accounts and IAM
├── infra/       # Main cloud infrastructure
├── makefile
└── variables.tfvars
```

## Deployment flow

The infrastructure is split into three layers.

### 1. Bootstrap

The bootstrap layer creates the Terraform state storage and enables the initial resources required to manage state remotely.

```
cd bootstrap
make bootstrap PROJECT_ID=my-project-id ZONE=asia-northeast1-c
```

### 2. Core infrastructure

The core layer manages privileged resources such as service accounts and IAM bindings.

```
cd core
make core PROJECT_ID=my-project-id ZONE=asia-northeast1-c
```

This step should be run with an account that has the required administrative permissions.

### 3. Main infrastructure

The infra layer provisions the main Google Cloud resources used by the pipeline.

```
cd infra
make deploy PROJECT_ID=my-project-id ZONE=asia-northeast1-c CLUSTER_NAME=my-gke-cluster VARS_FILE=../variables.tfvars
```

This step is intended to be run with the Terraform service account created by the core layer.

## Usage

List available commands.

```
make help
```

Authenticate with the Google Cloud CLI before running Terraform commands.

```
gcloud auth application-default login
```

Make sure the target project, zone, and cluster name match the values used by the GitOps deployment.
