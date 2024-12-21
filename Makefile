.PHONY: init ui ui-build ui-check auth support new-migrate

init:
	cp compose.override.yaml.sample compose.override.yaml

ui:
	docker compose up -d ui --build
	sleep 5
	open http://localhost:8000/www
ui-build:
	docker compose run --rm ui deno task build
	docker compose run --rm ui deno task preview
ui-check:
	docker compose run --rm ui deno task check

auth:
	docker compose up -d auth-db auth-redis --build
	sleep 3
	docker compose run --rm migrator /usr/local/bin/atlas migrate hash --dir file:///go/auth/migrator
	docker compose run --rm migrator /usr/local/bin/atlas migrate apply --url postgres://root:root@auth-db:5432/auth?sslmode=disable --dir file:///go/auth/migrator
	docker compose up -d auth --build

support:
	docker compose up -d support-db --build
	sleep 3
	docker compose run --rm migrator /usr/local/bin/atlas migrate hash --dir file:///go/support/migrator
	docker compose run --rm migrator /usr/local/bin/atlas migrate apply --url postgres://root:root@support-db:5432/support?sslmode=disable --dir file:///go/support/migrator
	docker compose up -d support --build

DIR="file:///go/support/migrator"
new-migrate:
	docker compose run --rm migrator /usr/local/bin/atlas migrate new --dir ${DIR}