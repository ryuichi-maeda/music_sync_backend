gen-api-doc:
	cd ./docs/spectaql; \
	npx spectaql config.yaml

migrate:
	@echo "Migrating database..."
	sqlx db create
	sqlx migrate run