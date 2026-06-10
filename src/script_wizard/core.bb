(ns script-wizard.core
  (:require [clojure.string :as str])
  (:import [org.jline.terminal TerminalBuilder Terminal Attributes]
           [org.jline.reader LineReaderBuilder LineReader LineReader$Option
            Completer Candidate EndOfFileException UserInterruptException]
           [org.jline.utils AttributedString AttributedStyle
            AttributedStringBuilder Display]))

;; ─── Terminal singleton ──────────────────────────────────────────────────────

(def ^:private terminal-atom (atom nil))

(defn- get-terminal
  "Return the shared JLine terminal, creating it on first use."
  ^Terminal []
  (or @terminal-atom
      (let [t (-> (TerminalBuilder/builder)
                  (.system true)
                  (.build))]
        (when (= "dumb" (.getType t))
          (.close t)
          (throw (ex-info "script-wizard requires an interactive terminal (got dumb)" {})))
        (.addShutdownHook (Runtime/getRuntime)
          (Thread. (fn [] (.close t))))
        (reset! terminal-atom t)
        t)))

(defn- cancel! []
  (throw (ex-info "canceled" {})))

;; ─── ask ─────────────────────────────────────────────────────────────────────

(defn ask
  "Prompt for free-text input. Returns the entered string.
   Options:
     :default      - pre-filled default value
     :suggestions  - vector of strings for tab-completion"
  [question & {:keys [default suggestions]}]
  (let [terminal (get-terminal)
        completer (when (seq suggestions)
                    (reify Completer
                      (complete [_ _reader line candidates]
                        (let [word (str (.word line))]
                          (doseq [s suggestions]
                            (when (or (str/blank? word)
                                      (str/includes?
                                        (str/lower-case (str s))
                                        (str/lower-case word)))
                              (.add candidates
                                (Candidate. (str s)))))))))
        builder (cond-> (doto (LineReaderBuilder/builder)
                          (.terminal terminal)
                          (.option LineReader$Option/DISABLE_EVENT_EXPANSION true)
                          (.option LineReader$Option/ERASE_LINE_ON_FINISH true))
                  completer (.completer completer))
        lr (.build builder)
        prompt (str question " ")]
    (loop []
      (let [result (try
                     (if default
                       (.readLine lr prompt nil nil (str default))
                       (.readLine lr prompt))
                     (catch UserInterruptException _ (cancel!))
                     (catch EndOfFileException _ (cancel!)))
            trimmed (str/trim result)]
        (if (str/blank? trimmed)
          (recur)
          trimmed)))))

;; ─── confirm ─────────────────────────────────────────────────────────────────

