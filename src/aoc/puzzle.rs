use reqwest::StatusCode;
use serde::Deserialize;
use std::error::Error;

/// Unique identifier for a puzzle, consists of (year, day)
pub type PuzzleKey = (usize, usize);

/// Data associated with a puzzle
#[derive(Debug, Clone, Deserialize)]
pub struct PuzzleDetails {
    /// Name of the puzzle
    name: String,
}

pub async fn fetch_puzzle_details(
    client: &reqwest::Client,
    year: usize,
    day: usize,
) -> Result<PuzzleDetails, Box<dyn Error>> {
    // Fetch
    let res = client
        .get(format!("https://adventofcode.com/{year}/day/{day}"))
        .send()
        .await?;

    // Check that the request was successfull
    if res.status() == StatusCode::NOT_FOUND {
        return Err("No such puzzle found".to_owned().into());
    }

    // Parse the html response
    todo!()
}
