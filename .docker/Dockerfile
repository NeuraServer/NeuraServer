# Use an official Rust runtime as a parent image
FROM rust:latest

# Set the working directory in the container
WORKDIR /usr/src/app

# Copy the current directory contents into the container at /usr/src/app
COPY . .

# Install any needed packages specified in requirements.txt
RUN apt-get update && apt-get install -y gcc g++ libssl-dev

# Build the Rust project
RUN cargo build --release

# Build the Go handler and logger
RUN go build -o handler src/server/handler.go
RUN go build -o logger src/server/logger.go

# Build the C++ server simulation
RUN g++ src/serverasm/mod.cpp -o src/serverasm/mod.exe

# Run the server
CMD ["./target/release/neuraserver"]

