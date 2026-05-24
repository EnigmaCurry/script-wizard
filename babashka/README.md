# script-wizard for Babashka

[script-wizard](https://github.com/enigmacurry/script-wizard) is an
interactive terminal prompt tool for shell scripts. It provides
commands for asking questions, confirming actions, choosing from lists,
and selecting multiple items — with features like autocompletion,
default values, and date pickers. Responses are printed to stdout for
your script to consume.

This directory contains a [Babashka](https://github.com/babashka/babashka)
wrapper library that lets you use script-wizard from Clojure scripts
with a native data structure API.

## Prerequisites

- [Babashka](https://github.com/babashka/babashka) installed
- [script-wizard](https://github.com/enigmacurry/script-wizard) installed
  and on your `PATH`

## Usage

From the `babashka/` directory:

```clojure
#!/usr/bin/env bb

(require '[script-wizard :as wiz])

;; Ask a free-text question
(wiz/ask "What is your name?")
(wiz/ask "City?" :default "New York")
(wiz/ask "Nickname?" :allow-blank true)
(wiz/ask "Language?" :suggestions ["Clojure" "Rust" "Python"])

;; Yes/no confirmation
(wiz/confirm "Continue?")
(wiz/confirm "Continue?" :default :yes)
(wiz/confirm "Continue?" :default :no)

;; Choose one from a list
(wiz/choose "Pick one:" ["a" "b" "c"])
(wiz/choose "Pick one:" ["a" "b" "c"] :default "b")

;; Select multiple from a list
(wiz/select "Pick many:" ["a" "b" "c"])
(wiz/select "Pick many:" ["a" "b" "c"] :default ["a" "c"])

;; Pick a date
(wiz/date "Start date:")
(wiz/date "Deadline:" :min-date "2026-01-01" :max-date "2026-12-31"
                      :week-start "monday" :help-message "Must be within 2026")

;; Full text editor
(wiz/editor "Enter notes:")
(wiz/editor "Edit README:" :default "# My Project" :file-extension ".md")

;; Interactive menu (re-implemented in Clojure, dispatches to functions
;; instead of shell commands like the original Bash-oriented `script-wizard menu`)
(wiz/menu "Main Menu"
  [["Greet"    #(println "Hello!")]
   ["Settings" #(wiz/menu "Settings"
                  [["Theme" #(println "dark")]
                   ["Back"  nil]])]
   ["Quit"     nil]])
```

## Examples

- [test.bb](examples/test.bb) — all prompt types (ask, confirm, choose, select, date)
- [editor.bb](examples/editor.bb) — full text editor input
- [menu.bb](examples/menu.bb) — hierarchical menu with submenus

## Using the library in your own project

Copy `src/script_wizard.clj` into your project's source path and add
`{:paths ["src"]}` (or wherever you place it) to your `bb.edn`.
