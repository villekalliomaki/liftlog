# LiftLog server

A REST API written in Rust.

## Configuration

Currently configured with environment variables or with an .env file in the working directory. Example .env file below.

```
LIFTLOG_DATABASE_URL="postgresql://"
LIFTLOG_LISTEN_ADDRESS="127.0.0.1:3000"
```

## Running

For development run with `cargo watch`. Install it first with `cargo install cargo-watch`.

To run and rebuild on file changes: `cargo watch -c -x run`.
