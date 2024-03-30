use crate::cli::CreateType;
use anyhow::Result;

pub fn parse_create_command(command: &CreateType) -> Result<String> {
    match command {
        CreateType::User {
            name,
            host,
            grants,
            database,
            table,
        } => {
            let password = rpassword::prompt_password("Password: ")?;
            let user_creation = format!("CREATE USER `{name}`@`{host}` IDENTIFIED BY '{password}'");
            let grants_creation = grants
                .as_ref()
                .map(|grants| grants.join(","))
                .unwrap_or_default();

            let permissions =
                format!("GRANT {grants_creation} ON `{database}`.`{table}` TO `{name}`@`{host}`");
            Ok(format!("{};\n{}", user_creation, permissions))
        }
        CreateType::Database { name } => Ok(format!("CREATE DATABASE `{}`", name)),
    }
}
