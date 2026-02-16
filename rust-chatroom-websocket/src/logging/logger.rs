use flexi_logger::{Logger, FileSpec, Duplicate, Criterion, Naming, Cleanup, Age};

/// Configuration for file-based logging.
pub struct Logging {
    pub n_file_kept: usize,
    pub log_dir: String,
    pub log_basename: String,
    pub log_suffix: String,
}

impl Default for Logging {
    /// Provides default logging configuration:
    /// - Keep 30 files
    /// - Directory: "logs"
    /// - Basename: "chatroom"
    /// - Suffix: "log"
    fn default() -> Self {
        Logging {
            n_file_kept: 30,
            log_dir: "logs".into(),
            log_basename: "chatroom".into(),
            log_suffix: "log".into(),
        }
    }
}

/// Initializes logging with console output and file logging with daily rotation.
///
/// # Parameters
/// - `log_level`: A string slice representing the logging level ("trace", "debug", "info", "warn", "error").
/// - `config`: Optional `Logging` configuration.
///
/// # Returns
/// A `flexi_logger::LoggerHandle` that control and modify the logger.
pub fn init(log_level: &str, config: Option<Logging>) -> flexi_logger::LoggerHandle {
    let cfg = config.unwrap_or_default();

    Logger::try_with_env_or_str(log_level)
        .unwrap()
        .format_for_files(flexi_logger::detailed_format)
        .format_for_stderr(flexi_logger::colored_default_format)
        .log_to_file(
            FileSpec::default()
                .directory(cfg.log_dir)
                .basename(cfg.log_basename)
                .suffix(cfg.log_suffix),
        )
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(cfg.n_file_kept),
        )
        .duplicate_to_stderr(Duplicate::All)
        .start()
        .unwrap()
}
