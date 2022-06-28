FROM rust:1.61.0

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=80

WORKDIR /app
COPY . .

RUN rustup default stable
RUN cargo build

CMD ["cargo", "run"]
