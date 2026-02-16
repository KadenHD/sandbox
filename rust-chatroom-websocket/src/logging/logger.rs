use crate::config::config;
use flexi_logger::{Logger, FileSpec, Duplicate, Criterion, Naming, Cleanup, Age};

/// Initialize logging once (console + file with daily rotation)
pub fn init() -> flexi_logger::LoggerHandle {
    let cfg = config();

    Logger::try_with_env_or_str(&cfg.log_level)
        .unwrap()
        .format_for_files(flexi_logger::detailed_format)
        .format_for_stderr(flexi_logger::colored_default_format)
        .log_to_file(
            FileSpec::default()
                .directory("logs")
                .basename("chatroom")
                .suffix("log"),
        )
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(30),
        )
        .duplicate_to_stderr(Duplicate::All)
        .start()
        .unwrap()
}
