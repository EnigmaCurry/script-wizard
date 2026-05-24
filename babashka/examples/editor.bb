#!/usr/bin/env bb

(require '[babashka.classpath :refer [add-classpath]])
(def script-dir (-> *file* io/file .getParentFile .getCanonicalPath))
(add-classpath (str script-dir "/../src"))

(require '[script-wizard :as wiz])

;; Basic editor
(def notes (wiz/editor "Enter your notes:"))

;; Editor with pre-filled text and markdown highlighting
(def readme (wiz/editor "Edit the README:"
                        :default "# My Project\n\nDescribe your project here."
                        :file-extension ".md"
                        :help-message "Write a short README for your project"))

;; --- summary ---
(println)
(println "=== Results ===")
(println "--- Notes ---")
(println notes)
(println)
(println "--- README ---")
(println readme)
