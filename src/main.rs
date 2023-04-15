use std::time::Duration;

use thirtyfour::{By, DesiredCapabilities, Key, WebDriver};

use crate::error::Error;

mod database;
mod environment;
mod error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = environment::load_env_vars();

    let db = database::init_db(env).await;

    let search_parameters = database::load_search_parameters(&db).await?;

    for search_parameter in search_parameters {
        let interest_points = search_parameter
            .interest_points
            .split(',')
            .collect::<Vec<&str>>();

        for interest_point in interest_points {
            perform_scrape(
                &search_parameter.city,
                &search_parameter.state,
                &interest_point,
            )
            .await
            .unwrap()
        }
    }

    Ok(())
}

async fn perform_scrape(city: &str, state: &str, interest_point: &str) -> Result<(), Error> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // normalization
    let interest_point = interest_point.replace(" ", "+");
    let city = city.replace(" ", "+");

    let url = format!("https://www.google.com/maps/search/{interest_point},+{city},+{state}");

    driver.goto(url).await?;

    let container = driver.find(By::ClassName("hfpxzc")).await?;
    container.click().await?;

    loop {
        let _ = tokio::time::sleep(Duration::from_secs(1)).await;

        let container = driver.find_all(By::ClassName("hfpxzc")).await?;

        container.last().unwrap().click().await?;

        driver
            .action_chain()
            .key_down(Key::PageDown)
            .perform()
            .await?;

        match driver.find(By::ClassName("HlvSq")).await {
            Ok(item) => {
                if item.text().await?.contains("final") {
                    break;
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        };
    }

    let elms = driver.find_all(By::ClassName("hfpxzc")).await?;

    let mut places = Vec::new();

    for e in elms {
        let interest_point = e.attr("aria-label").await?.unwrap();

        places.push(interest_point);
    }

    print!("{places:?}");

    driver.quit().await?;

    Ok(())
}
