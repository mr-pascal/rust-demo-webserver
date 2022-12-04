#################
##### Arguments
ARG APP

################
##### Builder
FROM rust:1.65.0 as builder
ARG APP
ENV PKG_CONFIG_ALLOW_CROSS=1

## Install musl dependencies
RUN apt-get update
RUN apt-get install musl-tools libssl-dev build-essential -y


## Install target platform (Cross-Compilation) --> Needed for AlpineA
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new $APP

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/$APP/

# Set the working directory
WORKDIR /usr/src/$APP

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/$APP/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/$APP/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.16.0 AS runtime 
ARG APP

# Copy application binary from builder image
COPY --from=builder /usr/src/$APP/target/x86_64-unknown-linux-musl/release/$APP /usr/local/bin
RUN mv /usr/local/bin/$APP /usr/local/bin/app

EXPOSE 8080

# Run the application
CMD ["/usr/local/bin/app"]


