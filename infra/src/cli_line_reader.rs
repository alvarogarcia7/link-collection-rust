use log::{info, warn};
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, EditMode, Editor};
use uuid::Uuid;

use crate::date::{DateFormattable, DateFormatter, DateProvidable, DateProvider};
use crate::fzf_selector::fzf_selector::FzfSelector;
use domain::interfaces::database::DatabaseReadAccess;
use domain::interfaces::record::RecordProvider;
use domain::interfaces::RecordProviderError;
pub(crate) use domain::Record;
use domain::{RecordGrain, Tags};

pub struct CliReaderRecordProvider {
    pub line_reader: MyEditor,
    pub date_provider: DateProvider,
    pub database: Box<dyn DatabaseReadAccess>,
}

impl CliReaderRecordProvider {}

impl CliReaderRecordProvider {
    pub fn new(
        line_reader: MyEditor,
        date_provider: DateProvider, /*id_provider: IdProvider*/
        database: Box<dyn DatabaseReadAccess>,
    ) -> Self {
        Self {
            line_reader,
            date_provider,
            // id_provider,
            database,
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

impl MyEditor {
    pub fn handle(
        &mut self,
        readline: Result<String, ReadlineError>,
    ) -> Result<String, ReadlineError> {
        match readline {
            Ok(line) => Ok(line),
            Err(ReadlineError::Interrupted) => {
                warn!("CTRL-C");
                readline
            }
            Err(ReadlineError::Eof) => {
                warn!("CTRL-D");
                readline
            }
            Err(err) => {
                warn!("Error: {:?}", err);
                Err(err)
            }
        }
    }
}

pub trait MyReadline {
    fn read_until_ctrl_d(&mut self, query: String) -> Vec<String>;
    fn read_until_ctrl_d_with_ret_code(&mut self, query: String) -> (Vec<String>, i32);
    fn read_line(&mut self, query: String) -> String;
    fn read_line_with_initial(&mut self, query: String, initial: (&str, &str)) -> String;
}

impl MyReadline for MyEditor {
    fn read_until_ctrl_d_with_ret_code(&mut self, query: String) -> (Vec<String>, i32) {
        let mut lines = vec![];
        self.print_prompt(&format!("Type '{}' (CTRL-D to finish)", query));
        let mut ret: i32 = 0;
        loop {
            let readline = self.read_line_raw(">> ");
            match readline {
                Ok(line_value) => {
                    // println!("Line: {}", line_value);
                    lines.push(line_value.trim().to_string());
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    ret = 1;
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    ret = 2;
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    ret = 3;
                }
            }
        }
        (lines, ret)
    }
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
                    warn!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    warn!("CTRL-D");
                    break;
                }
                Err(err) => {
                    warn!("Error: {:?}", err);
                }
            }
        }
        lines
    }

    fn read_line(&mut self, query: String) -> String {
        self.print_prompt(&format!("Type '{}' (Enter to finish)", query));
        loop {
            let readline = self.read_line_raw(">> ");
            if let Ok(line_value) = self.handle(readline) {
                return line_value;
            }
        }
    }
    fn read_line_with_initial(&mut self, query: String, initial: (&str, &str)) -> String {
        self.print_prompt(&format!("Type '{}' (Enter to finish)", query));
        loop {
            let readline = self.read_line_raw_with_initial(">> ", initial);
            if let Ok(line_value) = self.handle(readline) {
                return line_value.trim().to_string();
            };
        }
    }
}

impl MyEditor {
    fn print_prompt(&self, prompt: &str) {
        info!("{}", prompt);
    }

    fn read_line_raw(&mut self, prompt: &str) -> Result<String, ReadlineError> {
        self.rl.readline(prompt)
    }

    fn read_line_raw_with_initial(
        &mut self,
        prompt: &str,
        initial: (&str, &str),
    ) -> Result<String, ReadlineError> {
        self.rl.readline_with_initial(prompt, initial)
    }
}

impl RecordProvider for CliReaderRecordProvider {
    fn fetch(&mut self) -> Result<(Record, Vec<String>), RecordProviderError> {
        let id = Uuid::new_v4().to_string();
        let formatted_date = DateFormatter::default().format(&self.date_provider.now());

        let field_values = [
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
            ("Tags".to_string(), {
                let mut vec1: Vec<String> = vec![];
                loop {
                    let set = self
                        .database
                        .read_all_tags()
                        .into_iter()
                        .collect::<Vec<String>>();
                    let mut vec2 = FzfSelector::select_multiple_from(
                        "Pick tags (use TAB to select multiple)",
                        set.clone(),
                    );
                    println!("Selected tags from FZF: {:?}", vec2);
                    vec1.append(&mut vec2);
                    let mut x = self.line_reader.read_until_ctrl_d_with_ret_code(
                        "Tags (one per line or separated by comma) (CTRL-C to skip)".to_string(),
                    );
                    vec1.append(&mut x.0);
                    if x.1 == 2 {
                        break;
                    }
                }

                vec1.dedup();

                let tags = Tags::import(vec1);
                println!("Selected tags: {:?}", tags.values);

                tags.values.join(", ")
            }),
        ];

        let mut fields: Vec<RecordGrain> = vec![];

        for (key, value) in field_values.iter() {
            fields.push(RecordGrain::new(key.clone(), value.clone()));
        }

        Ok((
            Record {
                record_type: "Link".to_string(),
                fields,
            },
            vec![],
        ))
    }
}
