# NeuraServer

NeuraServer is an advanced TCP server written in Rust, demonstrating the integration of inline assembly (ASM) and asynchronous I/O using Tokio. This project serves as an example of combining low-level assembly code with the high-level capabilities of Rust.

## Features

- **Echo Server**: Basic TCP server functionality.
- **Inline ASM**: Demonstrates inline assembly for server operations.
- **Asynchronous I/O**: Utilizes Tokio for efficient I/O handling.
- **Docker Support**: Easy containerization and deployment using Docker.

## Getting Started

### Prerequisites

To build and run this project, you will need the following software installed on your system:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)

### Installation

1. **Clone the Repository**
    ```bash
    git clone https://github.com/yourusername/NeuraServer.git
    cd NeuraServer
    ```

2. **Build and Run the Server**
    ```bash
    cargo run
    ```

### Using Docker

NeuraServer includes Docker support for easy deployment. The Docker configuration files are located in the `.docker` directory.

1. **Navigate to the Docker Directory**
    ```bash
    cd .docker
    ```

2. **Build the Docker Image**
    ```bash
    docker-compose build
    ```

3. **Run the Docker Container**
    ```bash
    docker-compose up
    ```

The server will be accessible on `127.0.0.1:8080`. You can connect to it using a TCP client.

### Development

To ensure code quality and consistency, use the following commands during development:

- **Build the Project**
    ```bash
    make build
    ```

- **Run the Project**
    ```bash
    make run
    ```

- **Clean the Project**
    ```bash
    make clean
    ```

- **Format the Source Code**
    ```bash
    make fmt
    ```

- **Lint the Source Code**
    ```bash
    make lint
    ```

### License

This project is licensed under the GNU General Public License v3.0. See the [LICENSE](LICENSE) file for details.

