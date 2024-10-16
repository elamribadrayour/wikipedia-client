use serde_json::Value;
use std::error::Error;

use crate::utils::{get_query, get_query_response};

// Get the title of a Wikipedia image
//
// # Arguments
// * `value` - The value to get the title from
//
// # Returns
// * `Option<String>` - The title of the image
fn get_title(value: &Value) -> Option<String> {
    let prefix = "File:";
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

// Get the images of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_images(query: &str) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=images&titles={}&format=json",
        get_query(query).await
    );
    let output = get_query_response(&url).await?;
    let pages = output
        .get("pages")
        .unwrap()
        .as_object()
        .ok_or("no pages found")?;
    let item = pages.iter().next().ok_or("no item found")?;
    let images_obj: &Value = item.1.get("images").ok_or("no images found")?;
    let images_array: &Vec<Value> = images_obj
        .as_array()
        .ok_or("unable to parse images object to array")?;
    let output: Vec<String> = images_array.iter().filter_map(get_title).collect();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_title() {
        let inputs = serde_json::json!({
            "title": "File:Collapsed Kinzua Bridge.jpg",
        });
        let output = get_title(&inputs);
        let expected = Some("collapsed kinzua bridge.jpg".to_string());
        assert_eq!(output, expected);
    }

    #[tokio::test]
    async fn test_get_images() {
        let output = get_images("Rust").await.unwrap();
        assert!(output.len() > 0);
    }
}
