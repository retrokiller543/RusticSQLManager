use crate::config::AppConfig;
use clap::{Args, Parser, Subcommand};

/// # CLI Structure
///
/// The structure of the App will look something like this:
///
/// ```bash
/// $ app [OPTIONS] SUBCOMMAND
///
/// OPTIONS:
///     -u, --user <user>      Sets the user
///     -H, --host <host>      Sets the host
///
/// SUBCOMMANDS:
///     create      Create a new user or database
///     config      Configure the application
///     remove      Remove a user or database
/// ```
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Command {
    #[arg(short, long)]
    pub(crate) user: Option<String>,
    #[arg(short = 'H', long)]
    pub(crate) host: Option<String>,
    #[command(subcommand)]
    pub(crate) subcommands: CommandType,
}

#[derive(Subcommand)]
pub enum CommandType {
    Create(CreateCommandArgs),
    Config(AppConfig),
    Remove,
}

#[derive(Args)]
pub struct ConfigCommandArgs {
    #[arg(short, long, default_value = "root")]
    user: String,
    #[arg(short = 'H', long, default_value = "localhost")]
    host: String,
    #[arg(short, help = "Store the password in the keyring")]
    password: bool,
}

#[derive(Args)]
pub struct CreateCommandArgs {
    #[command(subcommand)]
    pub(crate) subcommands: CreateType,
}

#[derive(Subcommand, Clone)]
pub enum CreateType {
    User {
        #[arg(help = "Name of the user")]
        name: String,
        #[arg(help = "Host of the user", default_value = "%")]
        host: String,
        #[arg(
            long,
            help = "The grants to provide this user",
            use_value_delimiter = true
        )]
        grants: Option<Vec<String>>,
        #[arg(long, help = "The database to grant access to", default_value = "*")]
        database: String,
        #[arg(long, help = "The table to grant access to", default_value = "*")]
        table: String,
    },
    Database {
        #[arg(help = "Name of the database")]
        name: String,
    },
}
