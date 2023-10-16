# script-wizard

script-wizard is a shell script helper program to delegate the
responsibility of asking the user questions, confirmations,
selections, etc, and printing the normalized response for the script to consume.

This is a single-binary Rust application and utilizes the
[inquire](https://docs.rs/inquire/latest/inquire/index.html) library
for providing a nicer user interface than the typical shell script is
capable of.

## Examples

### ask

Ask the user a question and capture the response:

```
NAME=$(script-wizard ask "What is your name?")
```

### confirm

Ask the user a yes/no question, with a prepared default response (eg.
`yes` is the default here) :

```
ANSWER=$(script-wizard confirm "Do you like Linux?" yes)
```

### select

Present a list of options to the user and have them select a single
response from the list:

```
SELECTED=$(script-wizard select "Choose your character class" "Rogue" "Wizard" "Paladin" "Cleric" "Bard")
```
