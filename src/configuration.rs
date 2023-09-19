use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
    // pub email_client: EmailClientSettings,
    // pub email_client_base_url: String,
    // pub redis: RedisSettings,
    // pub web_app_base_url: String,
    // pub web_app_url_path: String,
    // pub allowed_origins: String,
    // pub jwt_secret: String,
    // pub jwt_expiry_days: u32,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailClientSettings {
    pub base_url: String,
    pub sender_email: String,
    pub authorization_token: String,
}

#[derive(Debug, Deserialize)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize()
}
