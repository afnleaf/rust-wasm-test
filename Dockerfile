# Dockerfile
FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-pack

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# set the workdir within the container
WORKDIR /app

# copy the code
COPY . .

# build the project
RUN cargo build --release

EXPOSE 7878

CMD ["cargo", "run"]
#CMD ["./target/release/server"]
