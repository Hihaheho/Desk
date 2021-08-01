FROM rust:alpine3.14 as builder
WORKDIR /desk-server
COPY . .
RUN apk add --no-cache musl-dev
RUN \
	--mount=type=cache,target=/usr/local/cargo/registry \
	--mount=type=cache,target=/desk-server/target \
	cargo build -p desk-server --release \
	&& cp /desk-server/target/release/desk-server .


FROM alpine:3.14
COPY --from=builder /desk-server/desk-server /
ENTRYPOINT "/desk-server"
