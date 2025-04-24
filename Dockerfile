FROM rust:alpine AS build-env

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Install necessary build dependencies
RUN apk add --no-cache musl-dev upx

RUN cargo build --release

# Compress the binary using upx
RUN upx --best target/release/async-runtime

# Use a minimal base image for the final image
FROM scratch

COPY --from=build-env /app/target/release/async-runtime .

CMD ["/async-runtime"]