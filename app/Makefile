SERVICE := marmot-timetable-chat.service
NPM_VERSION := 11.19.1
LOCAL_NPM_BIN := $(CURDIR)/.local/bin
LOCAL_NPM_ROOT := $(CURDIR)/.local/npm
NPM := $(LOCAL_NPM_BIN)/npm

.DEFAULT_GOAL := build

.PHONY: bootstrap-npm build check clean format install lint restart start status stop test typecheck

bootstrap-npm: $(NPM)

$(NPM):
	@command -v node >/dev/null 2>&1 || { echo "Node.js is required" >&2; exit 1; }
	@command -v curl >/dev/null 2>&1 || { echo "curl is required" >&2; exit 1; }
	@temporary_directory=$$(mktemp -d); \
	trap 'rm -rf "$$temporary_directory"' EXIT; \
	curl -fsSL "https://registry.npmjs.org/npm/-/npm-$(NPM_VERSION).tgz" \
		-o "$$temporary_directory/npm.tgz"; \
	tar -xzf "$$temporary_directory/npm.tgz" -C "$$temporary_directory"; \
	mkdir -p "$(LOCAL_NPM_BIN)" "$(LOCAL_NPM_ROOT)/node_modules"; \
	rm -rf "$(LOCAL_NPM_ROOT)/node_modules/npm"; \
	mv "$$temporary_directory/package" "$(LOCAL_NPM_ROOT)/node_modules/npm"; \
	ln -s "../npm/node_modules/npm/bin/npm-cli.js" "$(NPM)"

install: bootstrap-npm
	uv sync
	cd frontend && PATH="$(LOCAL_NPM_BIN):$$PATH" \
		npm_config_cache="$(LOCAL_NPM_ROOT)/cache" \
		npm_config_prefix="$(LOCAL_NPM_ROOT)" "$(NPM)" ci

build: bootstrap-npm
	cd frontend && PATH="$(LOCAL_NPM_BIN):$$PATH" \
		npm_config_cache="$(LOCAL_NPM_ROOT)/cache" \
		npm_config_prefix="$(LOCAL_NPM_ROOT)" "$(NPM)" run build

test:
	uv run pytest

lint: bootstrap-npm
	uv run ruff check .
	cd frontend && PATH="$(LOCAL_NPM_BIN):$$PATH" \
		npm_config_cache="$(LOCAL_NPM_ROOT)/cache" \
		npm_config_prefix="$(LOCAL_NPM_ROOT)" "$(NPM)" run lint

typecheck:
	uv run ty check

format: bootstrap-npm
	uv run ruff format .
	uv run ruff check . --fix
	cd frontend && PATH="$(LOCAL_NPM_BIN):$$PATH" \
		npm_config_cache="$(LOCAL_NPM_ROOT)/cache" \
		npm_config_prefix="$(LOCAL_NPM_ROOT)" "$(NPM)" run format:fix

check: lint typecheck test build

clean:
	rm -rf frontend/.next frontend/out .pytest_cache .ruff_cache build dist
	find src tests scripts -type d -name __pycache__ -prune -exec rm -rf {} +
	find src -maxdepth 1 -type d -name '*.egg-info' -exec rm -rf {} +

start:
	sudo systemctl start $(SERVICE)

stop:
	sudo systemctl stop $(SERVICE)

restart:
	sudo systemctl restart $(SERVICE)

status:
	sudo systemctl status $(SERVICE)
