use std::error::Error;
use tokio::task;

use crate::utils::get_query_response;

async fn get_content_by_id(page_id: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let url = format!(
        "https://en.wikipedia.org/w/api.php?action=query&prop=revisions&rvslots=main&rvprop=content&format=json&pageids={}",
        page_id
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
    Ok(main_string.to_string())
}

pub async fn get_content(
    page_ids: Vec<String>,
) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
    let mut handles = Vec::new();

    for id in page_ids {
        let handle = task::spawn(async move { get_content_by_id(&id).await });
        handles.push(handle);
    }

    let results = futures::future::join_all(handles).await;

    let mut outputs = Vec::new();
    for result in results {
        match result {
            Ok(Ok(output)) => outputs.push(output),
            Ok(Err(e)) => return Err(e), // Error from the async function
            Err(e) => return Err(Box::new(e)), // Error from the task itself
        }
    }

    Ok(outputs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_content_by_title() {
        let output = get_content_by_id("39270413").await.unwrap();
        assert!(output.len() > 0);
    }

    #[tokio::test]
    async fn test_get_content() {
        let output = get_content(vec!["39270413".to_string(), "39270414".to_string()])
            .await
            .unwrap();
        assert!(output.len() == 2);
    }
}
