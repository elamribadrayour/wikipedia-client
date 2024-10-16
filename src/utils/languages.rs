use serde_json::Value;
use std::error::Error;

use crate::utils::{get_query, get_response};

// Get the language of a Wikipedia page
//
// # Arguments
// * `value` - The value to get the language from
//
// # Returns
// * `Option<String>` - The language of the page
fn get_lang(value: &Value) -> Option<String> {
    value
        .get("lang")
        .and_then(|lang| lang.as_str())
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
pub async fn get_languages(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=langlinks&titles={}&format=json",
        get_query(query).await
    );
    let output = get_response(&url).await?;
    let pages = output
        .get("pages")
        .unwrap()
        .as_object()
        .ok_or("no pages found")?;
    let item = pages.iter().next().ok_or("no item found")?;
    let langlinks_obj: &Value = item.1.get("langlinks").ok_or("no langlinks found")?;
    let langlinks_array: &Vec<Value> = langlinks_obj
        .as_array()
        .ok_or("unable to parse langlinks object to array")?;
    let output: Vec<String> = langlinks_array.iter().filter_map(get_lang).collect();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_languages() {
        let output = get_languages("Rust").await.unwrap();
        assert!(output.len() > 0);
    }

    #[tokio::test]
    async fn test_get_lang() {
        let inputs = serde_json::json!({
            "lang": "FR",
        });
        let output = get_lang(&inputs);
        let expected = Some("fr".to_string());
        assert_eq!(output, expected);
    }
}
