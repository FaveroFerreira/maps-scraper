use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Environment {
    pub database_url: String,
}

pub fn load_env_vars() -> Environment {
    let profile = std::env::var("PROFILE").unwrap_or_default();

    if profile.is_empty() {
        dotenvy::dotenv().expect("failed to load .env");
    }

    envy::from_env().expect("failed to parse environment variables")
}
