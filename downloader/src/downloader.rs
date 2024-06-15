use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct NodeView {
    pub id: u64,
    pub by: String,
    time: u64,
    pub title: String,
    pub url: String,
}

impl NodeView {
    pub fn time(&self) -> DateTime<FixedOffset> {
        DateTime::<FixedOffset>::from(
            chrono::DateTime::from_timestamp(self.time as i64, 0).unwrap(),
        )
    }
}

pub struct FirebaseHackerNewsDownloader {
    domain: String,
}

impl FirebaseHackerNewsDownloader {
    pub fn new(domain: String) -> Self {
        FirebaseHackerNewsDownloader { domain }
    }

    pub fn get_item(&self, id: u64) -> Result<NodeView, reqwest::Error> {
        let response = reqwest::blocking::Client::new();
        let path = format!("{}/v0/item/{}.json", self.domain, id);
        let response = response
            .get(path)
            // .header(reqwest::header::USER_AGENT, "My Rust Application")
            .send();
        let response = response?;
        if response.status() != 200 {
            let error = response.error_for_status().unwrap_err();
            println!(
                "Download status is not OK. Status = {:?} at url = {:?}",
                error.status(),
                error.url().unwrap().as_str()
            );
            return Err(error);
        }
        response.json::<NodeView>()
    }
}

impl Default for FirebaseHackerNewsDownloader {
    fn default() -> Self {
        FirebaseHackerNewsDownloader {
            domain: "https://hacker-news.firebaseio.com".to_string(),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::downloader::{FirebaseHackerNewsDownloader, NodeView};

    #[test]
    fn fetch_and_parse_the_fields() {
        let downloader = FirebaseHackerNewsDownloader::new("http://0.0.0.0:8181".to_string());

        let response = downloader.get_item(12339182329);

        assert!(response.is_ok());

        let expected = NodeView {
            id: 27186675,
            by: "vanusa".to_string(),
            time: 1621276965,
            title: "Why Is the Gaza Strip Blurry on Google Maps?".to_string(),
            url: "https://www.bbc.com/news/57102499".to_string(),
        };

        assert_eq!(response.unwrap(), expected);
    }
}

#[cfg(test)]
pub mod date_tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_time() {
        let view = NodeView {
            id: 0,
            by: "".to_string(),
            // GMT: Monday, May 17, 2021 6:42:45 PM
            time: 1621276965,
            title: "".to_string(),
            url: "".to_string(),
        };

        assert_eq!(view.time, 1621276965);

        let expected_time = DateTime::<FixedOffset>::from_naive_utc_and_offset(
            NaiveDate::from_ymd_opt(2021, 5, 17)
                .unwrap()
                .and_hms_opt(18, 42, 45)
                .unwrap(),
            FixedOffset::east_opt(0).unwrap(),
        );

        assert_eq!(view.time(), expected_time);
    }
}
