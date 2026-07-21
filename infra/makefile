VARS_FILE ?= variables.tfvars
PROJECT_ID ?=
ZONE ?=
REGION ?= $(shell echo "$(ZONE)" | sed -E 's/-[a-z]$$//')

TF_ARGS_INFRA = -var-file=$(VARS_FILE) \
	-var="project_id=$(PROJECT_ID)" -var="zone=$(ZONE)" 

TF_ARGS_CORE_BOOTSTRAP = \
	-var="project_id=$(PROJECT_ID)" -var="zone=$(ZONE)"

.PHONY: activate_apis bootstrap_apply deploy core destroy_all help _confirm bootstrap _check_vars

bootstrap: activate_apis bootstrap_apply

help: ## Display this help
	@echo "Usage: make <target> [PROJECT_ID=...] [ZONE=...] [VARS_FILE=...] "
	@echo ""
	@echo "Targets :"
	@grep -E '^[a-zA-Z_-]+:.*?##' $(MAKEFILE_LIST) \
		| grep -v '^_' \
		| awk 'BEGIN {FS = ":.*?##"}; {printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}'

activate_apis: ## Activate required APIs
	gcloud services enable \
		iam.googleapis.com \
		cloudresourcemanager.googleapis.com \
		serviceusage.googleapis.com \
		artifactregistry.googleapis.com \
		container.googleapis.com \
		storage.googleapis.com \
		orgpolicy.googleapis.com \
		secretmanager.googleapis.com \
		compute.googleapis.com \
		--project=$(PROJECT_ID)
	@echo "Waiting for API propagation..." && sleep 30
	
bootstrap_apply: ## Apply bootstrap infrastructure
	@$(MAKE) _check_vars
	@cd bootstrap && { \
		terraform init && \
		terraform validate && \
		terraform apply $(TF_ARGS_CORE_BOOTSTRAP) ; \
	}

core: ## Deploy core infrastructure
	@$(MAKE) _check_vars
	@cd core && { \
		terraform init -backend-config="bucket=tfstate-$(PROJECT_ID)" -backend-config="prefix=core" && \
		terraform validate && \
		terraform apply $(TF_ARGS_CORE_BOOTSTRAP) ; \
	}

deploy: ## Deploy infrastructure
	@$(MAKE) _check_vars
	@cd infra && { \
		terraform init -backend-config="bucket=tfstate-$(PROJECT_ID)" -backend-config="prefix=gke" && \
		terraform validate && \
		terraform apply $(TF_ARGS_INFRA); \
	}


destroy_all: ## Destroy all resources
	@$(MAKE) _check_vars
	@echo "⚠️  You are about to destroy all resources. This action is irreversible."
	@$(MAKE) _confirm

	@cd infra && { \
		set -e; \
		terraform init -backend-config="bucket=tfstate-$(PROJECT_ID)" -backend-config="prefix=gke" && \
		terraform destroy $(TF_ARGS_INFRA) -auto-approve; \
	}

	@cd core && { \
		set -e; \
		terraform init -backend-config="bucket=tfstate-$(PROJECT_ID)" -backend-config="prefix=core" && \
		terraform destroy $(TF_ARGS_CORE_BOOTSTRAP) -auto-approve; \
	}

	@cd bootstrap && { \
		set -e; \
		terraform init && \
		terraform destroy $(TF_ARGS_CORE_BOOTSTRAP) -auto-approve; \
	}

	@$(MAKE) remove_pvcs
	@$(MAKE) remove_certs

	@gcloud services disable \
		iam.googleapis.com \
		cloudresourcemanager.googleapis.com \
		serviceusage.googleapis.com \
		artifactregistry.googleapis.com \
		container.googleapis.com \
		storage.googleapis.com \
		orgpolicy.googleapis.com \
		secretmanager.googleapis.com \
		compute.googleapis.com \
		--project=$(PROJECT_ID) \
		--force

	@echo "All resources have been destroyed successfully."


_confirm:
	@read -p "❓ Are you sure you want to continue? (yes/no): " confirm; \
	if [ "$$confirm" != "yes" ]; then \
		echo "❌ Operation cancelled."; \
		exit 1; \
	fi

_check_vars:
	@{ \
	  if [ -z "$(PROJECT_ID)" ]; then \
	    echo "❌ PROJECT_ID is not set. Use make <target> PROJECT_ID=your-project-id"; \
	    exit 1; \
	  fi; \
	  if [ -z "$(ZONE)" ]; then \
	    echo "❌ ZONE is not set. Use make <target> ZONE=your-zone"; \
	    exit 1; \
	  fi; \
	  if [ -z "$(REGION)" ]; then \
	    echo "❌ REGION could not be derived from ZONE=$(ZONE)"; \
	    exit 1; \
	  fi; \
	}