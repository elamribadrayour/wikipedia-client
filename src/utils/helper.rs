use serde_json::Value;

use std::error::Error;

pub async fn get_query(query: &str) -> String {
    urlencoding::encode(query).to_string()
}

pub async fn get_response(url: &str) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("failed to fetch results from wikipedia for url: {}", url),
        )));
    }
    let output: Value = response.json().await?;
    Ok(output)
}

pub async fn get_query_response(url: &str) -> Result<Value, Box<dyn Error>> {
    let response = get_response(url).await?;
    let query = response
        .get("query")
        .ok_or("unable to find query in response")?;
    Ok(query.clone())
}
