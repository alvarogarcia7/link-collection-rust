use uuid::Uuid;

use domain::interfaces::record::RecordProvider;
use domain::interfaces::RecordProviderError;
pub(crate) use domain::Record;
use domain::RecordGrain;
use downloader::downloader::{FirebaseHackerNewsDownloader, NodeView};

use crate::cli_line_reader::{MyEditor, MyReadline};
use crate::date::{DateFormattable, DateFormatter, DateProvider};

pub struct FirebaseHackerNewsImporterProvider {
    pub line_reader: MyEditor,
    pub date_provider: DateProvider,
    pub downloader: FirebaseHackerNewsDownloader,
    id: u64,
}

impl FirebaseHackerNewsImporterProvider {}

impl FirebaseHackerNewsImporterProvider {
    pub fn new(
        line_reader: MyEditor,
        date_provider: DateProvider,
        downloader: FirebaseHackerNewsDownloader,
        id: u64,
    ) -> Self {
        Self {
            line_reader,
            date_provider,
            downloader,
            id,
        }
    }
}

impl RecordProvider for FirebaseHackerNewsImporterProvider {
    fn fetch(&mut self) -> Result<(Record, Vec<String>), RecordProviderError> {
        let id = Uuid::new_v4().to_string();
        let node_view = self.downloader.get_item(self.id);
        if node_view.is_err() {
            return Err(RecordProviderError::ErrorFetchingRecord);
        }

        let mut tags: Vec<String> = Vec::with_capacity(6);
        tags.append(&mut vec!["hackernews".to_string(), "imported".to_string()]);
        let view_unwrapped = node_view.unwrap();
        let time_ = view_unwrapped.time();
        let NodeView { by, title, url, .. } = view_unwrapped;

        tags.push(by);

        let date = DateFormatter::default().format(&time_);

        const FAKE: bool = false;

        let (body, category, tags) = if FAKE {
            let body = vec!["FAKE BODY".to_string()];
            let category = "FAKE CATEGORY".to_string();
            let tags = vec!["FAKE".to_string()];
            (body, category, tags)
        } else {
            let body = self.line_reader.read_until_ctrl_d("Body".to_string());
            let category = self
                .line_reader
                .read_line("Category (mandatory)".to_string());
            tags.append(&mut self.line_reader.read_until_ctrl_d("Tags".to_string()));
            (body, category, tags)
        };

        let field_values = vec![
            ("Id".to_string(), id),
            ("Date".to_string(), date.clone()),
            ("Link".to_string(), url),
            ("Title".to_string(), title),
            ("Body".to_string(), body.join("\n")),
            ("Category".to_string(), category),
            ("Tags".to_string(), tags.join(", ")),
        ];

        let mut fields: Vec<RecordGrain> = vec![];

        for (key, value) in field_values.iter() {
            fields.push(RecordGrain::new(key.clone(), value.clone()));
        }

        println!("{:?}", fields);

        Ok((
            Record {
                record_type: "Link".to_string(),
                fields,
            },
            vec![format!("VARIABLE;DATE;{}", date)],
        ))
    }
}
