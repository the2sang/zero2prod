use secrecy::SecretString;
use secrecy::ExposeSecret;
//use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub port: u16,
    pub host: String,
    pub database_name: String,
   // pub require_ssl: bool
}

// impl DatabaseSettings {
//     pub fn connection_string(&self) -> PgConnectOptions {
//         let ssl_mode = if self.require_ssl {
//             PgSslMode::Require
//         } else {
//             PgSslMode::Prefer
//         };
//         PgConnectOptions::new()
//             .host(&self.host)
//             .password(self.password.expose_secret().clone())
//             .port(self.port)
//             .ssl_mode(ssl_mode)
//             .database(&self.database_name)
//     }
// }

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new("configuration.yaml", config::FileFormat::Yaml)
        )
        .build()?;
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> SecretString {
        SecretString::from(format!("postgres://{}:{}@{}:{}/{}",
                self.username, self.password.expose_secret(), self.host, self.port, self.database_name))
    }

    pub fn connection_string_without_db(&self) -> SecretString {
       SecretString::from(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}