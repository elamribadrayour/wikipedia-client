use std::{collections::HashMap, error::Error};

use crate::utils::{get_query, get_query_response};

// Get search results from Wikipedia for a query
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_search(query: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json",
        get_query(query).await
    );

    let response = get_query_response(&url).await?;

    let search_results = response
        .get("search")
        .ok_or("unable to find search results")?
        .as_array()
        .ok_or("unable to parse search results to array")?;

    let output = search_results
        .iter()
        .map(|x| {
            let title = x
                .get("title")
                .ok_or("unable to get title")
                .unwrap()
                .as_str()
                .ok_or("title is not a string")
                .unwrap()
                .to_string();

            let snippet_html = x
                .get("snippet")
                .ok_or("unable to get snippet")
                .unwrap()
                .as_str()
                .ok_or("snippet is not a string")
                .unwrap();

            let snippet = html2text::from_read(snippet_html.as_bytes(), 100)?;

            let pageid = x
                .get("pageid")
                .ok_or("unable to get pageid")?
                .as_i64()
                .ok_or("pageid is not a number")
                .unwrap()
                .to_string();

            Ok(HashMap::from([
                ("title".to_string(), title),
                ("snippet".to_string(), snippet),
                ("pageid".to_string(), pageid),
            ]))
        })
        .collect::<Result<Vec<HashMap<String, String>>, Box<dyn Error>>>()?;

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search() {
        let output = get_search("Rust").await.unwrap();
        assert!(output.len() > 0);
    }
}
