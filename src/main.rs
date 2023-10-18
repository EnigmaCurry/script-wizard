use clap::{Parser, Subcommand};
mod ask;

#[derive(Parser)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Ask an interactive question and output the response
    Ask {
        /// The question to ask
        question: String,
        default: Option<String>,
        #[arg(short, long)]
        json: bool,
    },
    /// Ask an interactive yes/no question
    Confirm {
        /// Ask a yes/no question
        question: String,
        /// Default answer yes/no
        default: Option<ask::Confirmation>,
    },
    /// Choose a single item from a list of choices
    Choose {
        /// Selection prompt
        question: String,
        /// Available choices
        options: Vec<String>,
        /// Default answer
        #[arg(short, long, value_name = "ITEM")]
        default: Option<String>,
        #[arg(short, long)]
        json: bool,
    },
    /// Select multiple items from a list of choices
    Select {
        /// Selection prompt
        question: String,
        /// Available choices
        options: Vec<String>,
        /// Default answer
        #[arg(short, long, value_name = "JSON_ARRAY")]
        default: Option<String>,
        #[arg(short, long)]
        json: bool,
    },
    /// Choose date
    Date {
        /// Selection prompt
        question: String,
        /// Default answer
        #[arg(default_value = "%Y-%m-%d", long, value_name = "FORMAT")]
        format: Option<String>,
        #[arg(long, value_name = "DATE")]
        default: Option<String>,
        #[arg(long, value_name = "DATE")]
        min_date: Option<String>,
        #[arg(long, value_name = "DATE")]
        max_date: Option<String>,
        #[arg(default_value = "sunday", long, value_name = "WEEKDAY")]
        week_start: Option<chrono::Weekday>,
        #[arg(long, value_name = "MESSAGE")]
        help_message: Option<String>,
        #[arg(short, long)]
        json: bool,
    },
    /// Full text editor box
    Editor {
        /// The question to ask
        message: String,
        #[arg(long, value_name = "TEXT")]
        default: Option<String>,
        #[arg(long, value_name = "MESSAGE")]
        help_message: Option<String>,
        #[arg(long, value_name = "EXTENSION")]
        file_extension: Option<String>,
        #[arg(short, long)]
        json: bool,
    },
}

fn program() -> Result<(), u8> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Ask {
            question,
            default,
            json,
        }) => {
            let response = ask::ask!(
                question,
                default.clone().unwrap_or(String::from("")).as_str()
            );
            if *json {
                println!(
                    "{}",
                    serde_json::to_string(&response).unwrap_or("".to_string())
                )
            } else {
                println!("{}", response);
            }
            Ok(())
        }
        Some(Commands::Confirm { question, default }) => {
            match ask::confirm(question, default.clone()) {
                true => Ok(()),
                false => Err(1),
            }
        }
        Some(Commands::Choose {
            question,
            options,
            default,
            json,
        }) => {
            let choice = ask::choose(
                question,
                default.clone().unwrap_or(String::from("")).as_str(),
                options.iter().map(String::as_str).collect(),
            );
            if *json {
                println!(
                    "{}",
                    serde_json::to_string(&choice).unwrap_or("".to_string())
                )
            } else {
                println!("{}", choice);
            }
            Ok(())
        }
        Some(Commands::Select {
            question,
            options,
            default,
            json,
        }) => {
            let selections = ask::select(
                question,
                default.clone().unwrap_or("".to_string()).as_str(),
                options.iter().map(String::as_str).collect(),
            );
            if *json {
                println!(
                    "{}",
                    serde_json::to_string(&selections).unwrap_or("".to_string())
                )
            } else {
                for t in selections {
                    println!("{}", t);
                }
            }
            Ok(())
        }
        Some(Commands::Date {
            question,
            default,
            min_date,
            max_date,
            week_start,
            help_message,
            format,
            json,
        }) => {
            let date = ask::date(
                question,
                default.clone().unwrap_or("".to_string()).as_str(),
                min_date.clone().unwrap_or("".to_string()).as_str(),
                max_date.clone().unwrap_or("".to_string()).as_str(),
                week_start.clone().unwrap_or(chrono::Weekday::Sun),
                help_message.clone().unwrap_or("".to_string()).as_str(),
                format.clone().unwrap_or("".to_string()).as_str(),
            );
            if *json {
                println!("{}", serde_json::to_string(&date).unwrap_or("".to_string()))
            } else {
                println!("{}", date);
            }
            Ok(())
        }
        Some(Commands::Editor {
            message,
            default,
            help_message,
            file_extension,
            json,
        }) => {
            let text = ask::editor(
                message,
                default.clone().unwrap_or("".to_string()).as_str(),
                help_message.clone().unwrap_or("".to_string()).as_str(),
                file_extension.clone().unwrap_or("".to_string()).as_str(),
            );
            if *json {
                println!("{}", serde_json::to_string(&text).unwrap())
            } else {
                println!("{}", text);
            }
            Ok(())
        }
        None => Err(1),
    }
}

fn main() {
    match program() {
        Ok(()) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    };
}
