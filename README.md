# script-wizard

`script-wizard` is a shell script helper program to delegate the
responsibility of asking questions to the user, asking for
confirmation, making selections, etc. The normalized response is
printed to stdout for the script to consume.

This is a single binary application written in Rust and it utilizes
the [inquire](https://docs.rs/inquire/latest/inquire/index.html)
library for providing a nicer user interface than the typical shell
script is capable of.

## Install

The latest release is found on the [releases
page](https://github.com/EnigmaCurry/script-wizard/releases). Make
sure to download the archive for the correct system architecture
(x86_64 for intel or amd systems, aarch64 for arm64).

If your system architecture is not supported, you can try to build it
yourself:

 * You will need to install stable Rust according to
[rustup](https://rustup.rs/) (includes `cargo` command).

Build and install `script-wizard` binary:

```
cargo install --git https://github.com/EnigmaCurry/script-wizard
```

`script-wizard` has only been tested on Linux so far. Please open an
issue and let me know if you'd like support for any different
architecture.

## Examples in Bash

### ask

Ask the user a question and capture the response:

```
NAME=$(script-wizard ask "What is your name?")
```

### confirm

Ask the user a yes/no question, with a prepared default response (eg.
`yes` is the default here) :

```
# Confirm returns an exit code: 0=yes 1=no :
if script-wizard confirm "Do you like Linux?" yes; then
  echo "Tux is great!"
else
  echo "Well, thats ok."
fi

# But maybe instead you want to record a literal "true" or "false" into a variable?:
LIKES_LINUX=$(script-wizard confirm "Do you like Linux?" yes && echo "true" || echo "false")
```

### select

Present a list of options to the user and have them select a single
response from the list:

```
SELECTED=$(script-wizard select "Choose your character class" "Rogue" "Wizard" "Paladin" "Cleric" "Bard")

# You can use an option from a bash array:
options=("red" "blue" "greenish orange" "purple")
COLOR=$(script-wizard select "Choose a color" "${options[@]}")
```
