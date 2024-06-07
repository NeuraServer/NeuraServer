# NeuraServer

NeuraServer is an advanced TCP server that demonstrates the integration of Go for speed-critical components, Rust for main server logic, and C++ for inline assembly operations. This project showcases how to leverage the strengths of different programming languages within a single application.

## Overview

NeuraServer is designed to handle high-performance network operations with the robustness of Rust, the efficiency of Go, and the low-level capabilities of C++ for critical assembly operations.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Go](https://golang.org/doc/install)
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [GCC/MINGW](https://sourceforge.net/projects/mingw/)

### Steps

1. Clone the repository:

    ```sh
    git clone https://github.com/yourusername/NeuraServer.git
    ```

2. Navigate to the project directory:

    ```sh
    cd NeuraServer
    ```

3. Build and run the server:

    ```sh
    cargo run
    ```

## Usage

### Running with Docker

The Docker configuration files are located in the `.docker` directory.

1. Navigate to the Docker directory:

    ```sh
    cd .docker
    ```

2. Build the Docker image:

    ```sh
    docker-compose build
    ```

3. Run the Docker container:

    ```sh
    docker-compose up
    ```

The server will be accessible on `127.0.0.1:8080`. You can connect to it using a TCP client.

## Development

To ensure code quality and consistency, use the following commands during development:

- Build the project:

    ```sh
    make build
    ```

- Run the project:

    ```sh
    make run
    ```

- Clean the project:

    ```sh
    make clean
    ```

- Format the source code:

    ```sh
    make fmt
    ```

- Lint the source code:

    ```sh
    make lint
    ```

## Source Code

The source code for NeuraServer is organized as follows:

- `main.rs`: The main entry point of the application. Initializes the server and starts the main event loop.
- `server/mod.rs`: Contains the server implementation, including the setup and handling of connections.
- `server/handler.go`: Defines the logic for handling client connections and requests in Go.
- `asm/mod.cpp`: Contains the inline assembly code for low-level server operations in C++.
- `.docker/Dockerfile`: Dockerfile for building the Docker image.
- `.docker/docker-compose.yml`: Docker Compose file for running the server in a Docker container.

### Main Components

#### main.rs

```rust
use tokio::net::TcpListener;
use crate::server::run_server;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    run_server(listener).await?;
    Ok(())
}
```

