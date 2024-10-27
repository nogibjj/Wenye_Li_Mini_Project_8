# Rust Makefile
rust_format:
	cargo fmt

rust_lint:
	cargo clippy

rust_test:
	cargo test

rust_check:
	cargo check

rust_build:
	cargo build --release

rust_run:
	cargo run

all: format lint test check build run

# Python Makefile
install:
	pip install --upgrade pip &&\
		pip install -r requirements.txt

test:
	python -m pytest -vv --cov=main test_*.py

format:	
	black *.py 

lint:
	ruff check *.py

container-lint:
	docker run --rm -i hadolint/hadolint < Dockerfile

refactor: format lint
