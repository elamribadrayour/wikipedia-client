use serde_json::Value;
use std::error::Error;

use crate::utils::{get_query, get_response};

// Get the title of a Wikipedia link
//
// # Arguments
// * `value` - The value to get the title from
//
// # Returns
// * `Option<String>` - The title of the link
fn get_title(value: &Value) -> Option<String> {
    value
        .get("title")
        .and_then(|title| title.as_str())
        .map(|x| x.to_lowercase())
        .map(|x| x.to_string())
}

// Get the links of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_links(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=links&titles={}&format=json",
        get_query(query).await
    );
    let output = get_response(&url).await?;
    let pages = output
        .get("pages")
        .unwrap()
        .as_object()
        .ok_or("no pages found")?;
    let item = pages.iter().next().ok_or("no item found")?;
    let links_obj: &Value = item.1.get("links").ok_or("no links found")?;
    let links_array: &Vec<Value> = links_obj
        .as_array()
        .ok_or("unable to parse links object to array")?;
    let output: Vec<String> = links_array.iter().filter_map(get_title).collect();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_title() {
        let inputs = serde_json::json!({
            "title": "Air moisture",
        });
        let output = get_title(&inputs);
        let expected = Some("air moisture".to_string());
        assert_eq!(output, expected);
    }

    #[tokio::test]
    async fn test_get_links() {
        let output = get_links("Rust").await.unwrap();
        assert!(output.len() > 0);
    }
}
