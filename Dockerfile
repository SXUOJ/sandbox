# Builder
FROM ubuntu:18.04 as builder

# Install rust
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y curl build-essential protobuf-compiler libprotobuf-dev libseccomp-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Build judger
WORKDIR /build
COPY . .
RUN cargo build --bin grpc --release

# Runner
FROM ubuntu:18.04 as runner

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y libseccomp-dev g++

COPY --from=builder /build/target/release/grpc /sandbox

ENTRYPOINT ["/sandbox", "--addr", "[::0]:50051"]
