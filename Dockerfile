# Dockerfile
FROM rust:1.83-alpine

# set the workdir within the container
WORKDIR /app

# copy the code
COPY . .

RUN cargo build

CMD ["cargo", "run"]
