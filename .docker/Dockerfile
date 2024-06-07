# Use the official Rust image as the base image
FROM rust:1.58 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin neura_server
WORKDIR /neura_server

# Copy the manifests
COPY Cargo.toml Cargo.lock ./

# Build the dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy the source code
COPY src ./src

# Build the project
RUN cargo build --release

# Use a minimal base image to reduce the size
FROM debian:buster-slim

# Copy the build artifacts from the builder
COPY --from=builder /neura_server/target/release/neura_server /usr/local/bin/neura_server

# Run the binary
CMD ["neura_server"]
