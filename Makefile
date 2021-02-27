.PHONY: run
run:
	cd interface && RUST_LOG=info cargo run -- -b server

.PHONY: run-deps
run-deps:
	docker-compose up -d