# From: https://kerkour.com/rust-small-docker-image

FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create appuser
ENV USER=liftlog_server
ENV UID=1000

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/none" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /liftlog_server

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /liftlog_server

# Copy our build
COPY --from=builder /liftlog_server/target/x86_64-unknown-linux-musl/release/liftlog_server ./

# Use an unprivileged user.
USER liftlog_server:liftlog_server

CMD ["/liftlog_server/liftlog_server"]