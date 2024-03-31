# Task app

Heavily inspired by the ["Rust Web App" series](https://www.youtube.com/watch?v=VIig9IcQ-w8&list=PL7r-PXl6ZPcCLvwpdD2Vj1O4CyoFTiHKd&index=16) by Jeremy Chone.

# Stack

## Backend

The backend is written in Rust. It uses:

* [sqlx](https://github.com/launchbadge/sqlx): SQLite database for storage.
* [serde](https://github.com/serde-rs/serde)/[serde-json](https://github.com/serde-rs/json): Serialization/deserialization.
* [warp](https://github.com/seanmonstar/warp): Web server/API.
* [tokio](https://github.com/tokio-rs/tokio): Async runtime.
* [log](https://github.com/rust-lang/log)/[env-logger](https://github.com/rust-cli/env_logger): Logging.
* [thiserror](https://github.com/dtolnay/thiserror): Custom error types.

### Architecture

The backend is split into three modules.

* web: Web server and REST API.
* model: Datamodel for tasks.
* database: SQLite driver.

## Frontend

