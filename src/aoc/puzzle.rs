use reqwest::StatusCode;
use serde::Deserialize;
use std::error::Error;

use crate::format::generate_puzzle_url;

/// Unique identifier for a puzzle, consists of (year, day)
pub type PuzzleKey = (usize, usize);

/// Data associated with a puzzle
#[derive(Debug, Clone, Deserialize)]
pub struct PuzzleDetails {
    /// Name of the puzzle
    pub name: String,
}

pub async fn fetch_puzzle_details(
    client: &reqwest::Client,
    year: usize,
    day: usize,
) -> Result<PuzzleDetails, Box<dyn Error>> {
    // Fetch puzzle page
    let res = client.get(generate_puzzle_url(year, day)).send().await?;

    // Check that the request was successfull
    if res.status() == StatusCode::NOT_FOUND {
        return Err("No such puzzle found".to_owned().into());
    }

    // Parse the html response
    let html = res.text().await?;
    let dom = tl::parse(&html, tl::ParserOptions::default())?;
    let parser = dom.parser();

    // Get the heading name from the <h2/>
    let heading_handle = dom
        .query_selector("h2")
        .expect("Should succesfully parse the query selector")
        .next()
        .ok_or("Couldn't find heading element")?;
    let heading = heading_handle
        .get(parser)
        .expect("Handle to resolve to element");
    let heading_text = heading.inner_text(parser);

    // Extract puzzle name from heading text
    let name = heading_text;
    let (_, name) = name.split_once(':').expect("Heading text to include a :");
    let (name, _) = name
        .split_once('-')
        .expect("Heading text to include a - after the :");
    let name = name.trim();

    Ok(PuzzleDetails {
        name: name.to_owned(),
    })
}

#[cfg(test)]
mod test {
    use super::fetch_puzzle_details;
    use reqwest::Client;

    #[tokio::test]
    async fn test_fetch_puzzle_details() {
        let client = Client::new();
        let puzzle = fetch_puzzle_details(&client, 2022, 1).await.unwrap();
        assert_eq!(puzzle.name, "Calorie Counting");
    }
}
