use serde_json::Value;
use std::error::Error;

use crate::utils::{get_query, get_response};

// Get the title of a Wikipedia category
//
// # Arguments
// * `value` - The value to get the title from
//
// # Returns
// * `Option<String>` - The title of the category
fn get_title(value: &Value) -> Option<String> {
    let prefix = "Category:";
    value
        .get("title")
        .and_then(|title| title.as_str())
        .map(|x| {
            if let Some(end) = x.strip_prefix(prefix) {
                end
            } else {
                x
            }
        })
        .map(|x| x.to_lowercase())
        .map(|x| x.to_string())
}

// Get the categories of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_categories(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=categories&titles={}&format=json",
        get_query(query).await
    );
    let output = get_response(&url).await?;
    let pages = output
        .get("pages")
        .unwrap()
        .as_object()
        .ok_or("no pages found")?;
    let item = pages.iter().next().ok_or("no item found")?;
    let categories_obj: &Value = item.1.get("categories").ok_or("no categories found")?;
    let categories_array: &Vec<Value> = categories_obj
        .as_array()
        .ok_or("unable to parse categories object to array")?;
    let output: Vec<String> = categories_array.iter().filter_map(get_title).collect();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_title() {
        let inputs = serde_json::json!({
            "title": "Category:Rust (programming language)",
        });
        let output = get_title(&inputs);
        let expected = Some("rust (programming language)".to_string());
        assert_eq!(output, expected);
    }

    #[tokio::test]
    async fn test_get_categories() {
        let output = get_categories("Rust").await.unwrap();
        assert!(output.len() > 0);
    }
}
