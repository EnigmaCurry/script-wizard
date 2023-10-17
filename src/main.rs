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
        default_answer: Option<String>,
    },
    /// Ask an interactive yes/no question
    Confirm {
        /// Ask a yes/no question
        question: String,
        /// Default answer yes/no
        default_answer: Option<ask::Confirmation>,
    },
    /// Choose a single item from a list of choices
    Choose {
        /// Selection prompt
        question: String,
        /// Available choices
        options: Vec<String>,
    },
    /// Select multiple items from a list of choices
    Select {
        /// Selection prompt
        question: String,
        /// Available choices
        options: Vec<String>,
    },
}

fn program() -> Result<(), u8> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Ask {
            question,
            default_answer,
        }) => {
            ask::ask!(
                question,
                default_answer.clone().unwrap_or(String::from("")).as_str()
            );
            Ok(())
        }
        Some(Commands::Confirm {
            question,
            default_answer,
        }) => match ask::confirm(question, default_answer.clone()) {
            true => Ok(()),
            false => Err(1),
        },
        Some(Commands::Choose { question, options }) => {
            println!(
                "{}",
                ask::choose(question, options.iter().map(String::as_str).collect())
            );
            Ok(())
        }
        Some(Commands::Select { question, options }) => {
            let selections = ask::select(question, options.iter().map(String::as_str).collect());
            for s in selections.iter() {
                println!("{}", s);
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
