mod cli;
mod config;
mod create;
mod password_handler;

use crate::cli::{Command, CommandType};
use crate::config::{get_config, write_config};
use anyhow::Result;
use clap::Parser;
use sqlx_mysql::MySqlPool;

/// Path to the configuration file.
///
/// This constant represents the relative path to the configuration file for the rustic_sql_manager application.
/// The configuration file is expected to be in TOML format and located in the ".config/rustic_sql_manager" directory.
pub const CONFIG_FILE: &str = ".config/rustic_sql_manager/config.toml";

/// Executes the provided SQL query.
///
/// This function first prints the SQL query with the password filtered out. It then prompts the user to confirm whether they want to continue.
/// If the user confirms, it retrieves the database password for the provided user and host by calling the `password_handler::get_db_pass` function.
/// It then establishes a connection to the MySQL database and executes the SQL query.
///
/// # Arguments
///
/// * `sql` - The SQL query to execute.
/// * `host` - The host of the MySQL database.
/// * `user` - The user to connect to the MySQL database.
///
/// # Returns
///
/// * `Result<(), anyhow::Error>` - Returns `Ok(())` if the SQL query was successfully executed. If an error occurred while retrieving the password, connecting to the database, or executing the query, it is returned as an `Err`.
async fn execute_sql(sql: String, host: String, user: String) -> Result<()> {
    print_sql_query_with_filtered_password(sql.clone());

    println!("Do you want to continue? (y/n)");
    if !yes_or_no()? {
        return Ok(());
    }

    let password = password_handler::get_db_pass(&user, &host)?;

    let pool = MySqlPool::connect(format!("mysql://{user}:{password}@{host}/").as_str()).await?;

    sqlx::query(&sql).execute(&pool).await?;
    Ok(())
}

/// Prompts the user for a yes or no response.
///
/// This function reads a line from standard input and checks if it is a "y" or "n" (case-insensitive).
/// If the input is "y" or "Y", it returns `Ok(true)`.
/// If the input is "n" or "N", it returns `Ok(false)`.
/// If the input is anything else, it returns an error.
///
/// # Returns
///
/// * `Result<bool, anyhow::Error>` - Returns `Ok(true)` if the user input was "y" or "Y", `Ok(false)` if the user input was "n" or "N", and an error if the user input was anything else.
fn yes_or_no() -> Result<bool> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    match input.trim() {
        "y" | "Y" => Ok(true),
        "n" | "N" => Ok(false),
        _ => Err(anyhow::anyhow!("Invalid input")),
    }
}

/// Prints the provided SQL query with the password filtered out.
///
/// This function finds the start index of the password placeholder in the SQL query. If found, it calculates the start and end indices of the password itself.
/// It then replaces the password portion with "[FILTERED]" and prints the filtered SQL query.
/// If the password placeholder is not found, it prints the SQL query as is.
///
/// # Arguments
///
/// * `query` - The SQL query to print with the password filtered out.
fn print_sql_query_with_filtered_password(query: String) {
    // Find the start index of the password placeholder
    if let Some(start_index) = query.find("IDENTIFIED BY '") {
        // Calculate the start and end indices of the password itself
        let password_start_index = start_index + "IDENTIFIED BY '".len();
        let password_end_index = password_start_index
            + query
                .get(password_start_index..)
                .unwrap()
                .find('\'')
                .unwrap_or(query.len());

        // Replace the password portion with "[FILTERED]"
        let mut filtered_sql_query = query.clone();
        filtered_sql_query.replace_range(password_start_index..password_end_index, "[FILTERED]");

        // Print the filtered SQL query
        println!("{}", filtered_sql_query);
    } else {
        println!("{}", query);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cmd = Command::parse();
    let config = get_config()?;

    let (user, host) = (
        cmd.user.unwrap_or(config.get("user")?),
        cmd.host.unwrap_or(config.get("host")?),
    );

    match cmd.subcommands {
        CommandType::Create(args) => {
            let sql = create::parse_create_command(&args.subcommands)?;
            execute_sql(sql, host, user).await?;
        }
        CommandType::Remove => {}
        CommandType::Config(args) => {
            write_config(&args)?;
        }
    };

    println!("Done");

    Ok(())
}

