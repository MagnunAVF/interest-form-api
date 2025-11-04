PROJECT_NAME = interest-form-api

# Include .env file and export all variables
ifneq (,$(wildcard ./.env))
	include .env
	export
endif

run:
	cargo lambda watch

test:
	cargo test

build:
	cargo lambda build --release

build-mac:
	cargo lambda build --release --arm64

ci-deploy-hml:
	cargo lambda deploy --region us-east-1 --binary-name interest-form-api hml-interest-form-api

ci-deploy-prod:
	cargo lambda deploy --region us-east-1 --binary-name interest-form-api prod-interest-form-api

clean:
	cargo clean

default: build