# Variables
RUST_SRC = src/main.rs src/handlers.rs src/logger.rs src/scalability.rs src/config.rs
GO_SRC = src/server/handler.go src/server/logger.go src/server/zookeeper.go
CPP_SRC = src/serverasm/mod.cpp src/serverasm/traffic_monitor.cpp src/serverasm/data_processor.cpp
SCALA_SRC = src/server/blockchain.scala src/server/security.scala

RUST_TARGET = target/release/neuraserver
GO_TARGET = bin/handler bin/logger bin/zookeeper
CPP_TARGET = bin/mod bin/traffic_monitor bin/data_processor
SCALA_TARGET = bin/blockchain bin/security

# Build commands
.PHONY: all build run clean fmt lint

all: build

build: $(RUST_TARGET) $(GO_TARGET) $(CPP_TARGET) $(SCALA_TARGET)

$(RUST_TARGET): $(RUST_SRC)
	cargo build --release

$(GO_TARGET): $(GO_SRC)
	go build -o bin/handler src/server/handler.go
	go build -o bin/logger src/server/logger.go
	go build -o bin/zookeeper src/server/zookeeper.go

$(CPP_TARGET): $(CPP_SRC)
	g++ src/serverasm/mod.cpp -o bin/mod
	g++ src/serverasm/traffic_monitor.cpp -o bin/traffic_monitor
	g++ src/serverasm/data_processor.cpp -o bin/data_processor

$(SCALA_TARGET): $(SCALA_SRC)
	scalac -d bin src/server/blockchain.scala
	scalac -d bin src/server/security.scala

run:
	$(RUST_TARGET)

clean:
	cargo clean
	go clean
	rm -rf bin/*.exe bin/*.class

fmt:
	cargo fmt
	gofmt -w src/server
	scalafmt

lint:
	cargo clippy
	golint ./...
	scalastyle
