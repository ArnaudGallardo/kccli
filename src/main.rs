use clap::{Args, Parser, Subcommand};

mod command;
mod print;
mod connector;
mod ps;
mod get;
mod create;

use crate::command::KccliCommand;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(default_value_t=command::Format::Properties, short, long, value_enum)]
    format: command::Format,
    #[arg(default_value="http://localhost:8083", short, long)]
    endpoint: String,
}

#[derive(Subcommand)]
enum Commands {
    Ps,
    Get(ConnectorArgs),
    Rm(ConnectorArgs),
    Create,
    Run,
    Diff(ConnectorArgs),
    Status(ConnectorArgs),
    Plugins,
    Describe(ClassPathArgs),
    Validate(ClassPathArgs),
    Restart(ConnectorArgs),
    Pause(ConnectorArgs),
    Resume(ConnectorArgs)
}

#[derive(Args)]
struct ConnectorArgs {
    connector: String,
}

#[derive(Args)]
struct ClassPathArgs {
    class_path: String,
}

#[derive(Args)]
struct GetArgs {
    connector: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let host = std::env::var("KAFKA_CONNECT_REST").unwrap_or(cli.endpoint);
    match &cli.command {
        Commands::Ps => ps::PsCommand::execute(host, String::new(), cli.format).await,
        Commands::Get(args) => get::GetCommand::execute(host, args.connector.to_string(), cli.format).await,
        Commands::Rm(_) => todo!(),
        Commands::Create => create::CreateCommand::execute(host, String::new(), cli.format).await,
        Commands::Run => todo!(),
        Commands::Diff(_) => todo!(),
        Commands::Status(_) => todo!(),
        Commands::Plugins => todo!(),
        Commands::Describe(_) => todo!(),
        Commands::Validate(_) => todo!(),
        Commands::Restart(_) => todo!(),
        Commands::Pause(_) => todo!(),
        Commands::Resume(_) => todo!(),
    }
}
