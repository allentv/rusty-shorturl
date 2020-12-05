FROM rustlang/rust:nightly-alpine AS build

LABEL base-reference="https://alexbrand.dev/post/how-to-package-rust-applications-into-minimal-docker-containers/"
LABEL author="Allen Thomas Varghese"
LABEL email="allentv4u@gmail.com"

# Installing this package to support gcc compilation
RUN apk add --no-cache musl-dev

# Download the target for static linking.
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app

# Create a new project
RUN USER=root cargo new url-shortener

WORKDIR /app/url-shortener

# Load the source files
COPY . .

# Create production build
RUN cargo build

# Add the executable to the path
CMD ["cargo", "run"]
