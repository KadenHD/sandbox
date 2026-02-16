use rust_chatroom_websocket::server::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
