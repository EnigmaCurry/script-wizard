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
    },
    /// Ask an interactive yes/no question
    Confirm {
        /// Ask a yes/no question
        question: String,
        /// Default answer yes/no
        default_answer: Option<ask::Confirmation>,
    },
    /// Select from a list of choices
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
        Some(Commands::Ask { question }) => {
            ask::ask!(question);
            Ok(())
        }
        Some(Commands::Confirm {
            question,
            default_answer,
        }) => match ask::confirm(question, default_answer.clone()) {
            true => Ok(()),
            false => Err(1),
        },
        Some(Commands::Select { question, options }) => {
            println!(
                "{}",
                ask::select(question, options.iter().map(String::as_str).collect())
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
