FROM --platform=linux/arm64 rust:1.69 as builder

RUN echo Building on linux/arm64

# Check if we are doing cross-compilation, if so we need to add in some more dependencies and run rustup
RUN apt-get update && apt-get install --no-install-recommends -y g++-aarch64-linux-gnu libc6-dev-arm64-cross libprotobuf-dev protobuf-compiler ca-certificates && \
    rustup target add aarch64-unknown-linux-gnu && \
    rustup toolchain install stable-aarch64-unknown-linux-gnu; \

WORKDIR /app/

COPY /src/shippingservice/ /app/
COPY /pb/ /app/proto/

# Compile or crosscompile
RUN env CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \
        CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc \
        CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++ \
    cargo build --target aarch64-unknown-linux-gnu && \
    cp /app/target/aarch64-unknown-linux-gnu/release/herpy /app/target/release/herpy; \


FROM debian:bullseye-slim as release

WORKDIR /app
COPY --from=builder /app/target/release/herpy /app/herpy

EXPOSE 8080
ENTRYPOINT ["/app/herpy"]