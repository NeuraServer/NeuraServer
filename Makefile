RUST_SRC = src/server_ops/*.rs
GO_SRC = src/server/*.go
CPP_SRC = src/server/monitor.cpp
ELIXIR_SRC = src/server_ops/*.ex

all: build_rust build_go build_cpp build_elixir

build_rust:
	cargo build --release --manifest-path $(RUST_SRC)

build_go:
	go build -o bin/server $(GO_SRC)

build_cpp:
	g++ $(CPP_SRC) -o bin/monitor -lboost_system

build_elixir:
	mix compile --source $(ELIXIR_SRC)

clean:
	cargo clean
	go clean
	rm -f bin/server
	rm -f bin/monitor
	mix clean
