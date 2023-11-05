use crate::ask;
extern crate custom_error;
use custom_error::custom_error;
use std::process::Command;
use std::str::FromStr;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros;
custom_error! {pub ExampleError
    ExampleNotFound{name:String} = "Unknown example: {name}"
}

#[derive(
    strum_macros::EnumProperty,
    strum_macros::EnumString,
    strum_macros::EnumIter,
    strum_macros::Display,
    Debug,
)]
#[allow(dead_code)]
enum Example {
    #[strum(props(Name = "Ask into a variable"))]
    Ask,
    #[strum(props(Name = "Confirm inline"))]
    Confirm,
    #[strum(props(Name = "Confirm into variable true/false"))]
    ConfirmTrueFalse,
    #[strum(props(Name = "Choose a single option from an inline list"))]
    ChooseInline,
    #[strum(props(Name = "Choose a single option from a bash array"))]
    ChooseArray,
    #[strum(props(Name = "Select multiple options into a bash array"))]
    SelectArray,
    #[strum(props(Name = "Date picker between two dates"))]
    DatePick,
    #[strum(props(Name = "Editor for free form text box converted to JSON"))]
    Editor,
    #[strum(props(Name = "Menu of shell commands to execute"))]
    Menu,
}

pub fn choose_example() -> Result<String, ExampleError> {
    let options: Vec<&str> = Example::iter()
        .map(|e| e.get_str("Name").unwrap())
        .collect();
    let choice = ask::choose("Choose an example", "", options, &true).parse::<usize>();
    match choice {
        Ok(i) => match Example::iter().nth(i) {
            Some(ex) => example(ex.to_string().as_str()),
            None => panic!("invalid choice: not a valid example"),
        },
        Err(_) => panic!("invalid choice: not a number"),
    }
}

pub fn run_bash(src: &'static str) -> String {
    Command::new("/bin/bash")
        .args(["-c", src])
        .status()
        .unwrap();
    format!("\n{}", src)
}

pub fn example(name: &str) -> Result<String, ExampleError> {
    match name {
        "" => Ok(choose_example().unwrap().to_string()),
        s => match Example::from_str(s) {
            Ok(ex) => match ex {
                Example::Ask => Ok(run_bash(example_ask())),
                Example::Confirm => Ok(run_bash(example_confirm_inline())),
                Example::ConfirmTrueFalse => Ok(run_bash(example_confirm_true_false())),
                Example::ChooseInline => Ok(run_bash(example_choose_inline())),
                Example::ChooseArray => Ok(run_bash(example_choose_array())),
                Example::SelectArray => Ok(run_bash(example_select_array())),
                Example::DatePick => Ok(run_bash(example_date_pick())),
                Example::Editor => Ok(run_bash(example_editor())),
                Example::Menu => Ok(run_bash(example_menu())),
            },
            Err(_) => Err(ExampleError::ExampleNotFound {
                name: name.to_string(),
            }),
        },
    }
}

pub fn example_ask() -> &'static str {
    r#"#!/bin/bash
## Shorter alias for the script-wizard ask command:
ask() { script-wizard ask "$@"; }

## Ask a question and capture the response into the NAME variable:
NAME=$(ask "What is your name?")

echo "Hi there ${NAME}"
"#
}

pub fn example_confirm_inline() -> &'static str {
    r#"#!/bin/bash
## Shorter alias for the script-wizard confirm command:
confirm() { script-wizard confirm "$@"; }

## Ask a confirmation question with the default answer being yes:
if confirm "Do you like Linux?" yes; then
  echo "Tux is great!"
else
  echo "Well, thats ok."
fi
"#
}

pub fn example_confirm_true_false() -> &'static str {
    r#"#!/bin/bash
## Shorter alias for the script-wizard confirm command:
confirm() { script-wizard confirm "$@"; }

## Example confirm into a variable as literal true/false:
LIKES_LINUX=$(confirm "Do you like Linux?" yes && echo "true" || echo "false")

if [[ "${LIKES_LINUX}" == "true" ]]; then
    echo "$USER likes Linux"
else
    echo "$USER does not like Linux"
fi
"#
}

pub fn example_choose_inline() -> &'static str {
    r#"#!/bin/bash
## Shorter alias for script-wizard choose command:
choose() { script-wizard choose "$@"; }

## Example of choosing a single option from an inline list:
CHOSEN_CLASS=$(choose "Select your character class" \
                "Rogue" "Wizard" "Paladin" "Cleric" "Bard")

echo "Chosen class: ${CHOSEN_CLASS}"
"#
}

pub fn example_choose_array() -> &'static str {
    r#"#!/bin/bash
## Shorter alias for script-wizard choose command:
choose() { script-wizard choose "$@"; }

## Example of choosing a single option from a bash array:
options=("red" "blue" "greenish orange" "purple")
COLOR=$(choose "Choose a color" "${options[@]}")
echo "You chose ${COLOR}"
"#
}

pub fn example_select_array() -> &'static str {
    r#"#!/bin/bash
readarray -t SELECTED < <(script-wizard select "Which games do you like?" "Rocket League" "Portal" "Quake" "Magic the Gathering")

echo "These are the games you said you like, printed on separate lines:"
printf '%s\n' "${SELECTED[@]}"
echo
echo "Games you like, printed on one row: ${SELECTED[@]@Q}"
"#
}

pub fn example_date_pick() -> &'static str {
    r#"#!/bin/bash
# Pick a date between 2023/10/01 and 2023/10/20:
DATE=$(script-wizard date "Enter a date" --week-start monday --format "%Y-%m-%d" --min-date "2023-10-01" --max-date "2023-10-20" --help-message "yadda yadda")

echo "The date you chose: ${DATE}"
"#
}

pub fn example_editor() -> &'static str {
    r#"#!/bin/bash
BIOGRAPHY=$(script-wizard editor "Tell me alllll about yourself" --default "Describe yourself" --json | sed 's/^[^\"]*//')

echo "This is what you told me about yourself:"
echo "${BIOGRAPHY}"
"#
}

pub fn example_menu() -> &'static str {
    r#"#!/bin/bash
script-wizard menu "main menu" \
  "print username = whoami" \
  "print all users = cat /etc/passwd | cut -d ':' -f 1" \
  "exit = exit 2"
"#
}
