.PHONY: run
run:
	cd interface && RUST_LOG=info cargo run -- -b server

.PHONY: run-deps
run-deps:
	docker-compose up -d

.PHONY: migrate
migrate:
	DATABASE_URL=postgresql://feeder:feeder@0.0.0.0:5433/feeder  sqlx migrate --source feeder/migrations run && \
	DATABASE_URL=postgresql://feeder:feeder@0.0.0.0:5433/feeder  sqlx migrate --source interface/migrations run --ignore-missing

