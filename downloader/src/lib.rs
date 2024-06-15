use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct NodeView {
    pub id: u64,
    pub by: String,
    pub time: u64,
    pub title: String,
    pub url: String,
}

impl NodeView {}

struct FirebaseHackerNewsDownloader {
    domain: String,
}
impl FirebaseHackerNewsDownloader {
    fn new(domain: String) -> Self {
        FirebaseHackerNewsDownloader { domain }
    }

    fn get_item(&self, id: u64) -> Result<NodeView, reqwest::Error> {
        let response = reqwest::blocking::Client::new();
        let path = format!("{}/v0/item/{}.json", self.domain, id);
        let response = response
            .get(path)
            // .header(reqwest::header::USER_AGENT, "My Rust Application")
            .send()?;
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
    use super::*;

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
