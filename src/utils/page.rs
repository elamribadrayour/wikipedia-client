use std::collections::HashMap;
use std::error::Error;

use crate::utils::{get_query, get_query_response};

pub async fn get_page(
    text: &str,
    input_type: &str,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let input_type = input_type.to_lowercase();
    if input_type != "pageid" && input_type != "title" {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "invalid input type: {}, input type must be either 'pageId' or 'titles'",
                input_type
            ),
        )));
    }

    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=revisions&rvslots=main&rvprop=content&format=json&{}s={}",
        input_type, get_query(text).await
    );
    let response = get_query_response(&url).await?;
    let pages = response.get("pages").ok_or("unable to find pages")?;
    let pages_object = pages.as_object().ok_or("unable to parse pages")?;
    let item = pages_object.iter().next().ok_or("unable to find page")?;
    let revisions = item.1.get("revisions").ok_or("unable to find revisions")?;
    let revisions_array = revisions.as_array().ok_or("unable to parse revisions")?;
    let revision = revisions_array
        .iter()
        .next()
        .ok_or("unable to find revision")?;
    let slots = revision.get("slots").ok_or("unable to find slots")?;
    let slots_object = slots.as_object().ok_or("unable to parse slots")?;
    let main = slots_object.get("main").ok_or("unable to find main")?;
    let main_string = main.get("*").ok_or("unable to find main content")?;
    Ok(HashMap::from([(
        "content".to_string(),
        main_string.to_string(),
    )]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_page_by_title() {
        let output = get_page("Rust", "title").await.unwrap();
        assert!(output.len() > 0);
        assert!(output.get("content").unwrap().len() > 0);
    }

    #[tokio::test]
    async fn test_get_page_by_page_id() {
        let output = get_page("39270413", "pageId").await.unwrap();
        assert!(output.len() > 0);
        assert!(output.get("content").unwrap().len() > 0);
    }
}
