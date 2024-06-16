use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, EditMode, Editor};
use uuid::Uuid;

use domain::interfaces::record::RecordProvider;
use domain::interfaces::RecordProviderError;
pub(crate) use domain::Record;
use domain::RecordGrain;

use crate::date::{DateFormattable, DateFormatter, DateProvidable, DateProvider};
use domain::tags::import;

pub struct CliReaderRecordProvider {
    pub line_reader: MyEditor,
    pub date_provider: DateProvider,
}

impl CliReaderRecordProvider {}

impl CliReaderRecordProvider {
    pub fn new(
        line_reader: MyEditor,
        date_provider: DateProvider, /*id_provider: IdProvider*/
    ) -> Self {
        Self {
            line_reader,
            date_provider,
            // id_provider,
        }
    }
}

pub struct MyEditor {
    rl: Editor<(), FileHistory>,
}

impl Default for MyEditor {
    fn default() -> Self {
        let mut result = DefaultEditor::new().unwrap();
        result.set_edit_mode(EditMode::Vi);
        Self { rl: result }
    }
}

pub trait MyReadline {
    fn read_until_ctrl_d(&mut self, query: String) -> Vec<String>;
    fn read_line(&mut self, query: String) -> String;
}

impl MyReadline for MyEditor {
    fn read_until_ctrl_d(&mut self, query: String) -> Vec<String> {
        let mut lines = vec![];
        self.print_prompt(&format!("Type '{}' (CTRL-D to finish)", query));
        loop {
            let readline = self.read_line_raw(">> ");
            match readline {
                Ok(line_value) => {
                    // println!("Line: {}", line_value);
                    lines.push(line_value.trim().to_string());
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
        lines
    }

    fn read_line(&mut self, query: String) -> String {
        self.print_prompt(&format!("Type '{}' (Enter to finish)", query));
        loop {
            let readline = self.read_line_raw(">> ");
            match readline {
                Ok(line_value) => {
                    // println!("Line: {}", line_value);
                    return line_value.trim().to_string();
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                }
            }
        }
    }
}

impl MyEditor {
    fn print_prompt(&self, prompt: &str) {
        println!("{}", prompt);
    }

    fn read_line_raw(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        self.rl.readline(prompt)
    }
}

impl RecordProvider for CliReaderRecordProvider {
    fn fetch(&mut self) -> Result<Record, RecordProviderError> {
        let id = Uuid::new_v4().to_string();
        let formatted_date = DateFormatter::default().format(&self.date_provider.now());

        let field_values = vec![
            ("Id".to_string(), id),
            ("Date".to_string(), formatted_date),
            (
                "Link".to_string(),
                self.line_reader.read_line("Url".to_string()),
            ),
            (
                "Title".to_string(),
                self.line_reader.read_line("Title (mandatory)".to_string()),
            ),
            (
                "Body".to_string(),
                self.line_reader
                    .read_until_ctrl_d("Body".to_string())
                    .join("\n"),
            ),
            (
                "Category".to_string(),
                self.line_reader
                    .read_line("Category (mandatory)".to_string()),
            ),
            (
                "Tags".to_string(),
                import(
                    self.line_reader
                        .read_until_ctrl_d("Tags (one per line or separated by comma)".to_string()),
                )
                .values
                .join(", "),
            ),
        ];

        let mut fields: Vec<RecordGrain> = vec![];

        for (key, value) in field_values.iter() {
            fields.push(RecordGrain::new(key.clone(), value.clone()));
        }

        Ok(Record {
            record_type: "Link".to_string(),
            fields,
        })
    }
}
