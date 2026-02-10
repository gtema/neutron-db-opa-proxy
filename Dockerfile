################
##### Builder
FROM rust:1.93.0-slim@sha256:df6ca8f96d338697ccdbe3ccac57a85d2172e03a2429c2d243e74f3bb83ba2f5 AS builder

RUN rustup target add x86_64-unknown-linux-musl &&\
    apt update && \
    apt install -y musl-tools musl-dev && \
    update-ca-certificates

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new neutron-db-opa-proxy

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/neutron-db-opa-proxy/
RUN mkdir -p /usr/src/neutron-db-opa-proxy/src/bin && cp /usr/src/neutron-db-opa-proxy/src/main.rs /usr/src/neutron-db-opa-proxy/src/bin/main.rs

# Set the working directory
WORKDIR /usr/src/neutron-db-opa-proxy

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

## This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY . /usr/src/neutron-db-opa-proxy/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/neutron-db-opa-proxy/src/lib.rs && touch /usr/src/neutron-db-opa-proxy/src/bin/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:3.21.3@sha256:a8560b36e8b8210634f77d9f7f9efd7ffa463e380b75e2e74aff4511df3ef88c AS runtime

LABEL maintainer="Artem Goncharov"

RUN apk add --no-cache bash

# Copy application binary from builder image
COPY --from=builder /usr/src/neutron-db-opa-proxy/target/x86_64-unknown-linux-musl/release/neutron-db-opa-proxy /usr/local/bin
