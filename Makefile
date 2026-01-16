SHELL := /bin/bash

REQUIRED_CONTAINERS := $(shell cat docker-compose.yaml | grep "container_name" | cut -d ":" -f2 | xargs)

CLIENT_DIR := client
BACKEND_DIR := backend

.PHONY: check-podman
check-podman:
	@if ! command -v podman >/dev/null 2>&1; then \
	  echo "❌ Podman not found. Install via brew: brew install podman"; \
	  exit 1; \
	else \
	  echo "✅ podman CLI is present: $$(podman -v)"; \
	fi; \
	if ! podman machine inspect >/dev/null 2>&1; then \
	  echo "⏳ No podman machine found. Initializing…"; \
	  podman machine init --now || { echo "❌ Failed to init podman machine"; exit 1; }; \
	else \
	  if ! podman machine info | grep -q 'machinestate: Running'; then \
	    echo "⏳ Starting podman machine…"; \
	    podman machine start || { echo "❌ Failed to start podman machine"; exit 1; }; \
	  else \
	    echo "✅ Podman machine already running"; \
	  fi \
	fi

.PHONY: check-podman-compose
check-podman-compose: check-podman
	@if ! command -v podman compose >/dev/null 2>&1; then \
	  echo "❌ Podman compose not found."; \
	  echo "Install it with: brew install podman-compose"; \
	  exit 1; \
	fi ;\
	echo "✅ Podman compose is already installed"

.PHONY: run-containers
run-containers: check-podman
	@podman compose up -d --remove-orphans >/dev/null

.PHONY: wait-containers
wait-containers: run-containers
	@echo "⏳ Waiting for all required containers to exist..." ; \
	container_count=$$(echo $(REQUIRED_CONTAINERS) | wc -w) ; \
	found=0 ; \
	while [ $$found -ne $$container_count ]; do \
	  found=0 ; \
	  for c in $(REQUIRED_CONTAINERS); do \
	    if podman ps --format '{{.Names}}' | grep -q "^$$c$$"; then \
	      echo "✅ $$c exists" ; \
	      found=$$((found+1)) ; \
	    else \
	      echo "❌ $$c not found yet" ; \
	    fi ; \
	  done ; \
	  if [ $$found -ne $$container_count ]; then \
	    echo "🔄 $$found/$$container_count found — retrying in 2s..." ; \
	    sleep 2 ; \
	  fi ; \
	done ; \
	echo "🎉 All required containers exist!"

.PHONY: help
help:
	@echo "Targets:"
	@echo "  make dev            Run client + backend (parallel)"
	@echo "  make client-dev     Run Svelte dev server"
	@echo "  make backend-dev    Run Rust backend (cargo run)"
	@echo "  make client-install Install client dependencies"
	@echo "  make backend-build  Build backend"
	@echo "  make clean          Clean backend build artifacts"

.PHONY: client-install
client-install:
	@cd $(CLIENT_DIR) && npm install

.PHONY: backend-build
backend-build:
	@cd $(BACKEND_DIR) && cargo build

.PHONY: client-dev
client-dev: client-install
	@cd $(CLIENT_DIR) && npm run dev

.PHONY: backend-dev
backend-dev:
	@cd $(BACKEND_DIR) && cargo run

# Run both processes in parallel; Ctrl+C stops both.
.PHONY: dev
dev: wait-containers
	@set -e; \
	trap 'echo ""; echo "Stopping..."; kill 0' INT TERM; \
	( $(MAKE) client-dev ) & \
	( $(MAKE) backend-dev ) & \
	wait

.PHONY: clean
clean:
	@cd $(BACKEND_DIR) && cargo clean

