PROJECT_NAME = interest-form-api

run:
	if [ -f .env ]; then export $$(cat .env | xargs) && make start-localhost-db && cargo lambda watch; else make start-localhost-db && cargo lambda watch; fi

start-localhost-db:
	./scripts/start-localhost-db.sh
	./scripts/create-db-table.sh

stop-localhost-db:
	./scripts/stop-localhost-db.sh

test:
	cargo test

build:
	cargo lambda build --release

build-arm:
	cargo lambda build --release --arm64

deploy-hml:
	$(eval IAM_ROLE_ARN := $(shell cd terraform && terraform output -raw lambda_execution_role_arn))
	cargo lambda deploy \
	--region us-east-1 \
	--env-file .env.hml \
	--memory 128 \
	--timeout 30 \
	--iam-role $(IAM_ROLE_ARN) \
	--binary-name $(PROJECT_NAME) \
	hml-$(PROJECT_NAME)

deploy-prod:
	$(eval IAM_ROLE_ARN := $(shell cd terraform && terraform output -raw lambda_execution_role_arn))
	cargo lambda deploy \
	--region us-east-1 \
	--env-file .env.prod \
	--memory 128 \
	--timeout 30 \
	--iam-role $(IAM_ROLE_ARN) \
	--binary-name $(PROJECT_NAME) \
	prod-$(PROJECT_NAME)

clean:
	cargo clean

destroy-prod:
	aws lambda delete-function --function-name prod-$(PROJECT_NAME) --region us-east-1

destroy-hml:
	aws lambda delete-function --function-name hml-$(PROJECT_NAME) --region us-east-1

default: build