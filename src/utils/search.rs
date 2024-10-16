use std::{collections::HashMap, error::Error};

use crate::utils::{get_query, get_response};

// Get the categories of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn search(query: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",
        get_query(query).await
    );

    let response = get_response(&url).await?;

    let search_results = response
        .get("search")
        .ok_or("unable to find search results")
        .unwrap()
        .as_array()
        .ok_or("unable to parse search results to array")
        .unwrap();

    let output = search_results
        .iter()
        .map(|x| {
            HashMap::from([
                (
                    "title".to_string(),
                    x.get("title")
                        .ok_or("unable to get title")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
                (
                    "snippet".to_string(),
                    x.get("snippet")
                        .ok_or("unable to get snippet")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
                (
                    "pageid".to_string(),
                    x.get("pageid")
                        .ok_or("unable to get page id")
                        .unwrap()
                        .as_i64()
                        .unwrap()
                        .to_string(),
                ),
                (
                    "timestamp".to_string(),
                    x.get("timestamp")
                        .ok_or("unable to get timestamp")
                        .unwrap()
                        .as_str()
                        .unwrap()
                        .to_string(),
                ),
            ])
        })
        .collect::<Vec<HashMap<String, String>>>();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search() {
        let output = search("Rust").await.unwrap();
        assert!(output.len() > 0);
    }
}
