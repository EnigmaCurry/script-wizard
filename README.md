# script-wizard

`script-wizard` is a shell script helper program to delegate the
responsibility of asking questions to the user, asking for
confirmation, making selections, etc. The normalized response is
printed to stdout for the script to consume.

This is a single binary application that utilizes the
[inquire](https://docs.rs/inquire/latest/inquire/index.html) library
for providing a nicer user interface than the typical shell script is
capable of.

## Platforms

Pre-built binaries are available for:

| OS | Architecture |
|---|---|
| Linux | x86_64, aarch64 |
| Windows | x86_64, aarch64 |
| macOS | x86_64, aarch64 |

## Install

### Install with cargo

```bash
cargo install script_wizard
```

### Install with Nix

```bash
nix profile install github:EnigmaCurry/script-wizard
```

### Install from GitHub releases

[Grab the latest release from GitHub](https://github.com/EnigmaCurry/script-wizard/releases) or run this command to download and install automatically:

```bash
cd ~/Downloads
ARCHIVE=script-wizard-$(uname -s)-$(uname -m).tar.gz
curl -LO https://github.com/Enigmacurry/script-wizard/releases/latest/download/${ARCHIVE}
tar xfv ${ARCHIVE}
sudo install script-wizard /usr/local/bin/script-wizard
```

Or, install with this curlbomb:

```bash
bash <(curl https://raw.githubusercontent.com/EnigmaCurry/script-wizard/refs/heads/master/install.sh) ~/.local/bin
```

## Examples in Bash

### ask

Ask the user a question and capture the response:

```bash
# Set the alias to make it easier to use:
alias ask='script-wizard ask'

# Record the user's response into the NAME variable:
NAME=$(ask "What is your name?")
```

### confirm

Ask the user a yes/no question, with a prepared default response (eg.
`yes` is the default here):

```bash
# Set the alias to make it easier to use:
alias confirm='script-wizard confirm'

# Confirm returns an exit code: 0=yes 1=no :
if confirm "Do you like Linux?" yes; then
  echo "Tux is great!"
else
  echo "Well, thats ok."
fi

# But maybe you want to record a literal "true" or "false" into a variable?:
LIKES_LINUX=$(confirm "Do you like Linux?" yes && echo "true" || echo "false")
```

### choose

Present a list of options to the user and have them select a *single*
response from the list:

```bash
# Set the alias to make it easier to use:
alias choose='script-wizard choose'

CHOSEN=$(choose "Select your character class" "Rogue" "Wizard" "Paladin" "Cleric" "Bard")

# You can use an option from a bash array too:
options=("red" "blue" "greenish orange" "purple")
COLOR=$(choose "Choose a color" "${options[@]}")
```

### select

Present a list of options to the user and have them select *multiple*
responses (zero or more) from the list:

```bash
readarray -t SELECTED < <(script-wizard select "Which games do you like?" "Rocket League" "Portal" "Quake" "Magic the Gathering")

echo "These are the games you said you like:"
# Use printf to print one per line (echo would merge into one line):
printf '%s\n' "${SELECTED[@]}"
```

### date

Present a date picker to the user:

```bash
# Pick a date between 2023/10/01 and 2023/10/20:
DATE=$(script-wizard date "Enter a date" --week-start monday --format "%Y-%m-%d" --min-date "2023-10-01" --max-date "2023-10-20" --help-message "yadda yadda")
```

### editor

Present a full text editor entry to the user:

```bash
BIOGRAPHY=$(script-wizard editor "Tell me alllll about yourself" --default "# Describe yourself" --json | sed 's/^[^\"]*//')
```

Watch out: There is a potential bug here if your editor prints
anything to stdout. (In the case of emacsclient, it undesirably
captures the text "Waiting for Emacs...".) Using `--json` will wrap
the correct editor text in double quotes, and piping the output
through `sed 's/^[^\"]*//'` will remove the text before the first
double quote.

Set the common `EDITOR` environment variable to choose the editor it
launches.

### menu

Present a menu of command entries that the user can select and
execute. The entries must be specified in the format: `ENTRY =
COMMAND` where `ENTRY` is the text line of the menu entry, and
`COMMAND` is the shell command to run if the entry is selected:

```bash
script-wizard menu --once "main menu" "print username = whoami"  "print all users = cat /etc/passwd | cut -d ':' -f 1"
```

## Babashka pod

script-wizard can run as a [Babashka](https://github.com/babashka/babashka)
[pod](https://github.com/babashka/pods), providing a native Clojure API
for all commands (`ask`, `confirm`, `choose`, `select`, `date`, `editor`,
and `menu`):

```clojure
(require '[babashka.pods :as pods])
(pods/load-pod ["script-wizard" "pod"])
(require '[pod.enigmacurry.script-wizard :as sw])

(sw/ask "What is your name?" :default "World")
(sw/confirm "Continue?" :default :yes)
(sw/choose "Pick one" ["a" "b" "c"])
```

## Common options

 * `--json` - the default is to print raw text even if it spans
   multiple lines. If you specify `--json` it will print it as compact
   JSON on a single line, splitting lines into lists of strings, or as
   a quoted string if its just supposed to be one line, depending on
   the subcommand.

## Documentation

[Full API documentation on docs.rs](https://docs.rs/script-wizard)
