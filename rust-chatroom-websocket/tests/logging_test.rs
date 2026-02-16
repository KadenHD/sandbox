#[test]
fn test_init_does_not_panic() {
    rust_chatroom_websocket::logging::init();
    log::info!("test")
}
