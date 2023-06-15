use aes_gcm::aes::cipher::typenum::Integer;
use colored::*;
use std::{
    fs,
    fs::File,
    io::{stdin, stdout},
    path::{Path, PathBuf},
};
// use std::io::stdout;
use rpassword;
use std::io::Write;
//use termion::{clear, cursor};
use crossterm::cursor::{position, MoveTo};
use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
// use tabled::tables::IterTable;
// use prettytable::{Table, Row, Cell};

pub struct Password {
    name: String,
    content: String,
    index: u16,
}

impl Password {
    pub fn get_input(mut err_message: &str) -> String {
        err_message = if err_message == "" {
            "Invalid choice, please try again."
        } else {
            err_message
        };
        let mut result = String::new();

        stdin().read_line(&mut result).expect(err_message);

        return result;
    }

    pub fn main_menu(auto_clear: bool) {
        if auto_clear {
            Password::tm_clear();
        }
        loop {
            println!("{}", "1) View Password List".bold());
            println!("{}", "2) Save New Password".bold());
            println!("{}", "3) Edit Password".bold());
            println!("{}", "4) Delete Password".bold());
            println!("5) Set Password Storage");
            println!("6) Exit");
            print!("\n{}", "Please select an option: ".cyan());
            stdout().flush().unwrap();

            let mut result = String::new();

            stdin()
                .read_line(&mut result)
                .expect("Invalid choice, please try again.");

            match result.trim().parse::<i8>() {
                Ok(num) => {
                    if num >= 1 && num <= 6 {
                        // چک کردن عدد در محدوده مورد نظر
                        Password::manage_menu(num);
                        break;
                    } else {
                        Password::tm_clear();
                        println!("Error: {}","The entered number is not within the range of 1 to 5. Please try again.".red().bold());
                    }
                }
                Err(_) => {
                    Password::tm_clear();
                    println!("{}", "Invalid input. Please enter a valid number!".red());
                }
            }
        }
    }

