FROM rust:latest

WORKDIR /usr/src/task
COPY . .

RUN cargo install sqlx-cli
RUN cargo install --path .
CMD ["task"]
