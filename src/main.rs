mod cli;
mod config;
mod create;
mod password_handler;

use crate::cli::{Command, CommandType};
use crate::config::{get_config, write_config};
use anyhow::Result;
use clap::Parser;
use sqlx_mysql::MySqlPool;

pub const CONFIG_FILE: &str = ".config/rustic_sql_manager/config.toml";

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

async fn execute_sql(sql: String, host: String, user: String) -> Result<()> {
    print_sql_query_with_filtered_password(sql.clone());

    println!("{sql}; Do you want to continue? (y/n)");
    if !yes_or_no()? {
        return Ok(());
    }

    let password = password_handler::get_db_pass(&user, &host)?;

    let pool = MySqlPool::connect(format!("mysql://{user}:{password}@{host}/").as_str()).await?;

    sqlx::query(&sql).execute(&pool).await?;
    Ok(())
}

fn yes_or_no() -> Result<bool> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    match input.trim() {
        "y" | "Y" => Ok(true),
        "n" | "N" => Ok(false),
        _ => Err(anyhow::anyhow!("Invalid input")),
    }
}

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
    }
}
