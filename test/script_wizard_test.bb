#!/usr/bin/env bb

;; Interactive test for script-wizard babashka port.
;; Run: bb -cp src test/script_wizard_test.bb

(require '[script-wizard.core :as wiz])

(println "=== script-wizard babashka port — interactive test ===")
(println)

;; --- ask ---

(println "── ask tests ──")

(def name-val
  (try (wiz/ask "What is your name?" :default "World")
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → name: " name-val))

(def lang
  (try (wiz/ask "Favorite language?"
                :suggestions ["Clojure" "Ruby" "Bash" "Rust" "Python" "Go" "Haskell" "Elixir"])
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → language: " lang))

(println)

;; --- confirm ---

(println "── confirm tests ──")

(def likes-coffee?
  (try (wiz/confirm "Do you like coffee?")
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → coffee: " likes-coffee?))

(def wants-updates?
  (try (wiz/confirm "Want updates?" :default :yes)
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → updates: " wants-updates?))

(def is-robot?
  (try (wiz/confirm "Are you a robot?" :default :no)
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → robot: " is-robot?))

(println)

;; --- choose ---

(println "── choose tests ──")

(def color
  (try (wiz/choose "Pick a color:" ["red" "green" "blue" "yellow" "purple"])
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → color: " color))

(def editor
  (try (wiz/choose "Preferred editor:" ["vim" "emacs" "vscode" "helix"] :default "vim")
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → editor: " editor))

(def backend
  (try (wiz/choose "Backend:" ["libvirt" "proxmox"])
       (catch Exception e
         (if (= "canceled" (ex-message e))
           (do (println "  (canceled)") nil)
           (throw e)))))
(println (str "  → backend: " backend))

(println)
(println "=== All tests complete ===")
