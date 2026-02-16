# Rust - Chat Room (WebSocket)

Real-time multi-user chat using WebSockets. Browser-based front end with a Rust back end.

## Learning outcomes

- Asynchronous programming
- WebSockets
- message broadcasting
- concurrency

## Tools

- [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin) (code tests coverage)

    ```bash
    cargo install cargo-tarpaulin
    ```

- [dotenvy](https://github.com/allan2/dotenvy) (manage environments variables)
- [tokio](https://docs.rs/tokio/latest/tokio/) (asynchronous I/O concurrency)
- [tokio-tungstenite](https://docs.rs/tokio-tungstenite/latest/tokio_tungstenite/) (asynchronous websocket usage)
- [axum](https://docs.rs/axum/latest/axum/) (web application framework)

## Milestones

- manage envs (dev, stg, prod) [✅]
- use a logging system (info, debug, warn...) [❌]
- handle connections [❌]
- broadcast messages [❌]
- basic rooms [❌]
- message history [❌]

## Extensions

- private messages [❌]
- authentication [❌]
- persistence of message history [❌]

## Commands

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
│    ├── main.rs
│    │
│    ├── config/
│    │   ├── mod.rs
│    │   └── settings.rs
│    │
│    ├── server/
│    │   ├── mod.rs
│    │   ├── router.rs
│    │   └── websocket.rs
│    │
│    ├── chat/
│    │   ├── mod.rs
│    │   ├── room.rs
│    │   └── message.rs
│    │
│    ├── db/
│    │   ├── mod.rs
│    │   └── connection.rs
│    │
│    └── logging/
│        ├── mod.rs
│        └── init.rs
│
└── tests/
    ├── chat_test.rs
    ├── config_test.rs
    ├── db_test.rs
    ├── logging_test.rs
    ├── main_test.rs
    └── server_test.rs
```
