#!/usr/bin/env bb

(require '[babashka.classpath :refer [add-classpath]])
(def script-dir (-> *file* io/file .getParentFile .getCanonicalPath))
(add-classpath (str script-dir "/../src"))

(require '[script-wizard :as wiz])

(defn settings-menu []
  (wiz/menu "Settings"
    [["Theme"  #(let [t (wiz/choose "Pick a theme:" ["dark" "light" "solarized"])]
                  (println (str "Theme set to: " t)))]
     ["Language" #(let [l (wiz/choose "Pick a language:" ["en" "es" "fr" "de"])]
                    (println (str "Language set to: " l)))]
     ["Back"   nil]]))

(wiz/menu "Main Menu"
  [["Greet"    #(let [name (wiz/ask "What is your name?")]
                  (println (str "Hello, " name "!")))]
   ["Settings" settings-menu]
   ["Quit"     nil]])
