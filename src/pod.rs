use bendy::decoding::{Decoder, Object};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::io::{self, Read, Write};
use std::process::{Command, Stdio};

const NAMESPACE: &str = "pod.enigmacurry.script-wizard";

fn read_bencode_message(reader: &mut impl Read) -> Option<HashMap<String, String>> {
    // Read all available bytes and try to decode a bencode dict
    let mut buf = Vec::new();
    let mut byte = [0u8; 1];

    // Bencode dicts start with 'd' and end with 'e'
    // We need to read one complete bencode value
    if reader.read_exact(&mut byte).is_err() {
        return None;
    }
    if byte[0] != b'd' {
        return None;
    }
    buf.push(byte[0]);

    // Read until we have a complete dict - track nesting depth
    let mut depth = 1;
    while depth > 0 {
        if reader.read_exact(&mut byte).is_err() {
            return None;
        }
        buf.push(byte[0]);
        match byte[0] {
            b'd' | b'l' => depth += 1,
            b'e' => depth -= 1,
            b'0'..=b'9' => {
                // It's a byte string length prefix - read the length then the string
                let mut len_str = String::new();
                len_str.push(byte[0] as char);
                loop {
                    if reader.read_exact(&mut byte).is_err() {
                        return None;
                    }
                    buf.push(byte[0]);
                    if byte[0] == b':' {
                        break;
                    }
                    len_str.push(byte[0] as char);
                }
                let len: usize = len_str.parse().ok()?;
                let mut string_buf = vec![0u8; len];
                if reader.read_exact(&mut string_buf).is_err() {
                    return None;
                }
                buf.extend_from_slice(&string_buf);
            }
            b'i' => {
                // Integer - read until 'e'
                loop {
                    if reader.read_exact(&mut byte).is_err() {
                        return None;
                    }
                    buf.push(byte[0]);
                    if byte[0] == b'e' {
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    // Now decode the complete message
    let mut decoder = Decoder::new(&buf);
    let dict = match decoder.next_object().ok()?? {
        Object::Dict(mut d) => {
            let mut map = HashMap::new();
            while let Ok(Some((key_bytes, value_obj))) = d.next_pair() {
                let key = String::from_utf8_lossy(key_bytes).to_string();
                if let Object::Bytes(val) = value_obj {
                    map.insert(key, String::from_utf8_lossy(val).to_string());
                }
            }
            map
        }
        _ => return None,
    };
    Some(dict)
}

/// A bencode value that can be a string, list, or dict.
enum BencodeValue {
    Str(String),
    List(Vec<BencodeValue>),
    Dict(Vec<(String, BencodeValue)>),
}

fn encode_bencode(val: &BencodeValue) -> Vec<u8> {
    match val {
        BencodeValue::Str(s) => {
            let mut buf = Vec::new();
            buf.extend_from_slice(format!("{}:", s.len()).as_bytes());
            buf.extend_from_slice(s.as_bytes());
            buf
        }
        BencodeValue::List(items) => {
            let mut buf = vec![b'l'];
            for item in items {
                buf.extend_from_slice(&encode_bencode(item));
            }
            buf.push(b'e');
            buf
        }
        BencodeValue::Dict(fields) => {
            let mut sorted: Vec<&(String, BencodeValue)> = fields.iter().collect();
            sorted.sort_by_key(|(k, _)| k.as_str());
            let mut buf = vec![b'd'];
            for (key, val) in sorted {
                buf.extend_from_slice(format!("{}:", key.len()).as_bytes());
                buf.extend_from_slice(key.as_bytes());
                buf.extend_from_slice(&encode_bencode(val));
            }
            buf.push(b'e');
            buf
        }
    }
}

fn write_describe_response(writer: &mut impl Write) {
    // Vars with code wrappers for idiomatic keyword-arg calling convention.
    // Internal pod functions use positional + map args; code wraps them.
    let ns_sym = NAMESPACE;

    struct VarDef {
        name: &'static str,
        meta: &'static str,
        code: Option<String>,
    }

    let vars = vec![
        VarDef {
            name: "ask*",
            meta: "",
            code: None,
        },
        VarDef {
            name: "ask",
            meta: "{:doc \"Ask a free-text question. Returns the response string.\n  Options: :default, :allow-blank, :suggestions\" :arglists ([question & {:keys [default allow-blank suggestions]}])}",
            code: Some(format!(
                "(defn ask [question & {{:keys [default allow-blank suggestions]}}] (try ({ns_sym}/ask* question {{\"default\" default \"allow-blank\" allow-blank \"suggestions\" suggestions}}) (catch Exception e (if (= \"canceled\" (ex-message e)) (System/exit 1) (throw e)))))"
            )),
        },
        VarDef {
            name: "confirm*",
            meta: "",
            code: None,
        },
        VarDef {
            name: "confirm",
            meta: "{:doc \"Ask a yes/no question. Returns true or false.\n  Options: :default\" :arglists ([question & {:keys [default]}])}",
            code: Some(format!(
                "(defn confirm [question & {{:keys [default]}}] (try ({ns_sym}/confirm* question {{\"default\" (when default (name default))}}) (catch Exception e (if (= \"canceled\" (ex-message e)) (System/exit 1) (throw e)))))"
            )),
        },
        VarDef {
            name: "choose*",
            meta: "",
            code: None,
        },
        VarDef {
            name: "choose",
            meta: "{:doc \"Choose one item from a list. Returns the chosen string.\n  Options: :default\" :arglists ([question options & {:keys [default]}])}",
            code: Some(format!(
                "(defn choose [question options & {{:keys [default]}}] (try ({ns_sym}/choose* question options {{\"default\" default}}) (catch Exception e (if (= \"canceled\" (ex-message e)) (System/exit 1) (throw e)))))"
            )),
        },
        VarDef {
            name: "select*",
            meta: "",
            code: None,
        },
        VarDef {
            name: "select",
            meta: "{:doc \"Select multiple items from a list. Returns a vector of chosen strings.\n  Options: :default\" :arglists ([question options & {:keys [default]}])}",
            code: Some(format!(
                "(defn select [question options & {{:keys [default]}}] (try ({ns_sym}/select* question options {{\"default\" default}}) (catch Exception e (if (= \"canceled\" (ex-message e)) (System/exit 1) (throw e)))))"
            )),
        },
        VarDef {
            name: "date*",
            meta: "",
            code: None,
        },
        VarDef {
            name: "date",
            meta: "{:doc \"Pick a date interactively. Returns a date string.\n  Options: :default, :format, :min-date, :max-date, :starting-date, :week-start, :help-message\" :arglists ([question & {:keys [default format min-date max-date starting-date week-start help-message]}])}",
            code: Some(format!(
                "(defn date [question & {{:keys [default format min-date max-date starting-date week-start help-message]}}] (try ({ns_sym}/date* question {{\"default\" default \"format\" format \"min-date\" min-date \"max-date\" max-date \"starting-date\" starting-date \"week-start\" week-start \"help-message\" help-message}}) (catch Exception e (if (= \"canceled\" (ex-message e)) (System/exit 1) (throw e)))))"
            )),
        },
        VarDef {
            name: "editor*",
            meta: "",
            code: None,
        },
        VarDef {
            name: "editor",
            meta: "{:doc \"Open a full text editor for input. Returns the entered text.\n  Options: :default, :help-message, :file-extension\" :arglists ([message & {:keys [default help-message file-extension]}])}",
            code: Some(format!(
                "(defn editor [message & {{:keys [default help-message file-extension]}}] (try ({ns_sym}/editor* message {{\"default\" default \"help-message\" help-message \"file-extension\" file-extension}}) (catch Exception e (if (= \"canceled\" (ex-message e)) (System/exit 1) (throw e)))))"
            )),
        },
        VarDef {
            name: "menu",
            meta: "{:doc \"Interactive menu that loops until quit. Entries are [label handler] pairs.\n  A nil handler exits the menu.\n  Options: :once, :default\" :arglists ([heading entries & {:keys [once default]}])}",
            code: Some(format!(concat!(
                "(defn menu [heading entries & {{:keys [once default]}}] ",
                "(let [labels (mapv first entries)] ",
                "(loop [dflt default] ",
                "(let [choice (try ({ns_sym}/choose* heading labels {{\"default\" dflt}}) (catch Exception e (if (= \"canceled\" (ex-message e)) nil (throw e))))] ",
                "(if (nil? choice) nil ",
                "(let [handler (second (first (filter #(= (first %) choice) entries)))] ",
                "(if (nil? handler) nil ",
                "(do (handler) ",
                "(if once nil (recur choice))))))))))"
            ), ns_sym = ns_sym)),
        },
    ];

    let var_list: Vec<BencodeValue> = vars
        .into_iter()
        .map(|v| {
            let mut fields = vec![
                ("name".to_string(), BencodeValue::Str(v.name.to_string())),
            ];
            if !v.meta.is_empty() {
                fields.push(("meta".to_string(), BencodeValue::Str(v.meta.to_string())));
            }
            if let Some(code) = v.code {
                fields.push(("code".to_string(), BencodeValue::Str(code)));
            }
            BencodeValue::Dict(fields)
        })
        .collect();

    let ns = BencodeValue::Dict(vec![
        ("name".to_string(), BencodeValue::Str(NAMESPACE.to_string())),
        ("vars".to_string(), BencodeValue::List(var_list)),
    ]);

    let ops = BencodeValue::Dict(vec![(
        "shutdown".to_string(),
        BencodeValue::Dict(vec![]),
    )]);

    let response = BencodeValue::Dict(vec![
        ("format".to_string(), BencodeValue::Str("json".to_string())),
        ("namespaces".to_string(), BencodeValue::List(vec![ns])),
        ("ops".to_string(), ops),
    ]);

    writer.write_all(&encode_bencode(&response)).unwrap();
    writer.flush().unwrap();
}

fn write_invoke_response(writer: &mut impl Write, id: &str, value: &str) {
    let response = BencodeValue::Dict(vec![
        ("id".to_string(), BencodeValue::Str(id.to_string())),
        (
            "status".to_string(),
            BencodeValue::List(vec![BencodeValue::Str("done".to_string())]),
        ),
        ("value".to_string(), BencodeValue::Str(value.to_string())),
    ]);
    writer.write_all(&encode_bencode(&response)).unwrap();
    writer.flush().unwrap();
}

fn write_invoke_error(writer: &mut impl Write, id: &str, message: &str) {
    let response = BencodeValue::Dict(vec![
        (
            "ex-message".to_string(),
            BencodeValue::Str(message.to_string()),
        ),
        ("id".to_string(), BencodeValue::Str(id.to_string())),
        (
            "status".to_string(),
            BencodeValue::List(vec![
                BencodeValue::Str("done".to_string()),
                BencodeValue::Str("error".to_string()),
            ]),
        ),
    ]);
    writer.write_all(&encode_bencode(&response)).unwrap();
    writer.flush().unwrap();
}

fn invoke_script_wizard(args: &[String]) -> Result<String, String> {
    let exe = std::env::current_exe().unwrap_or_else(|_| "script-wizard".into());

    let mut cmd = Command::new(&exe);
    cmd.args(args).stdout(Stdio::piped());

    #[cfg(target_os = "macos")]
    let saved_stdin = unsafe {
        // On macOS, crossterm's kqueue can't handle /dev/tty opened
        // post-fork. Use the pod's inherited stderr (the original pty
        // slave fd from the shell) for the child's stdin and stderr.
        let saved = libc::dup(0);
        libc::dup2(2, 0);
        libc::tcflush(0, libc::TCIFLUSH);

        cmd.stdin(Stdio::inherit()).stderr(Stdio::inherit());

        saved
    };

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // On Linux, pass /dev/tty directly as stdin/stderr for the child.
        let tty = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/tty")
            .map_err(|e| format!("Cannot open /dev/tty: {}", e))?;
        let tty_err = tty.try_clone().map_err(|e| format!("Clone tty: {}", e))?;
        cmd.stdin(Stdio::from(tty)).stderr(Stdio::from(tty_err));
    }

    #[cfg(windows)]
    {
        cmd.stdin(Stdio::from(
            std::fs::OpenOptions::new()
                .read(true)
                .open("CONIN$")
                .map_err(|e| format!("Cannot open CONIN$: {}", e))?,
        ))
        .stderr(Stdio::from(
            std::fs::OpenOptions::new()
                .write(true)
                .open("CONOUT$")
                .map_err(|e| format!("Cannot open CONOUT$: {}", e))?,
        ));
    }

    let output = cmd
        .output()
        .map_err(|e| format!("Failed to spawn: {}", e));

    // Restore the pod's original stdin (macOS only)
    #[cfg(target_os = "macos")]
    unsafe {
        libc::dup2(saved_stdin, 0);
        libc::close(saved_stdin);
    }

    let output = output?;
    let code = output.status.code().unwrap_or(1);
    if code == 0 {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else if code == 2 {
        Err("canceled".to_string())
    } else {
        Err(format!("script-wizard exited with code {}", code))
    }
}

fn handle_invoke(var: &str, args_json: &str) -> Result<String, String> {
    let args: Vec<JsonValue> =
        serde_json::from_str(args_json).map_err(|e| format!("Bad args JSON: {}", e))?;

    let fn_name = var.strip_prefix(&format!("{}/", NAMESPACE)).unwrap_or(var);

    let cmd_args = match fn_name {
        "ask*" => build_ask_args(&args)?,
        "confirm*" => build_confirm_args(&args)?,
        "choose*" => build_choose_args(&args)?,
        "select*" => build_select_args(&args)?,
        "date*" => build_date_args(&args)?,
        "editor*" => build_editor_args(&args)?,
        _ => return Err(format!("Unknown var: {}", var)),
    };

    match invoke_script_wizard(&cmd_args) {
        Ok(result) => match fn_name {
            "confirm*" => Ok("true".to_string()),
            "select*" => {
                let lines: Vec<&str> = result.lines().collect();
                serde_json::to_string(&lines).map_err(|e| e.to_string())
            }
            _ => serde_json::to_string(&result).map_err(|e| e.to_string()),
        },
        Err(e) if e == "canceled" => Err(e),
        Err(_) if fn_name == "confirm*" => Ok("false".to_string()),
        Err(e) => Err(e),
    }
}

fn build_ask_args(args: &[JsonValue]) -> Result<Vec<String>, String> {
    let question = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or("ask requires a question string")?;
    let mut cmd = vec!["ask".to_string(), question.to_string()];

    if let Some(opts) = args.get(1).and_then(|v| v.as_object()) {
        if let Some(default) = opts.get("default").and_then(|v| v.as_str()) {
            cmd.push(default.to_string());
        }
        if opts
            .get("allow-blank")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            cmd.push("--allow-blank".to_string());
        }
        if let Some(suggestions) = opts.get("suggestions").and_then(|v| v.as_array()) {
            cmd.push("--suggestions".to_string());
            cmd.push(serde_json::to_string(suggestions).unwrap());
        }
    }
    cmd.extend(["--cancel-code".to_string(), "2".to_string()]);
    Ok(cmd)
}

fn build_confirm_args(args: &[JsonValue]) -> Result<Vec<String>, String> {
    let question = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or("confirm requires a question string")?;
    let mut cmd = vec!["confirm".to_string(), question.to_string()];

    if let Some(opts) = args.get(1).and_then(|v| v.as_object()) {
        if let Some(default) = opts.get("default").and_then(|v| v.as_str()) {
            cmd.push(default.to_string());
        }
    }
    cmd.extend(["--cancel-code".to_string(), "2".to_string()]);
    Ok(cmd)
}

fn build_choose_args(args: &[JsonValue]) -> Result<Vec<String>, String> {
    let question = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or("choose requires a question string")?;
    let options = args
        .get(1)
        .and_then(|v| v.as_array())
        .ok_or("choose requires an options array")?;

    let mut cmd = vec!["choose".to_string(), question.to_string()];
    for opt in options {
        if let Some(s) = opt.as_str() {
            cmd.push(s.to_string());
        }
    }

    if let Some(opts) = args.get(2).and_then(|v| v.as_object()) {
        if let Some(default) = opts.get("default").and_then(|v| v.as_str()) {
            cmd.push("--default".to_string());
            cmd.push(default.to_string());
        }
    }
    cmd.extend(["--cancel-code".to_string(), "2".to_string()]);
    Ok(cmd)
}

fn build_select_args(args: &[JsonValue]) -> Result<Vec<String>, String> {
    let question = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or("select requires a question string")?;
    let options = args
        .get(1)
        .and_then(|v| v.as_array())
        .ok_or("select requires an options array")?;

    let mut cmd = vec!["select".to_string(), question.to_string()];
    for opt in options {
        if let Some(s) = opt.as_str() {
            cmd.push(s.to_string());
        }
    }

    if let Some(opts) = args.get(2).and_then(|v| v.as_object()) {
        if let Some(default) = opts.get("default").and_then(|v| v.as_str()) {
            cmd.push("--default".to_string());
            cmd.push(default.to_string());
        }
    }
    cmd.extend(["--cancel-code".to_string(), "2".to_string()]);
    Ok(cmd)
}

fn build_date_args(args: &[JsonValue]) -> Result<Vec<String>, String> {
    let question = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or("date requires a question string")?;
    let mut cmd = vec!["date".to_string(), question.to_string()];

    if let Some(opts) = args.get(1).and_then(|v| v.as_object()) {
        for (key, flag) in [
            ("default", "--default"),
            ("format", "--format"),
            ("min-date", "--min-date"),
            ("max-date", "--max-date"),
            ("starting-date", "--starting-date"),
            ("week-start", "--week-start"),
            ("help-message", "--help-message"),
        ] {
            if let Some(val) = opts.get(key).and_then(|v| v.as_str()) {
                cmd.push(flag.to_string());
                cmd.push(val.to_string());
            }
        }
    }
    cmd.extend(["--cancel-code".to_string(), "2".to_string()]);
    Ok(cmd)
}

fn build_editor_args(args: &[JsonValue]) -> Result<Vec<String>, String> {
    let message = args
        .first()
        .and_then(|v| v.as_str())
        .ok_or("editor requires a message string")?;
    let mut cmd = vec!["editor".to_string(), message.to_string()];

    if let Some(opts) = args.get(1).and_then(|v| v.as_object()) {
        for (key, flag) in [
            ("default", "--default"),
            ("help-message", "--help-message"),
            ("file-extension", "--file-extension"),
        ] {
            if let Some(val) = opts.get(key).and_then(|v| v.as_str()) {
                cmd.push(flag.to_string());
                cmd.push(val.to_string());
            }
        }
    }
    cmd.extend(["--cancel-code".to_string(), "2".to_string()]);
    Ok(cmd)
}

pub fn run_pod() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut reader = stdin.lock();
    let mut writer = stdout.lock();

    while let Some(msg) = read_bencode_message(&mut reader) {
        let op = msg.get("op").map(|s| s.as_str()).unwrap_or("");

        match op {
            "describe" => {
                write_describe_response(&mut writer);
            }
            "invoke" => {
                let id = msg.get("id").map(|s| s.as_str()).unwrap_or("");
                let var = msg.get("var").map(|s| s.as_str()).unwrap_or("");
                let args = msg.get("args").map(|s| s.as_str()).unwrap_or("[]");

                // eprintln!("[pod] invoke var={} args={}", var, args);
                match handle_invoke(var, args) {
                    Ok(value) => {
                        // eprintln!("[pod] ok: {}", value);
                        write_invoke_response(&mut writer, id, &value);
                    }
                    Err(e) => {
                        // eprintln!("[pod] err: {}", e);
                        write_invoke_error(&mut writer, id, &e);
                    }
                }
            }
            "shutdown" => {
                break;
            }
            _ => {}
        }
    }
}
