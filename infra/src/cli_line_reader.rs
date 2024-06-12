use domain::interfaces::record::RecordProvider;
use domain::Record;
use rustyline::config::Configurer;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{DefaultEditor, EditMode, Editor};
use std::collections::HashMap;

pub struct CliReaderRecordProvider {
    pub line_reader: MyEditor,
}

impl CliReaderRecordProvider {}

impl CliReaderRecordProvider {
    pub fn new(line_reader: MyEditor) -> Self {
        Self { line_reader }
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

trait MyReadline {
    fn read_until_ctrl_d(&mut self, query: String) -> Vec<String>;
    fn read_line(&mut self, query: String) -> String;
}

impl MyReadline for MyEditor {
    fn read_until_ctrl_d(&mut self, query: String) -> Vec<String> {
        let mut lines = vec![];
        self.print_prompt(&format!("Type '{}' (CTRL-D to finish)", query));
        loop {
            let readline = self.read_line_raw(">>");
            match readline {
                Ok(line_value) => {
                    println!("Line: {}", line_value);
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
            let readline = self.read_line_raw(">>");
            match readline {
                Ok(line_value) => {
                    println!("Line: {}", line_value);
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
    fn fetch(&mut self) -> Record {
        let id = "a1a6925a-7958-11e8-a87f-0242ac110002".to_string();
        let formatted_date = "Tue, 26 Jun 2018 15:50:21 +0000".to_string();
        let category = "category1".to_string();
        let formatted_tags = "tag1, tag_2, name-surname".to_string();

        let fields_dto = vec![
            ("Id".to_string(), id),
            ("Date".to_string(), formatted_date),
            ("Category".to_string(), category),
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
            ("Tags".to_string(), formatted_tags),
        ];

        let mut fields: HashMap<String, String> = HashMap::with_capacity(fields_dto.len());

        for (key, value) in fields_dto.iter() {
            fields.insert(key.clone(), value.clone());
        }

        Record {
            record_type: "Link".to_string(),
            fields,
            fields_dto,
        }
    }
}
