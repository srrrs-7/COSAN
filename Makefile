.PHONY: init unittest fmt key all ui auth support new-migrate sqlx-prepare

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

key:
	cd ms/dev && ./key.sh

all: ui auth cosan support key

ui:
	docker compose up -d ui --build
	sleep 5
	open http://localhost:8000/www

auth:
	docker compose up -d auth-db --build
	sleep 3
	docker compose run --rm migrator migrate hash --dir file:///go/auth/migrator
	docker compose run --rm migrator migrate apply --url postgres://root:root@auth-db:5432/auth?sslmode=disable --dir file:///go/auth/migrator
	docker compose up -d auth --build

cosan:
	docker compose up -d cosan-db --build
	sleep 3
	docker compose run --rm migrator migrate hash --dir file:///go/cosan/migrator
	docker compose run --rm migrator migrate apply --url postgres://root:root@cosan-db:5432/cosan?sslmode=disable --dir file:///go/cosan/migrator
	docker compose up -d cosan --build

support:
	docker compose up -d support-db --build
	sleep 3
	docker compose run --rm migrator migrate hash --dir file:///go/support/migrator
	docker compose run --rm migrator migrate apply --url postgres://root:root@support-db:5432/support?sslmode=disable --dir file:///go/support/migrator
	docker compose up -d support --build

new-migrate:
	docker compose run --rm migrator migrate new --dir file:///go/support/migrator