use std::path::Path;

use chrono::{DateTime, Utc};
use sqlx::migrate::Migrator;
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

use crate::environment::Environment;
use crate::error::Error;

#[derive(Debug, FromRow)]
pub struct Search {
    pub id: String,
    pub started_at: DateTime<Utc>,
    pub finished_at: DateTime<Utc>,
    pub error: Option<String>,
    pub search_parameter_id: String,
}

#[derive(Debug, FromRow)]
pub struct SearchParameter {
    pub id: String,
    pub city: String,
    pub state: String,
    pub interest_points: String,
    pub enabled: bool,
}

#[derive(Debug, FromRow)]
pub struct Location {
    pub uuid: String,
    pub name: String,
    pub kind: String,
    pub latitude: i32,
    pub longitude: i32,
    pub street: String,
    pub neighbourhood: String,
    pub number: String,
    pub zip_code: String,
}

pub async fn init_db(env: Environment) -> MySqlPool {
    let pool = MySqlPool::connect(&env.database_url).await.unwrap();

    Migrator::new(Path::new("./migrations"))
        .await
        .unwrap()
        .run(&pool)
        .await
        .unwrap();

    pool
}

pub async fn load_search_parameters(pool: &MySqlPool) -> Result<Vec<SearchParameter>, Error> {
    let search_parameters =
        sqlx::query_as::<_, SearchParameter>("SELECT * FROM search_parameter WHERE enabled = true")
            .fetch_all(pool)
            .await?;

    Ok(search_parameters)
}
