use std::process::Command;

use chrono::{NaiveDate, Weekday};
use clap::ValueEnum;
use inquire::{
    autocompletion::Replacement, error::CustomUserError, Confirm, DateSelect, Editor, InquireError,
    MultiSelect, Select, Text,
};

#[derive(Clone, ValueEnum)]
pub enum Confirmation {
    Yes,
    No,
}

fn read_json_array(json: &str) -> Result<Vec<String>, CustomUserError> {
    let a: Vec<String> = serde_json::from_str(json).expect("invalid json array");
    Ok(a)
}

#[derive(Clone, Default)]
pub struct AskAutoCompleter {
    input: String,
    suggestions_json: String,
    suggestions: Vec<String>,
    suggestion_index: usize,
}

impl AskAutoCompleter {
    fn update_input(&mut self, input: &str) -> Result<(), CustomUserError> {
        if input == self.input {
            // No change:
            return Ok(());
        }
        self.input = input.to_string();
        self.suggestion_index = 0;
        Ok(())
    }
}

impl inquire::Autocomplete for AskAutoCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        self.update_input(input)?;
        self.suggestions = read_json_array(&self.suggestions_json)
            .expect("Couldn't parse suggestions")
            .iter()
            .filter(|s| s.to_lowercase().contains(&input.to_lowercase()))
            .map(|s| String::from(s.clone()))
            .collect();
        Ok(self.suggestions.clone())
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        self.update_input(input)?;
        match highlighted_suggestion {
            Some(suggestion) => Ok(Replacement::Some(suggestion)),
            None => {
                if self.suggestions.len() > 0 {
                    self.suggestion_index = (self.suggestion_index + 1) % self.suggestions.len();
                    Ok(Replacement::Some(
                        self.suggestions
                            .get(self.suggestion_index)
                            .unwrap()
                            .to_string(),
                    ))
                } else {
                    Ok(Replacement::None)
                }
            }
        }
    }
}

pub fn ask_prompt(
    question: &str,
    default: &str,
    allow_blank: bool,
    suggestions_json: &str,
) -> String {
    if question == "" {
        panic!("Blank question")
    }
    let mut auto_completer = AskAutoCompleter::default();
    auto_completer.suggestions_json = suggestions_json.to_string();
    match allow_blank {
        true => {
            let r: Result<String, InquireError>;
            match default {
                "" => {
                    r = Text::new(question)
                        .with_autocomplete(auto_completer.clone())
                        .prompt();
                }
                _ => {
                    r = Text::new(question)
                        .with_autocomplete(auto_completer.clone())
                        .with_default(default)
                        .prompt();
                }
            }
            if r.is_err() {
                std::process::exit(1);
            }
            r.unwrap()
        }
        false => {
            let mut a = String::from("");
            while a == "" {
                let r: Result<String, InquireError>;
                match default {
                    "" => {
                        r = Text::new(question)
                            .with_autocomplete(auto_completer.clone())
                            .prompt();
                    }
                    _ => {
                        r = Text::new(question)
                            .with_default(default)
                            .with_autocomplete(auto_completer.clone())
                            .prompt();
                    }
                }
                if r.is_err() {
                    std::process::exit(1);
                }
                a = r.unwrap();
            }
            a
        }
    }
}

macro_rules! ask {
    ($question: expr, $default: expr, $allow_blank: expr, $suggestions_json: expr) => {
        ask::ask_prompt($question, $default, $allow_blank, $suggestions_json)
    };
    ($question: expr, $default: expr, $allow_blank: expr) => {
        ask::ask_prompt($question, $default, $allow_blank, "")
    };
    ($question: expr, $default: expr) => {
        ask::ask_prompt($question, $default, false, "")
    };
    ($question: expr) => {
        ask::ask_prompt($question, "", false, "")
    };
}
pub(crate) use ask;

pub fn confirm(question: &str, default_answer: Option<Confirmation>) -> bool {
    let mut c = Confirm::new(question);
    match default_answer {
        Some(Confirmation::Yes) => c = c.with_default(true),
        Some(Confirmation::No) => c = c.with_default(false),
        _ => (),
    }
    match c.prompt() {
        Ok(true) => true,
        Ok(false) => false,
        Err(_) => false,
    }
}

pub fn choose(question: &str, default: &str, options: Vec<&str>, numeric: &bool) -> String {
    let default_index = options.iter().position(|&r| r == default).unwrap_or(0);
    let ans: Result<&str, InquireError> = Select::new(question, options.clone())
        .with_starting_cursor(default_index)
        .prompt();
    match ans {
        Ok(selection) => match numeric {
            true => {
                let index = options.iter().position(|&r| r == selection).unwrap();
                format!("{}", index)
            }
            false => String::from(selection),
        },
        Err(_) => std::process::exit(1),
    }
}

pub fn select(question: &str, default: &str, options: Vec<&str>) -> Vec<String> {
    let defaults: Vec<&str> = serde_json::from_str(default).unwrap_or(vec![]);
    let mut default_indices = vec![];
    for (index, item) in options.iter().enumerate() {
        match defaults.iter().find(|&r| r == item) {
            Some(_) => default_indices.append(&mut vec![index]),
            None => {}
        };
    }
    let ans = MultiSelect::new(question, options)
        .with_default(&default_indices)
        .prompt();
    match ans {
        Ok(selection) => selection.iter().map(|&x| x.into()).collect(),
        Err(_) => std::process::exit(1),
    }
}

pub fn date(
    question: &str,
    default: &str,
    min_date: &str,
    max_date: &str,
    starting_date: &str,
    week_start: Weekday,
    help_message: &str,
    date_format: &str,
) -> String {
    let date = DateSelect::new(question)
        .with_starting_date(
            NaiveDate::parse_from_str(default, date_format)
                .unwrap_or(chrono::Local::now().naive_local().into()),
        )
        .with_min_date(NaiveDate::parse_from_str(min_date, date_format).unwrap_or(NaiveDate::MIN))
        .with_max_date(NaiveDate::parse_from_str(max_date, date_format).unwrap_or(NaiveDate::MAX))
        .with_starting_date(
            NaiveDate::parse_from_str(starting_date, date_format).unwrap_or(
                NaiveDate::parse_from_str(min_date, date_format).unwrap_or(NaiveDate::MIN),
            ),
        )
        .with_week_start(week_start)
        .with_help_message(help_message)
        .prompt()
        .unwrap();
    return date.format(date_format).to_string();
}

pub fn editor(message: &str, default: &str, help_message: &str, file_extension: &str) -> String {
    let text = Editor::new(message)
        .with_predefined_text(default)
        .with_help_message(help_message)
        .with_file_extension(file_extension)
        .prompt()
        .unwrap();
    return text;
}

pub fn menu(heading: &str, entries: &Vec<String>, default: &Option<String>) -> Result<usize, u8> {
    let titles: Vec<&str> = entries
        .iter()
        .map(|e| e.split(" = ").collect::<Vec<&str>>()[0])
        .collect();
    let commands: Vec<&str> = entries
        .iter()
        .map(|e| e.split(" = ").collect::<Vec<&str>>()[1])
        .collect();
    let command_index = choose(
        heading,
        default.clone().unwrap_or(String::from("")).as_str(),
        titles,
        &true,
    )
    .parse::<usize>()
    .unwrap_or(1);
    // Run the command:
    let cmd = commands[command_index];
    let status = Command::new("/bin/bash")
        .args(["-c", cmd])
        .status()
        .unwrap();

    match status.code().unwrap_or(1) {
        0 => Ok(0),
        _ => Err(1),
    }
}