(defn confirm
  "Prompt for yes/no confirmation. Returns boolean.
   Options:
     :default - :yes or :no"
  [question & {:keys [default]}]
  (let [terminal (get-terminal)
        hint (case default
               :yes "[Y/n]"
               :no  "[y/N]"
               "[y/n]")
        lr (-> (doto (LineReaderBuilder/builder)
                 (.terminal terminal)
                 (.option LineReader$Option/DISABLE_EVENT_EXPANSION true)
                 (.option LineReader$Option/ERASE_LINE_ON_FINISH true))
               (.build))
        prompt (str question " " hint " ")]
    (loop []
      (let [input (try
                    (.readLine lr prompt)
                    (catch UserInterruptException _ (cancel!))
                    (catch EndOfFileException _ (cancel!)))
            answer (str/trim (str/lower-case input))]
        (cond
          (contains? #{"y" "yes"} answer)             true
          (contains? #{"n" "no"} answer)              false
          (and (str/blank? answer) (= default :yes))  true
          (and (str/blank? answer) (= default :no))   false
          :else                                       (recur))))))

;; ─── choose (raw terminal mode) ─────────────────────────────────────────────

(defn- build-header
  "Build the question header as an AttributedString."
  [question filter-text]
  (let [asb (AttributedStringBuilder.)]
    (.style asb (.foreground (.bold AttributedStyle/DEFAULT) AttributedStyle/GREEN))
    (.append asb "? ")
    (.style asb (.bold AttributedStyle/DEFAULT))
    (.append asb question)
    (when (seq filter-text)
      (.style asb AttributedStyle/DEFAULT)
      (.append asb (str " " filter-text)))
    (.toAttributedString asb)))

(defn- build-option-line
  "Build a single option line as an AttributedString."
  [item selected?]
  (let [asb (AttributedStringBuilder.)]
    (if selected?
      (do (.style asb (.foreground (.bold AttributedStyle/DEFAULT) AttributedStyle/CYAN))
          (.append asb (str "> " item)))
      (do (.style asb AttributedStyle/DEFAULT)
          (.append asb (str "  " item))))
    (.toAttributedString asb)))

(defn- compute-filtered
  "Filter options by the given filter text (case-insensitive contains)."
  [options filter-text]
  (if (str/blank? filter-text)
    (vec options)
    (vec (filter #(str/includes? (str/lower-case %) (str/lower-case filter-text))
                 options))))

(defn- read-with-timeout
  "Read one char from reader with timeout in ms. Returns -1 on timeout."
  [reader timeout-ms]
  (.read reader (long timeout-ms)))

(defn choose
  "Prompt to select one item from a list. Returns the selected string.
   Options:
     :default - option string to pre-select"
  [question options & {:keys [default]}]
  (let [terminal (get-terminal)
        saved    (.enterRawMode terminal)
        writer   (.writer terminal)
        reader   (.reader terminal)
        display  (Display. terminal false)
        options  (vec options)]
    (try
      (let [default-idx (if default
                          (let [idx (.indexOf options default)]
                            (if (neg? idx) 0 idx))
                          0)
            term-height (let [h (.getHeight terminal)] (if (pos? h) h 24))
            max-visible (min (count options) (max 1 (- term-height 2)))]
        (.resize display term-height (.getWidth terminal))
        (loop [cursor     default-idx
               filter-text ""
               filtered   options
               scroll-top 0]
          ;; Render
          (let [end    (min (count filtered) (+ scroll-top max-visible))
                visible (subvec filtered scroll-top end)
                lines  (into [(build-header question filter-text)]
                             (map-indexed
                               (fn [vi item]
                                 (build-option-line item (= (+ scroll-top vi) cursor)))
                               visible))]
            (.update display (java.util.ArrayList. lines) -1)
            (.flush writer)

            ;; Read key
            (let [ch (.read reader)]
              (cond
                ;; ESC or ESC sequence
                (= ch 27)
                (let [next (read-with-timeout reader 50)]
                  (if (= next -1)
                    ;; standalone ESC → cancel
                    (do (.update display (java.util.ArrayList.) -1)
                        (.flush writer)
                        (cancel!))
                    (if (= next 91) ;; CSI: ESC [
                      (let [arrow (.read reader)]
                        (cond
                          ;; Up
                          (= arrow 65)
                          (let [nc (max 0 (dec cursor))
                                ns (if (< nc scroll-top) nc scroll-top)]
                            (recur nc filter-text filtered ns))
                          ;; Down
                          (= arrow 66)
                          (let [nc (min (dec (count filtered)) (inc cursor))
                                ns (if (>= nc (+ scroll-top max-visible))
                                     (- nc (dec max-visible))
                                     scroll-top)]
                            (recur nc filter-text filtered ns))
                          :else
                          (recur cursor filter-text filtered scroll-top)))
                      ;; Unknown ESC sequence, ignore
                      (recur cursor filter-text filtered scroll-top))))

                ;; Enter
                (= ch 13)
                (if (empty? filtered)
                  (recur cursor filter-text filtered scroll-top)
                  (let [selected (nth filtered cursor)]
                    (.update display (java.util.ArrayList.) -1)
                    (.flush writer)
                    selected))

                ;; Ctrl-C
                (= ch 3)
                (do (.update display (java.util.ArrayList.) -1)
                    (.flush writer)
                    (cancel!))

                ;; Backspace
                (or (= ch 127) (= ch 8))
                (if (str/blank? filter-text)
                  (recur cursor filter-text filtered scroll-top)
                  (let [nf (subs filter-text 0 (dec (count filter-text)))
                        nfilt (compute-filtered options nf)]
                    (recur 0 nf nfilt 0)))

                ;; Printable character → filter
                (and (>= ch 32) (<= ch 126))
                (let [nf (str filter-text (char ch))
                      nfilt (compute-filtered options nf)]
                  (recur 0 nf nfilt 0))

                ;; Anything else → ignore
                :else
                (recur cursor filter-text filtered scroll-top))))))
      (finally
        (.setAttributes terminal saved)))))
