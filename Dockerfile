# Create a stage for building the application.
ARG RUST_VERSION=1.77.0
ARG APP_NAME=herpy

FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME

WORKDIR /app
COPY . .

RUN apt update && apt install --yes binutils build-essential pkg-config libssl-dev clang lld git

# Build the application
RUN cargo build --locked --release
RUN cp ./target/release/$APP_NAME /bin/herpy

FROM debian:bullseye-slim AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/herpy /bin/herpy

# Expose the port that the application listens on.
EXPOSE 8080

# What the container should run when it is started.
ENTRYPOINT ["/bin/herpy"]