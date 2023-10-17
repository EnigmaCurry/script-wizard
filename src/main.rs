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
        None => Err(1),
    }
}

fn main() {
    match program() {
        Ok(()) => std::process::exit(0),
        Err(_) => std::process::exit(1),
    };
}
