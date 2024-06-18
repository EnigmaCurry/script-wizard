#!/bin/bash
cargo build --release
script-wizard() {
    ./target/release/script-wizard "$@"
}

NAME=$(script-wizard ask "What is your name?")
echo "Hello, ${NAME}"


if script-wizard confirm "Do you like Linux?" yes; then
  echo "Tux is great!"
else
  echo "Well, thats ok."
fi


CHOSEN=$(script-wizard choose "Select your character class" "Rogue" "Wizard" "Paladin" "Cleric" "Bard")
echo "You chose: ${CHOSEN}"

# You can use an option from a bash array too:
options=("red" "blue" "greenish orange" "purple")
COLOR=$(script-wizard choose "Choose a color" "${options[@]}")
echo "Your color: ${COLOR}"

readarray -t SELECTED < <(script-wizard select "Which games do you like?" "Rocket League" "Portal" "Quake" "Magic the Gathering")
 
echo "These are the games you said you like:"
# Use printf to print one per line (echo would merge into one line):
printf '%s\n' "${SELECTED[@]}"

# Pick a date between 2023/10/01 and 2023/10/20:
DATE=$(script-wizard date "Enter a date" --week-start monday --format "%Y-%m-%d" --min-date "2023-10-01" --max-date "2023-10-20" --help-message "yadda yadda")
echo "Date selected: ${DATE}"


script-wizard menu --once \
              "main menu" \
              "print username = whoami"  \
              "print all users = cat /etc/passwd | cut -d ':' -f 1"
