.PHONY: dev run test build-docker run-docker stop-docker cleanup-docker

dev:
	cargo install sqlx-cli  --no-default-features --features sqlite
	sqlx database create
	sqlx migrate run
	cargo build
	mkdir -p uploads

run: dev
	cargo run

test:
	cargo test

build-docker:
	docker build -f Dockerfile.builder -t microbloggy-builder .
	docker run -v ${PWD}:/opt \
		-v microbloggy-builder-cargo:/cargo \
		-e CARGO_HOME=/cargo microbloggy-builder \
			cargo build --release
	docker build -t microbloggy .

run-docker: build-docker
	docker run \
		--rm \
		--detach \
		--name microbloggy \
		-p 8080:8080 \
		-v microbloggy-data:/data \
		-v microbloggy-uploads:/uploads \
		-e ADMIN_USERNAME=testuser \
		-e ADMIN_PASSWORD=testpassword \
		-e SESSION_SECRET=session-secret-atleast-32-chars500 \
		-e DATABASE_URL=sqlite:///data/testdatabse.sqlite3 \
		-e UPLOADS_PATH=/uploads \
		microbloggy

stop-docker:
	docker stop microbloggy

cleanup-docker: stop-docker
	docker volume rm microbloggy-data
	docker volume rm microbloggy-uploads
