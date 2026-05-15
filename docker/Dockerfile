# Build stage
FROM rust:alpine AS builder

RUN apk add --no-cache \
    musl-dev \
    build-base \
    # pkgconf \
    nodejs \
    npm

RUN npm install -g pnpm

WORKDIR /app

# Install JS deps first for better layer caching
COPY templates/package.json templates/pnpm-lock.yaml ./templates/
RUN cd templates && pnpm install --frozen-lockfile

# Copy source (node_modules excluded via .dockerignore)
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates

RUN cargo build --release

# Runtime stage
FROM alpine:3.23

# RUN apk upgrade --no-cache && apk add --no-cache ca-certificates tzdata

# Create a non-root system user with fixed UID/GID 1000 (matches typical host user)
RUN addgroup -S -g 1000 gandalf \
 && adduser -S -G gandalf -u 1000 -H -s /sbin/nologin gandalf

WORKDIR /app

COPY --from=builder --chown=gandalf:gandalf /app/target/release/why_is_docker_the_best ./

EXPOSE 8080

ENV HOST=0.0.0.0:8080
ENV CARGO_CRATE_NAME=why_is_docker_the_best
ENV RUST_LOG=why_is_docker_the_best=info,tower_http=info,axum::rejection=trace

USER gandalf

CMD ["./why_is_docker_the_best"]
