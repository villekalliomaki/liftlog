# LiftLog server

A REST API written in Rust.

## Configuration

Currently configured with environment variables or with an .env file in the working directory. Example .env file below.

```
LIFTLOG_DATABASE_URL="postgresql://postgres:postgres@127.0.0.1/postgres"
LIFTLOG_LISTEN_ADDRESS="127.0.0.1:3000"

# For sqlx-cli
DATABASE_URL="postgresql://postgres:postgres@127.0.0.1/postgres"
```

## Running

For development run with `cargo watch`. Install it first with `cargo install cargo-watch`.

To run and rebuild on file changes: `cargo watch -c -x run -w src`.

Development database can be brought up with `docker-compose up -d`. Because the container runs postrges under 1000:1000 and docker creates the development_data directory with root owner, the owner has to be updated to get postgres to start: `sudo chown -R 1000:1000 development_data`.

## Development

For sqlx migrations and generating them, the sqlx-cli tool is needed. Install (or update) it with `cargo install sqlx-cli`. To connect to the database: `psql "postgresql://postgres:postgres@127.0.0.1/postgres"`.