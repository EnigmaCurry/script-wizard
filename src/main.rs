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
    },
}

fn program() -> Result<(), u8> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Ask { question, default }) => {
            ask::ask!(
                question,
                default.clone().unwrap_or(String::from("")).as_str()
            );
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
        }) => {
            println!(
                "{}",
                ask::choose(
                    question,
                    default.clone().unwrap_or(String::from("")).as_str(),
                    options.iter().map(String::as_str).collect()
                )
            );
            Ok(())
        }
        Some(Commands::Select {
            question,
            options,
            default,
        }) => {
            let selections = ask::select(
                question,
                default.clone().unwrap_or("".to_string()).as_str(),
                options.iter().map(String::as_str).collect(),
            );
            println!(
                "{}",
                serde_json::to_string(&selections).unwrap_or("[]".to_string())
            );
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
            println!("{}", date);
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
