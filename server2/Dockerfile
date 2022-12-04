################
##### Builder
FROM rust:1.65.0 as builder
# FROM rust:1.65.0-slim as builder
ENV PKG_CONFIG_ALLOW_CROSS=1

## Install musl dependencies
RUN apt-get update
RUN apt-get install musl-tools libssl-dev build-essential -y


## Install target platform (Cross-Compilation) --> Needed for AlpineA
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new medium-rust-dockerize

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/medium-rust-dockerize/


# Set the working directory
WORKDIR /usr/src/medium-rust-dockerize

# RUN CARGO_UNSTABLE_SPARSE_REGISTRY=true cargo update

# This is a dummy build to get the dependencies cached.
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --target x86_64-unknown-linux-musl --release
RUN cargo build --target x86_64-unknown-linux-musl --release



# Now copy in the rest of the sources
COPY src /usr/src/medium-rust-dockerize/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/medium-rust-dockerize/src/main.rs

# This is the actual application build.
# RUN RUSTFLAGS=-Clinker=musl-gcc cargo build --target x86_64-unknown-linux-musl --release
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime 

# Copy application binary from builder image
# COPY --from=builder /usr/src/medium-rust-dockerize/target/release/medium-rust-dockerize /usr/local/bin
COPY --from=builder /usr/src/medium-rust-dockerize/target/x86_64-unknown-linux-musl/release/server2 /usr/local/bin

EXPOSE 8080

# Run the application
CMD ["/usr/local/bin/server2"]


