.PHONY: init unittest ui ui-build ui-check auth support new-migrate sqlx-prepare

init:
	cp compose.override.yaml.sample compose.override.yaml

unittest:
	docker compose run --rm gopher /go/ms/dev/test.sh gopher
	docker compose run --rm rustacean /ms/dev/test.sh rustacean
	docker compose run --rm deno /ms/dev/test.sh deno

fmt:
	docker compose run --rm gopher /go/ms/dev/fmt.sh gopher
	docker compose run --rm rustacean /ms/dev/fmt.sh rustacean
	docker compose run --rm deno /ms/dev/fmt.sh deno

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
	docker compose up -d auth-db --build
	sleep 3
	docker compose run --rm migrator migrate hash --dir file:///go/auth/migrator
	docker compose run --rm migrator migrate apply --url postgres://root:root@auth-db:5432/auth?sslmode=disable --dir file:///go/auth/migrator
	docker compose up -d auth --build

support:
	docker compose up -d support-db --build
	sleep 3
	docker compose run --rm migrator migrate hash --dir file:///go/support/migrator
	docker compose run --rm migrator migrate apply --url postgres://root:root@support-db:5432/support?sslmode=disable --dir file:///go/support/migrator
	docker compose up -d support --build

new-migrate:
	docker compose run --rm migrator migrate new --dir file:///go/support/migrator