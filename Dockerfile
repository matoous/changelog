# builder
FROM rust:1.69 as builder

# protoc necessary for otel
RUN apt update && apt install -y \
  protobuf-compiler \
  && rm -rf /var/lib/apt/lists/*

# deps
WORKDIR /app
RUN USER=root cargo init
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# app
ADD . ./
RUN rm ./target/release/deps/feature_togglers*
RUN cargo build --release

# runtime
FROM debian:buster-slim
WORKDIR /app
ARG VERSION

RUN apt update \
    && apt install -y \
      ca-certificates \
      protobuf-compiler \
      tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC

COPY --from=builder /app/target/release/feature-togglers feature-togglers

EXPOSE 8080

CMD ["/app/feature-togglers"]
