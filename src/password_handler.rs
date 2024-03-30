use anyhow::Result;
use keyring::Entry;
use std::io;

/// Retrieves a keyring entry for the specified user and host.
///
/// This function creates a new keyring entry with the service name "rustic_sql_manager" and the account name formatted as "{usr}@{host}".
/// It then returns this keyring entry.
///
/// # Arguments
///
/// * `usr` - The username for which to retrieve the keyring entry.
/// * `host` - The host for which to retrieve the keyring entry.
///
/// # Returns
///
/// * `Result<Entry, anyhow::Error>` - Returns the keyring entry if it was successfully retrieved. If an error occurred while retrieving the keyring entry, it is returned as an `Err`.
fn get_keyring(usr: &str, host: &str) -> Result<Entry> {
    Ok(Entry::new("rustic_sql_manager", format!("{usr}@{host}").as_str())?)
}

/// Retrieves the database password for the specified user and host.
///
/// This function first tries to retrieve the keyring entry for the specified user and host by calling the `get_keyring` function.
/// If the keyring entry is successfully retrieved, it tries to get the password from the keyring entry.
/// If the password is successfully retrieved, it is returned.
/// If the password is not found in the keyring entry, the function prompts the user to enter their password and asks whether they want to save it.
/// If the user chooses to save the password, it is saved in the keyring entry.
/// If the keyring entry is not found, the function prompts the user to enter their password and returns it.
///
/// # Arguments
///
/// * `usr` - The username for which to retrieve the database password.
/// * `host` - The host for which to retrieve the database password.
///
/// # Returns
///
/// * `Result<String, anyhow::Error>` - Returns the database password if it was successfully retrieved or entered by the user. If an error occurred while retrieving the keyring entry, getting the password from the keyring entry, or saving the password in the keyring entry, it is returned as an `Err`.
pub fn get_db_pass(usr: &str, host: &str) -> Result<String> {
    match get_keyring(usr, host) {
        Ok(entry) => {
            match entry.get_password() {
                Ok(pass) => Ok(pass),
                Err(_) => {
                    let pass = rpassword::prompt_password("Enter User password: ")?;
                    // does the user want to save the password? y/n
                    loop {
                        println!("Do you want to save your password? (yes|y/no|n)");
                        let mut save_password = String::new();
                        io::stdin()
                            .read_line(&mut save_password)
                            .expect("Failed to read line");
                        let save_password = save_password.trim().to_lowercase();

                        match save_password.as_str() {
                            "yes" | "y" => {
                                println!("Saving password...");
                                if entry.set_password(&pass).is_err() {
                                    eprintln!("No Native Keyring found. Please install one and try again.");
                                }
                                break;
                            }
                            "no" | "n" => {
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
        Err(_) => {
            eprintln!("No Native Keyring found. Please install one and try again.");
            let pass = rpassword::prompt_password("Enter User password: ")?;
            Ok(pass)
        }
    }
}