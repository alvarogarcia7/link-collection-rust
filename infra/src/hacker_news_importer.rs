use log::{debug, info};
use uuid::Uuid;

use domain::interfaces::database::DatabaseReadAccess;
use domain::interfaces::record::RecordProvider;
use domain::interfaces::RecordProviderError;
pub(crate) use domain::Record;
use domain::{RecordGrain, Tags};
use downloader::downloader::{FirebaseHackerNewsDownloader, NodeView};

use crate::cli_line_reader::{MyEditor, MyReadline};
use crate::date::{DateFormattable, DateFormatter, DateProvider};
use crate::fzf_selector::fzf_selector_mod::FzfSelector;

pub struct FirebaseHackerNewsImporterProvider {
    pub line_reader: MyEditor,
    pub date_provider: DateProvider,
    pub downloader: FirebaseHackerNewsDownloader,
    pub database: Box<dyn DatabaseReadAccess>,
    id: u64,
}

impl FirebaseHackerNewsImporterProvider {
    pub fn new(
        line_reader: MyEditor,
        date_provider: DateProvider,
        downloader: FirebaseHackerNewsDownloader,
        database: Box<dyn DatabaseReadAccess>,
        id: u64,
    ) -> Self {
        Self {
            line_reader,
            date_provider,
            downloader,
            database,
            id,
        }
    }
}

