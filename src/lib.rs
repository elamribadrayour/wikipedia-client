use chrono::NaiveDate;
use std::collections::HashMap;
use std::error::Error;

mod utils;

// List pages similar to a given search query on Wikipedia
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn search(query: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    utils::get_search(query).await
}

// Get page content from a Wikipedia page by title
//
// # Arguments
// * `query` - The title of the page
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_page_by_title(query: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    utils::get_page(query, "title").await
}

// Get page content from a Wikipedia page by page id
//
// # Arguments
// * `query` - The page id
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_page_by_id(query: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    utils::get_page(query, "pageId").await
}

// Get media files from a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_images(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    utils::get_images(query).await
}

// Get the categories of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_categories(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    utils::get_categories(query).await
}

// Get the links of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_links(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    utils::get_links(query).await
}

// Get the languages of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_languages(query: &str) -> Result<Vec<String>, Box<dyn Error>> {
    utils::get_languages(query).await
}

// Get the views of a Wikipedia page
//
// # Arguments
// * `query` - The search query
//
// # Returns
// * `Result<Value, Box<dyn Error>>` - The result of the search
pub async fn get_views(
    query: &str,
    start_date: &str,
    nb_days: i64,
) -> Result<HashMap<NaiveDate, i64>, Box<dyn Error>> {
    utils::get_views(query, start_date, nb_days).await
}
