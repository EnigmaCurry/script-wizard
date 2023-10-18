use chrono::{NaiveDate, Weekday};
use clap::ValueEnum;
use inquire::{Confirm, DateSelect, Editor, InquireError, MultiSelect, Select, Text};

#[derive(Clone, ValueEnum)]
pub enum Confirmation {
    Yes,
    No,
}

pub fn ask_prompt(question: &str, default: &str, allow_blank: bool) -> String {
    if question == "" {
        panic!("Blank question")
    }
    match allow_blank {
        true => Text::new(question).with_default(default).prompt().unwrap(),
        false => {
            let mut a = String::from("");
            while a == "" {
                a = Text::new(question).with_default(default).prompt().unwrap();
            }
            a
        }
    }
}

macro_rules! ask {
    ($question: expr, $default: expr, $allow_blank: expr) => {
        ask::ask_prompt($question, $default, $allow_blank)
    };
    ($question: expr, $default: expr) => {
        ask::ask_prompt($question, $default, false)
    };
    ($question: expr) => {
        ask::ask_prompt($question, "", false)
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
        Err(_) => panic!("Error with confirm"),
    }
}

pub fn choose(question: &str, default: &str, options: Vec<&str>) -> String {
    let default_index = options.iter().position(|&r| r == default).unwrap_or(0);
    let ans: Result<&str, InquireError> = Select::new(question, options)
        .with_starting_cursor(default_index)
        .prompt();
    match ans {
        Ok(selection) => String::from(selection),
        Err(_) => panic!("Cancelled selection"),
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
        Err(_) => panic!("Cancelled selection"),
    }
}

pub fn date(
    question: &str,
    default: &str,
    min_date: &str,
    max_date: &str,
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
