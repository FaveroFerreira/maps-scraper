#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("gmaps error: {0:?}")]
    Gmaps(#[from] thirtyfour::error::WebDriverError),

    #[error("database error: {0:?}")]
    Database(#[from] sqlx::error::Error),
}
