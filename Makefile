# Define project version to be used in CI/CD pipelines
PROJECT_NAME              ?= make-envs
PROJECT_VERSION           ?= 0.4.0
PROJECT_BRANCH            ?= $(shell git branch --show-current | tr / _)
PROJECT_COMMIT_HASH       ?= $(shell git rev-parse HEAD)
PROJECT_COMMIT_HASH_SHORT ?= $(shell echo $(PROJECT_COMMIT_HASH) | cut -c 1-7)
BUILD_ID                  ?= local

# Global parameters for Docker image publishing
AWS_REGION ?= eu-west-1
DOCKER_REGISTRY_REPO ?= 630104266194.dkr.ecr.eu-west-1.amazonaws.com

# Parameters to be used in a Docker tag
# Format example: 0.1.0.master.abc123.b0
BASE_CONTAINER_NAME := $(PROJECT_NAME)
BASE_CONTAINER_TAG  := $(PROJECT_VERSION).$(PROJECT_BRANCH).$(PROJECT_COMMIT_HASH_SHORT).$(BUILD_ID)

# accept name and tag overrides if needed, defaults to the values calculated above
OVERRIDDEN_CONTAINER_NAME ?= $(BASE_CONTAINER_NAME)
OVERRIDDEN_CONTAINER_TAG  ?= $(BASE_CONTAINER_TAG)

# Scan ci/ folder and find all the folders
$(eval CONTAINERS_LIST := $(shell ls -d ci/*/ | awk -F"/" '{ print $$1"/"$$2 }'))

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# Helper target, which builds proper files layout in a project's root folder
build_files_layout:
	@echo "*** Build files layout in a project root folder... ***"
	@cp -R ci/container.sh $(CURDIR)
	@for container in $(CONTAINERS_LIST); do \
		if [ $$container == "ci/container_main" ]; then \
			cp $$container/Dockerfile $(CURDIR)/Dockerfile; \
		else \
			cp $$container/Dockerfile $(CURDIR)/Dockerfile.$$(echo $$container | awk -F"_" '{ print $$2 }'); \
		fi \
	done
	@echo ""


docker_build: build_files_layout ## Build Docker image
	@echo "*** Building the images... ***"
	./container.sh --build $(AWS_REGION) $(BASE_CONTAINER_NAME) $(BASE_CONTAINER_TAG) $(DOCKER_REGISTRY_REPO)
	@echo ""


docker_publish: build_files_layout docker_build ## Push Docker image
	@echo "*** Pushing the images to the repo... ***"
	./container.sh --publish $(AWS_REGION) $(BASE_CONTAINER_NAME) $(BASE_CONTAINER_TAG) $(DOCKER_REGISTRY_REPO)
	@echo ""

docker_run: build_files_layout ## Run container with the arguments provided example: make docker_run ARGS=run_server
	@echo "*** Running the container, image tag used -> $(OVERRIDDEN_CONTAINER_TAG)... ***"
	@docker run $(DOCKER_ENV) $(OVERRIDDEN_CONTAINER_NAME):$(OVERRIDDEN_CONTAINER_TAG) $(ARGS)
	@echo ""


clean: ## Cleanup any files created in a root project folder during the Makefile run
	@echo "*** Cleaning up the root project folder... ***"
	@rm -rf Dockerfile*
	@rm -rf container.sh
	@echo ""

set_teamcity_parameters:
	@echo "*** Setting the image version in TeamCity parameters... ***"
	@echo "##teamcity[setParameter name='env.BASE_CONTAINER_TAG' value='$(BASE_CONTAINER_TAG)']"
	@echo ""

.PHONY:  help build_files_layout docker_build docker_publish set_teamcity_parameters clean

