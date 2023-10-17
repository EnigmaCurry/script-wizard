use clap::ValueEnum;
use inquire::{Confirm, InquireError, MultiSelect, Select, Text};

#[derive(Clone, ValueEnum)]
pub enum Confirmation {
    Yes,
    No,
}

pub fn ask_prompt(question: &str, default: &str, allow_blank: bool) {
    if question == "" {
        panic!("Blank question")
    }
    match allow_blank {
        true => println!(
            "{}",
            Text::new(question).with_default(default).prompt().unwrap()
        ),
        false => {
            let mut a = String::from("");
            while a == "" {
                a = Text::new(question).with_default(default).prompt().unwrap();
            }
            println!("{}", a)
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

pub fn select(question: &str, default: Vec<&str>, options: Vec<&str>) -> Vec<String> {
    let ans = MultiSelect::new(question, options).prompt();
    match ans {
        Ok(selection) => selection.iter().map(|&x| x.into()).collect(),
        Err(_) => panic!("Cancelled selection"),
    }
}
