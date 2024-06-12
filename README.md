![image (47)](https://github.com/NeuraServer/NeuraServer/assets/140754373/5f7962b5-28de-426b-b461-97c04105c4e4)

# NeuraServer

NeuraServer is an advanced TCP server that demonstrates the integration of Go for speed-critical components, Rust for main server logic, and C++ for inline assembly operations. This project showcases how to leverage the strengths of different programming languages within a single application.

## Overview

NeuraServer is designed to handle high-performance network operations with the robustness of Rust, the efficiency of Go, and the low-level capabilities of C++ for critical assembly operations.

## Features

- High-performance network operations
- Robustness with Rust
- Efficiency with Go
- Monitoring with C++
- Big Data Processing with Apache Hadoop
- Data Streaming with Apache Kafka, Apache Pulsar, and Apache Zookeeper
- Data Warehousing with Amazon Redshift, Google BigQuery, and Snowflake
- Data Lakes with Apache Hadoop and Amazon S3
- Machine Learning and AI with TensorFlow and PyTorch
- Graph Databases with Neo4j and Amazon Neptune
- Data Virtualization with Denodo
- Data Security and Privacy
- Blockchain and Distributed Ledger Technology
- Data Governance and Metadata Management
- Kubernetes orchestration for deployment, service discovery, and autoscaling
- Proxy and reverse proxy server functionality
- Advanced database interactions with Redis and MS SQL Server
- High-speed data pipelines with Kafka and Hadoop
- Scalability with Kubernetes
- Data visualization tools
- Advanced logging and monitoring
- Customizable API and OAuth2.0 support
- Support for Apache DataFusion and Flink for dynamic data processing
- Web integration with Node.js and JavaScript for IP-based connections
- Cloud computing features with Apache products
- Scala support in the build process
- Fault-tolerant and distributed system designs

## Checks
![Static Badge](https://img.shields.io/badge/server-_127.0.0.1%3A5500-red?logo=github) ![Static Badge](https://img.shields.io/badge/build-_passing-green?logo=github) ![Static Badge](https://img.shields.io/badge/docker-_present-blue?logo=github)

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Go](https://golang.org/doc/install)
- [Docker](https://www.docker.com/get-started)
- [Docker Compose](https://docs.docker.com/compose/install/)
- [GCC/MINGW](https://sourceforge.net/projects/mingw/)
- [kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl/)
- [Kubernetes cluster](https://kubernetes.io/docs/setup/)

### Steps

1. Clone the repository:

    ```sh
    git clone https://github.com/NeuraServer/NeuraServer.git
    ```

2. Navigate to the project directory:

    ```sh
    cd NeuraServer
    ```

3. Build and run the server:

    ```sh
    cargo run
    go build -o handler src/server/handler.go
    go build -o logger src/server/logger.go
    g++ src/serverasm/mod.cpp -o src/serverasm/mod.exe
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

### Running on Kubernetes

The Kubernetes configuration files are located in the `orchestration` directory.

1. Deploy the application:

    ```sh
    kubectl apply -f orchestration/deployment.yaml
    kubectl apply -f orchestration/service.yaml
    kubectl apply -f orchestration/ingress.yaml
    kubectl apply -f orchestration/autoscaler.yaml
    ```

2. Verify the deployment:

    ```sh
    kubectl get pods
    kubectl get svc
    kubectl get ing
    kubectl get hpa
    ```

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
