use anyhow::Result;
use keyring::Entry;
use std::io;

pub fn get_db_pass(usr: &str, host: &str) -> Result<String> {
    let entry = Entry::new("rustic_sql_manager", format!("{usr}@{host}").as_str())?;

    match entry.get_password() {
        Ok(pass) => Ok(pass),
        Err(_) => {
            let pass = rpassword::prompt_password("Enter password: ")?;
            // does the user want to save the password? y/n
            loop {
                println!("Do you want to save your password? (yes|Y|y/no|N|n)");
                let mut save_password = String::new();
                io::stdin()
                    .read_line(&mut save_password)
                    .expect("Failed to read line");
                let save_password = save_password.trim().to_lowercase();

                match save_password.as_str() {
                    "yes" | "y" | "Y" => {
                        println!("Saving password...");
                        entry.set_password(&pass)?;
                        break;
                    }
                    "no" | "n" | "N" => {
                        println!("Not saving password.");
                        break;
                    }
                    _ => println!("Invalid input. Please enter 'yes' or 'no'."),
                }
            }
            Ok(pass)
        }
    }
}
