(ns script-wizard
  (:require [babashka.process :refer [shell]]
            [cheshire.core :as json]
            [clojure.string :as str]))

(defn- check-installed! []
  (when-not (= 0 (:exit (shell {:out :string :err :string :continue true} "which" "script-wizard")))
    (binding [*out* *err*]
      (println "Error: script-wizard is not installed.")
      (println "Get it from https://github.com/enigmacurry/script-wizard"))
    (System/exit 1)))

(check-installed!)

(defn ask
  "Ask a free-text question. Returns the user's response string."
  [question & {:keys [default allow-blank suggestions]}]
  (let [args (cond-> ["script-wizard" "ask" question]
               default (conj default)
               allow-blank (conj "--allow-blank")
               suggestions (conj "--suggestions" (json/generate-string suggestions)))]
    (-> (apply shell {:out :string} args)
        :out
        str/trim)))

(defn confirm
  "Ask a yes/no question. Returns true or false."
  [question & {:keys [default]}]
  (let [args (cond-> ["script-wizard" "confirm" question]
               default (conj (name default)))]
    (-> (apply shell {:continue true} args)
        :exit
        (= 0))))

(defn choose
  "Choose one item from a list. Returns the chosen string."
  [question options & {:keys [default]}]
  (let [args (cond-> (into ["script-wizard" "choose" question] options)
               default (conj "--default" default))]
    (-> (apply shell {:out :string} args)
        :out
        str/trim)))

(defn select
  "Select multiple items from a list. Returns a vector of chosen strings."
  [question options & {:keys [default]}]
  (let [args (cond-> (into ["script-wizard" "select" question] options)
               default (conj "--default" (json/generate-string default)))]
    (-> (apply shell {:out :string} args)
        :out
        str/trim
        str/split-lines)))

(defn date
  "Pick a date interactively. Returns a date string.
   Options:
     :default       - default date string
     :format        - date format string (default \"%Y-%m-%d\")
     :min-date      - minimum selectable date
     :max-date      - maximum selectable date
     :starting-date - initial cursor date
     :week-start    - day the week starts on (e.g. \"sunday\", \"monday\")
     :help-message  - help text shown below the picker"
  [question & {:keys [default format min-date max-date starting-date week-start help-message]}]
  (let [args (cond-> ["script-wizard" "date" question]
               default       (conj "--default" default)
               format        (conj "--format" format)
               min-date      (conj "--min-date" min-date)
               max-date      (conj "--max-date" max-date)
               starting-date (conj "--starting-date" starting-date)
               week-start    (conj "--week-start" week-start)
               help-message  (conj "--help-message" help-message))]
    (-> (apply shell {:out :string} args)
        :out
        str/trim)))

(defn editor
  "Open a full text editor for input. Returns the entered text.
   Options:
     :default        - pre-filled text in the editor
     :help-message   - help text shown to the user
     :file-extension - file extension for syntax highlighting (e.g. \".md\")"
  [message & {:keys [default help-message file-extension]}]
  (let [args (cond-> ["script-wizard" "editor" message]
               default        (conj "--default" default)
               help-message   (conj "--help-message" help-message)
               file-extension (conj "--file-extension" file-extension))]
    (-> (apply shell {:out :string} args)
        :out
        str/trim)))

(defn menu
  "Interactive menu that loops until quit. This is a re-implementation of
   `script-wizard menu` built on top of `choose`. The original menu command
   was designed for Bash and runs shell commands as subprocesses. This version
   dispatches to Clojure functions instead, making it natural for Babashka
   scripts to build hierarchical menus via recursive calls.

   Entries are [label handler] pairs. A nil handler exits the menu.
   Options:
     :once    - exit after the first selection instead of looping
     :default - default choice label"
  [heading entries & {:keys [once default]}]
  (let [labels (mapv first entries)]
    (loop [dflt default]
      (let [choice (choose heading labels :default dflt)
            handler (second (first (filter #(= (first %) choice) entries)))]
        (if (nil? handler)
          nil
          (do (handler)
              (if once
                nil
                (recur choice))))))))
