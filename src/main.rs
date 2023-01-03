use clap::{Parser, Subcommand};

mod airtable;
mod command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let cli = Cli::parse();

    command::rsa::generate_all().await;

    Ok(())
}
