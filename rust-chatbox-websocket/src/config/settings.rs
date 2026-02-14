use dotenvy::from_filename;
use std::env;
use std::sync::LazyLock;

pub struct Config {
    pub env_name: String,
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
    pub log_level: String,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let env_name = env::var("APP_ENV").unwrap_or_else(|_| "development".into());

        // Load environment file for non-production and non-test
        if !matches!(env_name.as_str(), "production" | "test") {
            let filename = format!(".env.{}", env_name);
            from_filename(&filename).unwrap_or_else(|_| panic!("Missing {}", filename));
        }

        Self {
            env_name,
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "rust-chatroom-websocket".into()),

            app_host: env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into()),

            app_port: env::var("APP_PORT")
                .unwrap_or_else(|_| "8080".into())
                .parse()
                .expect("APP_PORT must be a valid u16"),

            log_level: env::var("LOG_LEVEL").unwrap_or_else(|_| "info".into()),

            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}

static CONFIG: LazyLock<Config> = LazyLock::new(Config::from_env);

pub fn config() -> &'static Config {
    &CONFIG
}
