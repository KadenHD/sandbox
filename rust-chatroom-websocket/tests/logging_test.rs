use rust_chatroom_websocket::logging::{Logging, init};

#[test]
fn test_init_does_not_panic() {
    let log_level = "trace";
    init(
        log_level,
        Some(Logging {
            n_file_kept: 50,
            log_dir: "logs".into(),
            log_basename: "test".into(),
            log_suffix: "log".into(),
        }),
    );
    println!("");
    log::trace!("test trace");
    log::debug!("test debug");
    log::info!("test info");
    log::warn!("test warn");
    log::error!("test error");
}
