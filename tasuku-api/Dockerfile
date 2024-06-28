# Rust as the base image
FROM rust:1.77.1-buster

EXPOSE 80
# 1. Create a new empty shell project
RUN USER=root cargo new --bin tasuku-api
WORKDIR /tasuku-api

# 2. Copy our manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release

# 4. Now that the dependency is built, copy your source code
COPY ./src ./src

# 5. Build for release.
RUN cargo install --path .

CMD ["tasuku-api"]