    pub fn getKyesFilesList() -> Vec<Password> {
        let path = Password::get_path_keys();
        let mut files: Vec<Password> = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
        let mut idx:u16 = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.file_name().unwrap_or_default().to_str().unwrap();

                        idx+=1;
                        files.push(Password {
                            name: file_name.to_string(),
                            index: idx,
                            content: "".to_string(),
                        });
                    }
                }
            }
        }

        return files;
    }

    pub fn show_list_of_passwords() {
        Password::tm_clear();

        println!("Password list: \n");
        let list = Password::getKyesFilesList();

        let mut idx = 1;

        for file in list.iter() {
            // println!("{}: {}",file.index ,file.name );

            print!(" {}.", file.index.to_string().yellow());
            print!("{}     ", file.name.bold());
            if idx % 3 == 0 {
                println!();
            }
            idx += 1;
        }

        if (idx - 1) % 3 != 0 {
            println!();
        }

        print!("Please send password number: ");
        stdout().flush().unwrap();

        Password::get_input("");

        // println!("List {:?}", list);
    }
    pub fn tm_clear() {
        let mut stdout = stdout();
        stdout.execute(MoveTo(0, 0)).unwrap();
        stdout.execute(Clear(ClearType::All)).unwrap();
    }

    fn manage_menu(number: i8) {
        if number == 1 {
            Password::show_list_of_passwords();
        } else if number == 2 {
            Password::create_new_password();
        } else if (number == 5) {
            Password::config_the_storage_keys();
        } else if (number == 6) {
            println!("{}", "Goodbay.".bold());
            std::process::exit(0);
        }
    }

    fn error_access_message() -> &'static str {
        "Access Denied: Unable to create file or directory."
    }

    fn get_path_config() -> PathBuf {
        let config_path = dirs::config_dir().expect(Password::error_access_message());
        return config_path.join("epass");
    }

    fn get_path_config_keys() -> PathBuf {
        Password::get_path_config().join("storage_kyes_path")
    }

    pub fn get_path_default_documents() -> PathBuf {
        let documents = dirs::home_dir().unwrap().join("Documents");
        return documents;
    }

    pub fn get_path_keys() -> PathBuf {
        let file = Password::get_path_config_keys();
        let content = fs::read_to_string(file).unwrap();
        return Path::new(content.trim()).to_path_buf();
    }

    pub fn config_the_storage_keys() {
        let mut first = true;

        Password::tm_clear();

        loop {
            println!(
                "Select the password storage location (all information will be stored encrypted)"
            );
            println!(
                " {}",
                "1. Set default save location to Documents/Keys".bold()
            );
            println!(" {}", "2. Set custom save location".bold());
            print!("{}", "Choose an option: ".blue());
            stdout().flush().unwrap();

            let mut choice = String::new();
            stdin().read_line(&mut choice).expect("Failed to read line");

            match choice.trim().parse::<i8>() {
                Ok(num) => {
                    if num == 1 {
                        let keys_path = Password::get_path_default_documents().join("Keys");
                        let keys_path_config_file_path = Password::get_path_config_keys();

                        let mut file = File::create(keys_path_config_file_path).unwrap();
                        file.write_all(keys_path.as_path().display().to_string().as_bytes())
                            .unwrap();

                        Password::main_menu(true);
                        break;
                    } else if num == 2 {
                    } else {
                        Password::tm_clear();

                        println!("{}", "Invalid input, please select 1 or 2".red());
                    }
                }
                Err(_) => {
                    Password::tm_clear();

                    println!("{}", "Invalid input, please select 1 or 2".red());
                }
            }
        }
    }

    pub fn init_config() {
        let config_epass = Password::get_path_config();

        let path = Path::new(config_epass.as_path());
        let display = path.display();

        let error = format!("{}:{}", Password::error_access_message(), display);
        fs::create_dir_all(config_epass).expect(error.trim());
    }

    pub fn init_save_keys_path() {
        // let mut config_path = dirs::document_dir().expect(Password::error_access_message());
        // let config_epass = config_path.join("Keys");
        let config_epass = Password::get_path_keys();
        fs::create_dir_all(config_epass).expect(Password::error_access_message());
    }

    pub fn check_current_pass() {
        let storage_keys_path = Password::get_path_config_keys();
        let hash_file = Path::new(storage_keys_path.as_path());

        if (hash_file.exists() == false) {
            Password::tm_clear();
            println!("{}", "About:".bold());
            println!("{}","epass is a simple and secure program for saving, viewing, and managing passwords locally and offline");
            println!("repo: {}", "https://github.com/parsgit/epass");
            println!("version: {}\n\n", "1.0.0".bold());

            Password::config_the_storage_keys();
            // print!("{}", "Enter your password: ".yellow());
            // stdout().flush().unwrap();

            // let password = Password::get_input("");

            // let password2 = rpassword::prompt_password("Repeat the password: ").unwrap();

            // if (password.trim() == password2.trim()) {
            //     Password::tm_clear();
            //     println!("{}\n", "✅ Password saved".green().bold());
            //     Password::main_menu(false);
            // }
        }
    }

    fn create_new_password() {
        Password::tm_clear();
        println!("{}", "Send 0 to cancel and return to the menu");
        print!("{}", "The title of the new password: ".blue().bold());
        stdout().flush().unwrap();
        let name = Password::get_input("");

        if name.trim() == "0" {
            Password::main_menu(true);
        } else {
            print!("{}", "Enter your password: ".yellow());
            stdout().flush().unwrap();

            let password = Password::get_input("");

            let password2 = rpassword::prompt_password("Repeat the password: ").unwrap();

            if (password.trim() == password2.trim()) {
                Password::tm_clear();

                let mut file = File::create(Password::get_path_keys().join(name.trim())).unwrap();
                file.write_all(password.as_bytes()).unwrap();

                println!("{}\n", "✅ Password saved".green().bold());
            }
        }
    }
}
