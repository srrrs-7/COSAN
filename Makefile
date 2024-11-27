.PHONY: init ui auth support

init:
	cp compose.override.yaml.sample compose.override.yaml

ui:
	docker compose up -d ui --build
	sleep 5
	open http://localhost:8000

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