# Rust - Chat Room (WebSocket)

Real-time multi-user chat using WebSockets. Browser-based front end with a Rust back end.

## Learning outcomes

- Asynchronous programming
- WebSockets
- Message broadcasting
- Concurrency

## Tools

- [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) (code tests coverage)

    ```bash
    cargo install cargo-tarpaulin
    ```

- [flexi_logger](https://docs.rs/flexi_logger/latest/flexi_logger/) (logging system CLI/Files)
- [dotenvy](https://github.com/allan2/dotenvy) (manage environments variables)
- [tokio](https://docs.rs/tokio/latest/tokio/) (asynchronous I/O concurrency)
- [axum](https://docs.rs/axum/latest/axum/) (web application framework)
- [sqlx](https://docs.rs/sqlx/latest/sqlx/) (async SQL toolkit)

    ```bash
    cargo install sqlx-cli --no-default-features --features postgres
    ```

- [include_dir](https://docs.rs/include_dir/latest/include_dir/) (embeds files in rust binary at compile time)

## Milestones

- manage envs (dev, stg, prod) [✅]
- use a logging system (info, debug, warn...) [✅]
- handle connections [✅]
- broadcast messages [✅]
- basic rooms [✅]
- message history [✅]
- persistence of message history [✅]

## Extensions

- 100% test coverage [❌]
- better dockerization (include rust, all dev/prod/stage and env vars) [❌]
- private messages [❌]
- authentication [❌]

## Commands

Execute this in you database: [init.sql](./init.sql)

See the available project commands :

```bash
make help
```

## Structure

```bash
rust-chatroom-websocket/
│
├── Cargo.toml
├── Makefile
├── .env.development
├── .env.production
├── .env.staging
│
├── src/
│   ├── main.rs
│   │
│   ├── config/
│   │   ├── mod.rs
│   │   └── settings.rs
│   │
│   ├── server/
│   │   ├── mod.rs
│   │   ├── router.rs
│   │   └── websocket.rs
│   │
│   ├── chat/
│   │   ├── mod.rs
│   │   ├── manager.rs
│   │   ├── room.rs
│   │   └── message.rs
│   │
│   ├── db/
│   │   ├── mod.rs
│   │   └── connection.rs
│   │
│   └── logging/
│       ├── mod.rs
│       └── logger.rs
│
├── static/
│   ├── chat.js
│   ├── index.html
│   └── styles.css
│
└── tests/
    ├── chat_test.rs
    ├── config_test.rs
    ├── db_test.rs
    ├── logging_test.rs
    └── server_test.rs
```
