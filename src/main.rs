use std::time::Duration;

use thirtyfour::{prelude::WebDriverResult, By, DesiredCapabilities, Key, WebDriver};

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    let caps = DesiredCapabilities::firefox();
    let driver = WebDriver::new("http://localhost:4444", caps).await?;

    // system imput
    let city = "Passo Fundo";
    let state = "RS";
    let interest_point = "Hotel";

    // normalization
    let interest_point = interest_point.replace(" ", "+");
    let city = city.replace(" ", "+");

    let url = format!("https://www.google.com/maps/search/{interest_point},+{city},+{state}");

    driver.goto(url).await?;

    let container = driver.find(By::ClassName("hfpxzc")).await?;
    container.click().await?;

    loop {
        let _ = tokio::time::sleep(Duration::from_secs(3)).await;

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
