//! script-wizard is a shell script (Bash) helper program, to delegate
//! the responsibility of asking questions to the user, asking for
//! confirmation, making selections, etc. The normalized response is
//! printed to stdout for the script to consume.
//!
//! This is a single binary application written in Rust and it
//! utilizes the
//! [inquire](https://docs.rs/inquire/latest/inquire/index.html)
//! library for providing a nicer user interface than the typical
//! shell script is capable of.
//!
//! # Source code
//! The source code is hosted at github [EnigmaCurry/script-wizard](https://github.com/EnigmaCurry/script-wizard)
//! # Install with cargo
//!
//! ```bash
//! cargo install script_wizard
//! ```
//!
//! # Install without cargo
//! [Grab the latest release from github](https://github.com/EnigmaCurry/script-wizard/releases) or run this command to download and install automatically:
//! ```bash
//! cd ~/Downloads
//! ARCHIVE=script-wizard-$(uname -s)-$(uname -m).tar.gz
//! curl -LO https://github.com/Enigmacurry/script-wizard/releases/latest/download/${ARCHIVE}
//! tar xfv ${ARCHIVE}
//! sudo install script-wizard /usr/local/bin/script-wizard
//! ```
//!
//! ## Examples in Bash
//! 
//! ### ask
//! 
//! Ask the user a question and capture the response:
//! 
//! ```bash
//! # Set the alias to make it easier to use:
//! alias ask='script-wizard ask'
//! 
//! # Record the user's response into the NAME variable:
//! NAME=$(ask "What is your name?")
//! ```
//! 
//! ### confirm
//! 
//! Ask the user a yes/no question, with a prepared default response (eg.
//! `yes` is the default here) :
//! 
//! ```bash
//! # Set the alias to make it easier to use:
//! alias confirm='script-wizard confirm'
//! 
//! # Confirm returns an exit code: 0=yes 1=no :
//! if confirm "Do you like Linux?" yes; then
//!   echo "Tux is great!"
//! else
//!   echo "Well, thats ok."
//! fi
//! 
//! # But maybe you want to record a literal "true" or "false" into a variable?:
//! LIKES_LINUX=$(confirm "Do you like Linux?" yes && echo "true" || echo "false")
//! ```
//! 
//! ### choose
//! 
//! Present a list of options to the user and have them select a *single*
//! response from the list:
//! 
//! ```bash
//! # Set the alias to make it easier to use:
//! alias choose='script-wizard choose'
//! 
//! CHOSEN=$(choose "Select your character class" "Rogue" "Wizard" "Paladin" "Cleric" "Bard")

//! # You can use an option from a bash array too:
//! options=("red" "blue" "greenish orange" "purple")
//! COLOR=$(choose "Choose a color" "${options[@]}")
//! ```
//! 
//! ### select
//! 
//! Present a list of options to the user and have them select *multiple*
//! responses (zero or more) from the list:
//! 
//! ```bash
//! readarray -t SELECTED < <(script-wizard select "Which games do you like?" "Rocket League" "Portal" "Quake" "Magic the Gathering")
//! 
//! echo "These are the games you said you like:"
//! # Use printf to print one per line (echo would merge into one line):
//! printf '%s\n' "${SELECTED[@]}"
//! ```
//! 
//! ### date
//! 
//! Present a date picker to the user:
//! 
//! ```bash
//! # Pick a date between 2023/10/01 and 2023/10/20:
//! DATE=$(script-wizard date "Enter a date" --week-start monday --format "%Y-%m-%d" --min-date "2023-10-01" --max-date "2023-10-20" --help-message "yadda yadda")
//! ```
//! 
//! ### editor
//! 
//! Present a full text editor entry to the user:
//! 
//! ```bash
//! BIOGRAPHY=$(script-wizard editor "Tell me alllll about yourself" --default "# Describe yourself" --json | sed 's/^[^\"]*//')
//! ```
//! 
//! Watch out: There is a potential bug here if your editor prints
//! anything to stdout. (In the case of emacsclient, it undesirably
//! captures the text "Waiting for Emacs...".) Using `--json` will wrap
//! the correct editor text in double quotes, and pipeing the output
//! through `sed 's/^[^\"]*//'` will remove the text before the first
//! double quote.)
//! 
//! Set the common `EDITOR` environment variable to choose the editor it
//! launches.
//! 
//! ### menu
//! 
//! Present a menu of command entries that the user can select and
//! execute. The entries must be specified in the format: `ENTRY =
//! COMMAND` where `ENTRY` is the text line of the menu entry, and
//! `COMMAND` is the shell command to run if the entry is selected:
//! 
//! ```bash
//! script-wizard menu --once "main menu" "print username = whoami"  "print all users = cat /etc/passwd | cut -d ':' -f 1"
//! ```
//!
//! ## Common options
//! 
//!  * `--json` - the default is to print raw text even if it spans
//!    multiple lines. If you specify `--json` it will print it as compact
//!    JSON on a single line, splitting lines into lists of strings, or as
//!    a quoted string if its just supposed to be one line, depending on
//!    the subcommand.
pub mod ask;
