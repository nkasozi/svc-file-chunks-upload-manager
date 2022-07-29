# Start with a rust alpine image
FROM rust:1-alpine3.15 as build

# This is important, see https://github.com/rust-lang/docker-rust/issues/85
ENV RUSTFLAGS="-C target-feature=-crt-static"
# if needed, add additional dependencies here
# Install required tools and packages
# Ensure to update the versions of the packages
# if Alpine updates. Get the latest version details from
# https://pkgs.alpinelinux.org/packages
RUN apk add --update --no-cache \
    protobuf=3.18.1-r1 \
    protobuf-dev=3.18.1-r1

RUN apk add --no-cache musl-dev

# create a new empty shell project
RUN USER=root cargo new --bin svc-task-details-repository-manager
WORKDIR /svc-task-details-repository-manager

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN rustup component add rustfmt

# copy your source tree
COPY ./src ./src

# build for release
RUN cargo build --release -v

# our final base
# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.15

# if needed, install additional dependencies here
RUN apk add --no-cache libgcc

# copy the build artifact from the build stage
COPY --from=build /svc-task-details-repository-manager/target/release/svc-task-details-repository-manager .

# set the startup command to run your binary
CMD ["./svc-task-details-repository-manager"]