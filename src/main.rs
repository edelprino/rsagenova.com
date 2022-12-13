use clap::{Parser, Subcommand};

mod airtable;
mod command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    GenerateIssue { number: i64 },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Command::GenerateIssue { number } => command::issue::generate_issue(*number).await,
    }
    Ok(())
}
