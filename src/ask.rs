use clap::ValueEnum;
use inquire::{Confirm, InquireError, Select, Text};

#[derive(Clone, ValueEnum)]
pub enum Confirmation {
    Yes,
    No,
}

pub fn ask(question: &str) {
    if question == "" {
        panic!("Blank question")
    }
    let status = Text::new(question).prompt();
    println!("{}", status.unwrap())
}

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

pub fn select(question: &str, options: Vec<&str>, separator: &str) -> u16 {
    let ans: Result<&str, InquireError> = Select::new(question, options).prompt();
    16
}
