use chrono::{Duration, NaiveDate};
use std::collections::HashMap;
use std::error::Error;

use crate::utils::{get_query, get_response};

pub async fn get_views(
    query: &str,
    start_date: &str,
    nb_days: i64,
) -> Result<HashMap<NaiveDate, i64>, Box<dyn Error>> {
    let dt_start = NaiveDate::parse_from_str(start_date, "%Y%m%d")
        .expect("start_date argument should be in format YYYYMMDD");
    let end_date = (dt_start + Duration::days(nb_days - 1))
        .format("%Y%m%d")
        .to_string();

    let url = format!("https://wikimedia.org/api/rest_v1/metrics/pageviews/per-article/en.wikipedia/all-access/all-agents/{}/daily/{}/{}",
        get_query(query).await,
        start_date.to_string() + "00",
        end_date.to_string() + "00"
    );
    let response = get_response(&url).await?;
    let items = response.get("items").unwrap().as_array().unwrap();
    let output = items
        .iter()
        .map(|x| {
            let timestamp = x.get("timestamp").unwrap().as_str().unwrap();
            let dt_timestamp = NaiveDate::parse_from_str(&timestamp[..8], "%Y%m%d").unwrap();
            let views = x.get("views").unwrap().as_i64().unwrap();
            (dt_timestamp, views)
        })
        .collect();

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_views() {
        let output = get_views("Barack_Obama", "20240801", 10).await.unwrap();
        assert_eq!(output.len(), 10);
    }
}
