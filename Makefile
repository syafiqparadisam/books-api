
run: docker-compose
	cargo run
docker-compose:
	cd infra/docker && docker compose up -d

down:
	cd infra/docker && docker compose down
