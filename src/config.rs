use crate::generator::name_generator::NameGenerator;
use std::env;

pub struct AppState {
    pub app_config: AppConfig,
    pub name_generator: NameGenerator,
}

impl AppState {
    pub fn new(app_config: AppConfig, name_generator: NameGenerator) -> Self {
        Self {
            app_config,
            name_generator,
        }
    }
}

#[cfg_attr(test, derive(PartialEq))]
#[derive(Debug)]
pub struct DBConfig {
    postgres_user: String,
    postgres_password: String,
    postgres_database_name: String,
    postgres_host: String,
    postgres_port: String,
}

pub struct AppConfig {
    pub base_url: String,
    pub app_port: String,
    pub db_config: DBConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        if env::var("APP_PROD").is_ok() {
            log::info!("App starting in production mode.");
            AppConfig::from_env()
        } else {
            dotenvy::from_path("config/.env")
                .inspect_err(|_| {
                    log::info!("No .env file was found. Proceeding with the test values.")
                })
                .map(|_| AppConfig::from_env())
                .unwrap_or_else(|_| AppConfig::new())
        }
    }
}

impl AppConfig {
    fn from_env() -> Self {
        let base_url_key_name = "BASE_URL";
        let app_port_key_name = "APP_PORT";
        let base_url = read_key(base_url_key_name);
        let app_port = read_key(app_port_key_name);
        let db_config = DBConfig::from_env();
        if !base_url.ends_with('/') {
            panic!("The base URL must end with a slash '/' for the application to work correctly.")
        };

        AppConfig {
            base_url,
            app_port,
            db_config,
        }
    }

    fn new() -> Self {
        AppConfig {
            base_url: "http://localhost:8000/".to_string(),
            app_port: "8000".to_string(),
            db_config: DBConfig::new(),
        }
    }
}

impl DBConfig {
    fn from_env() -> Self {
        let user_key_name = "POSTGRES_USER";
        let password_key_name = "POSTGRES_PASSWORD";
        let db_key_name = "POSTGRES_DB";
        let host_key_name = "POSTGRES_HOST";
        let port_key_name = "POSTGRES_PORT";

        let postgres_user = read_key(user_key_name);
        let postgres_password = read_key(password_key_name);
        let postgres_database_name = read_key(db_key_name);
        let postgres_host = read_key(host_key_name);
        let postgres_port = read_key(port_key_name);
        DBConfig {
            postgres_user,
            postgres_password,
            postgres_database_name,
            postgres_host,
            postgres_port,
        }
    }

    fn new() -> Self {
        DBConfig {
            postgres_user: "postgres".to_string(),
            postgres_password: "postgres".to_string(),
            postgres_database_name: "postgres".to_string(),
            postgres_host: "localhost".to_string(),
            postgres_port: "5432".to_string(),
        }
    }

    pub fn to_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database_name
        )
    }
}

fn key_error_message(key: &str) -> String {
    format!(".env file was read but is missing environment variable {key}. Cannot proceed with startup.")
}

fn read_key(key: &str) -> String {
    let error_msg = key_error_message(key);
    env::var(key).expect(&error_msg)
}

impl Default for DBConfig {
    /// Constructs a new DBConfig instance by reading the .env file.
    /// If the file is not found, it will use the default values. This is intentionally done
    /// to make it easy to use the DBConfig in a dev environment.
    fn default() -> Self {
        if env::var("APP_PROD").is_ok() {
            DBConfig::from_env()
        } else {
            dotenvy::from_path("config/.env")
                .inspect_err(|_| {
                    log::info!("No .env file was found. Proceeding with the test values.")
                })
                .map(|_| DBConfig::from_env())
                .unwrap_or_else(|_| DBConfig::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loading_default_app_config_loads_default_db_config() {
        let app_config = AppConfig::default();
        let db_config = DBConfig::default();
        assert_eq!(app_config.db_config, db_config);
    }
}
