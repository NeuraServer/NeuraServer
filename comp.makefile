# ----------------------------------
# --- NeuraServer Copyright 2024 ---
# ----------------------------------

# Define variables
PROJECT_NAME = neura_server
TARGET_DIR = target
RELEASE_DIR = $(TARGET_DIR)/release
SRC_DIR = src

# Default target
all: build

# Build the project
build:
	cargo build --release

# Run the project
run: build
	$(RELEASE_DIR)/$(PROJECT_NAME)

# Clean the project
clean:
	cargo clean

# Format the source code
fmt:
	cargo fmt

# Check for common mistakes
lint:
	cargo clippy

.PHONY: all build run clean fmt lint