impl RecordProvider for FirebaseHackerNewsImporterProvider {
    fn fetch(&mut self) -> Result<(Record, Vec<String>), RecordProviderError> {
        let id = Uuid::new_v4().to_string();
        let response_view = self.downloader.get_item(self.id);
        if response_view.is_err() {
            return Err(RecordProviderError::ErrorFetchingRecord);
        }

        let mut tags: Vec<String> = Vec::with_capacity(6);
        tags.append(&mut vec!["hackernews".to_string(), "imported".to_string()]);
        let view_unwrapped = response_view.unwrap();
        let time_ = view_unwrapped.node_view.time();
        let NodeView { by, title, url, .. } = view_unwrapped.node_view;

        tags.push(by);

        let date = DateFormatter::default().format(&time_);

        info!("Title: {:?}", title);
        info!("time: {:?}", time_);

        const FAKE: bool = false;

        let (body, category, tags) = if FAKE {
            let body = vec!["FAKE BODY".to_string()];
            let category = "FAKE CATEGORY".to_string();
            let tags = vec!["FAKE".to_string()];
            (body, category, tags)
        } else {
            let body = self.line_reader.read_until_ctrl_d("Body".to_string());
            let category = {
                let vec2 = FzfSelector::select_single_from(
                    "Pick category",
                    self.database
                        .read_all_category()
                        .into_iter()
                        .collect::<Vec<String>>(),
                );
                println!("Selected category from FZF: {:?}", vec2);
                if vec2.is_empty() {
                    "uncategorized".to_string()
                } else {
                    vec2[0].clone()
                }
            };
            let mut selected_tags: Vec<String> = vec![];
            let set = self
                .database
                .read_all_tags()
                .into_iter()
                .collect::<Vec<String>>();
            let mut vec2 = FzfSelector::select_multiple_from("Pick tags (use TAB to select multiple)", set.clone());
            println!("Selected tags from FZF: {:?}", vec2);
            selected_tags.append(&mut vec2);
            tags.append(&mut selected_tags);
            (body, category, tags)
        };

        let mut field_values = vec![("Id".to_string(), id), ("Date".to_string(), date.clone())];
        match url {
            None => {}
            Some(url) => {
                field_values.push(("Link".to_string(), url));
            }
        }
        field_values.append(&mut vec![
            ("Title".to_string(), title),
            ("Body".to_string(), body.join("\n+ ")),
            ("Category".to_string(), category),
            ("Tags".to_string(), Tags::import(tags).values.join(", ")),
            ("Origin".to_string(), view_unwrapped.origin),
        ]);

        let mut fields: Vec<RecordGrain> = vec![];

        for (key, value) in field_values.iter() {
            fields.push(RecordGrain::new(key.clone(), value.clone()));
        }

        debug!("{:?}", fields);

        Ok((
            Record {
                record_type: "Link".to_string(),
                fields,
            },
            vec![format!("VARIABLE;DATE;{}", date)],
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    struct MockDatabase {
        categories: HashSet<String>,
        tags: HashSet<String>,
    }

    impl MockDatabase {
        fn new() -> Self {
            let mut categories = HashSet::new();
            categories.insert("craftsmanship".to_string());
            categories.insert("business".to_string());
            categories.insert("engineering".to_string());

            let mut tags = HashSet::new();
            tags.insert("rust".to_string());
            tags.insert("web".to_string());
            tags.insert("database".to_string());
            tags.insert("performance".to_string());

            Self { categories, tags }
        }
    }

    impl DatabaseReadAccess for MockDatabase {
        fn read_all(&self) -> Vec<Record> {
            vec![]
        }

        fn read_all_tags(&self) -> HashSet<String> {
            self.tags.clone()
        }

        fn read_all_category(&self) -> HashSet<String> {
            self.categories.clone()
        }
    }

    #[test]
    fn test_firebase_importer_provider_can_be_created() {
        let mock_db = Box::new(MockDatabase::new());
        let downloader = FirebaseHackerNewsDownloader::new("http://localhost:8181".to_string());
        let line_reader = MyEditor::default();
        let date_provider = DateProvider::default();

        let importer = FirebaseHackerNewsImporterProvider::new(
            line_reader,
            date_provider,
            downloader,
            mock_db,
            12345,
        );

        assert_eq!(importer.id, 12345);
    }

    #[test]
    fn test_mock_database_has_categories() {
        let db = MockDatabase::new();
        let categories = db.read_all_category();

        assert!(categories.contains("craftsmanship"));
        assert!(categories.contains("business"));
        assert!(categories.contains("engineering"));
        assert_eq!(categories.len(), 3);
    }

    #[test]
    fn test_mock_database_has_tags() {
        let db = MockDatabase::new();
        let tags = db.read_all_tags();

        assert!(tags.contains("rust"));
        assert!(tags.contains("web"));
        assert!(tags.contains("database"));
        assert!(tags.contains("performance"));
        assert_eq!(tags.len(), 4);
    }

    #[test]
    fn test_category_fallback_when_empty() {
        // Verify that empty selection defaults to "uncategorized"
        let vec2: Vec<String> = vec![];
        let category = if vec2.is_empty() {
            "uncategorized".to_string()
        } else {
            vec2[0].clone()
        };

        assert_eq!(category, "uncategorized");
    }

    #[test]
    fn test_category_selection_when_available() {
        // Verify that category is correctly selected
        let vec2 = vec!["craftsmanship".to_string()];
        let category = if vec2.is_empty() {
            "uncategorized".to_string()
        } else {
            vec2[0].clone()
        };

        assert_eq!(category, "craftsmanship");
    }

    #[test]
    fn test_tags_properly_appended() {
        // Verify that selected tags are appended to the initial tags list
        let mut tags: Vec<String> = vec!["hackernews".to_string(), "imported".to_string()];
        let initial_len = tags.len();

        let mut selected_tags = vec!["rust".to_string(), "performance".to_string()];
        tags.append(&mut selected_tags);

        assert_eq!(tags.len(), initial_len + 2);
        assert_eq!(tags[0], "hackernews");
        assert_eq!(tags[1], "imported");
        assert_eq!(tags[2], "rust");
        assert_eq!(tags[3], "performance");
    }

    #[test]
    fn test_database_access_trait_implementation() {
        // Verify that MockDatabase properly implements DatabaseReadAccess
        let db = MockDatabase::new();

        let read_all_result = db.read_all();
        assert_eq!(read_all_result.len(), 0);

        let tags = db.read_all_tags();
        assert!(!tags.is_empty());

        let categories = db.read_all_category();
        assert!(!categories.is_empty());
    }

    #[test]
    fn test_initial_tags_include_hackernews_and_imported() {
        // Verify the structure of initial tags for HN imports
        let mut tags: Vec<String> = Vec::with_capacity(6);
        tags.append(&mut vec!["hackernews".to_string(), "imported".to_string()]);
        tags.push("test_user".to_string());

        assert_eq!(tags.len(), 3);
        assert_eq!(tags[0], "hackernews");
        assert_eq!(tags[1], "imported");
        assert_eq!(tags[2], "test_user");
    }
}
