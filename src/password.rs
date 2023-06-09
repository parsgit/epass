use colored::*;
use crossterm::cursor::{position, MoveTo, MoveToColumn};
use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use rand::Rng;
use rpassword;
use std::io::Write;
use std::{
    fs,
    fs::File,
    io::{stdin, stdout, Read},
    path::{Path, PathBuf},
};

use crossterm::{
    cursor, execute,
    style::{Color, Print, SetForegroundColor},
    terminal,
};
use std::time::Duration;
use std::{thread, time};

use crate::config::Config;

pub struct Password {
    password: String,
}
pub struct PasswordItem {
    name: String,
    content: String,
    index: i16,
}

impl Password {
    pub fn new(current_password: String) -> Password {
        return Password {
            password: current_password,
        };
    }

    pub fn get_main_password() -> String {
        Password::tm_clear();

        let password_file = Config::config_file_password_hash_path();
        let path = password_file.as_path();

        if path.exists() {
            let pass = rpassword::prompt_password("Enter main password:").unwrap();
            let hash = Config::read_text_file(password_file);
            let user_enter_pass_hash = Config::make_password_hash(&pass);

            if user_enter_pass_hash == hash {
                return pass;
            } else {
                println!("{}", "Invalid password. Please try again.".red().bold());
                std::process::exit(0);
            }
        } else {
            return Password::login_by_password();
        }
    }

    pub fn login_by_password() -> String {
        loop {
            println!("{}\n", "For the first time, users need to set a master password in order to access all of their saved passwords.".bold());

            let password1 = rpassword::prompt_password("Enter main password:").unwrap();
            let password2 = rpassword::prompt_password("Repeat the password:").unwrap();

            if password1 == password2 {
                let config_path = Config::config_file_password_hash_path();
                let mut file = File::create(config_path).unwrap();

                file.write_all(Config::make_password_hash(&password1).as_bytes())
                    .unwrap();

                return password1;
            } else {
                Password::tm_clear();
                println!("{}", "Error: Password and confirmation do not match.".red());
            }
        }
    }

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

    fn re_encrypt_all_passwords(old_password: &str, new_password: &str) {
        let list = Password::getKyesFilesList();

        for item in list.iter() {
            let pass_path = Config::get_path_keys().join(&item.name);
            let content = Config::read_file(pass_path);

            let decrypt_string = Config::decode(old_password, content);

            let ciphertext = Config::encode(new_password, decrypt_string.as_str());

            let mut file = File::create(Config::get_path_keys().join(item.name.trim())).unwrap();
            file.write_all(&ciphertext.as_bytes()).unwrap();
        }
    }

    fn manage_menu(&self, number: i8) {
        if number == 1 {
            let mut list = self.show_list_of_passwords();
            self.get_password_index_and_show_content(&mut list);
        } else if number == 2 {
            self.create_new_password();
        } else if number == 3 {
            self.edit_a_password();
        } else if number == 4 {
            self.delete_a_password();
        } else if number == 5 {
            let old_password = &self.password;

            let new_password = Password::login_by_password();
            Password::re_encrypt_all_passwords(old_password.as_str(), new_password.as_str());
            println!("Password has been changed. Please log in again.");
            std::process::exit(0);
        } else if number == 6 {
            Config::export();
        } else if number == 7 {
            if Config::import() {
                Password::tm_clear();
                println!("{}\n", "Import was successful".green().bold());
                self.main_menu(false);
            }
        } else if number == 8 {
            println!("{}", "Goodbay.".bold());
            std::process::exit(0);
        }
    }

    pub fn main_menu(&self, auto_clear: bool) {
        if auto_clear {
            Password::tm_clear();
        }
        loop {
            println!("{}", "1) Show Password".bold());
            println!("{}", "2) Add Password".bold());
            println!("{}", "3) Edit Password".bold());
            println!("{}", "4) Delete Password".bold());
            println!("5) Change Main Password");
            println!("6) Export");
            println!("7) Import");
            println!("8) Exit");
            print!("\n{}", "Please select an option: ".cyan());
            stdout().flush().unwrap();

            let mut result = String::new();

            stdin()
                .read_line(&mut result)
                .expect("Invalid choice, please try again.");

            match result.trim().parse::<i8>() {
                Ok(num) => {
                    if num >= 1 && num <= 8 {
                        self.manage_menu(num);
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

    pub fn getKyesFilesList() -> Vec<PasswordItem> {
        let path = Config::get_path_keys();
        let mut files: Vec<PasswordItem> = Vec::new();

        if let Ok(entries) = fs::read_dir(path) {
            let mut idx: i16 = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.file_name().unwrap_or_default().to_str().unwrap();

                        idx += 1;
                        files.push(PasswordItem {
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

    pub fn show_list_of_passwords(&self) -> Vec<PasswordItem> {
        Password::tm_clear();

        println!("Password list:");
        let list = Password::getKyesFilesList();

        let mut idx = 1;

        for file in list.iter() {
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

        list
    }

    pub fn get_password_index_and_show_content(&self, list: &mut Vec<PasswordItem>) {
        println!("\n{}", "0) Back");
        print!("{}", "Please send password number: ".bold());
        stdout().flush().unwrap();

        let get_index = Password::get_input("");

        if get_index.trim() == "0" {
            self.main_menu(true);
            return;
        }

        let mut select_pass_index = 0;

        match get_index.trim().parse::<i16>() {
            Ok(number) => select_pass_index = number,
            Err(_) => {
                Password::tm_clear();
                println!("{}", "Error: The input is not a valid integer".red());
                self.main_menu(false);
                return;
            }
        }

        let password = self.find_password_by_index(select_pass_index, list, true);

        let password = match password {
            Some(p) => p,
            None => {
                Password::tm_clear();
                println!("{}", "Password not found".red());
                self.main_menu(false);
                return;
            }
        };

        println!("\npassword:{}\n", password.content.bold());

        let mut seconds = 15;
        let mut stdout = stdout();

        execute!(stdout, cursor::Hide).unwrap();

        while seconds > 0 {
            execute!(
                stdout,
                terminal::Clear(terminal::ClearType::CurrentLine),
                cursor::MoveToColumn(0),
                // SetForegroundColor(Color::Red),
                Print(format!("{} seconds left", seconds))
            )
            .unwrap();
            stdout.flush().unwrap();

            std::thread::sleep(std::time::Duration::from_secs(1));
            seconds -= 1;
        }

        execute!(
            stdout,
            terminal::Clear(terminal::ClearType::CurrentLine),
            cursor::MoveToColumn(0),
            Print("Time's up!")
        )
        .unwrap();
        stdout.flush().unwrap();

        execute!(stdout, cursor::Show).unwrap();
        stdout.flush().unwrap();

        Password::tm_clear();
        self.main_menu(false);
    }

    pub fn find_password_by_index(
        &self,
        index: i16,
        list: &mut Vec<PasswordItem>,
        decode: bool,
    ) -> Option<PasswordItem> {
        let mut find = PasswordItem {
            name: "".to_string(),
            content: "".to_string(),
            index: -1,
        };

        for mut pass in list.iter_mut() {
            if pass.index == index {
                let pass_path = Config::get_path_keys().join(&pass.name);
                let content = Config::read_file(pass_path);

                find.name = pass.name.clone();
                find.index = pass.index;
                if decode {
                    find.content = Config::decode(self.password.as_str(), content).to_string();
                }

                return Some(find);
            }
        }

        None
    }

    pub fn tm_clear() {
        let mut stdout = stdout();
        stdout.execute(MoveTo(0, 0)).unwrap();
        stdout.execute(Clear(ClearType::All)).unwrap();
    }

    pub fn delete_a_password(&self) {
        let mut list = self.show_list_of_passwords();
        print!(
            "\nO) Back\n{}{}: ",
            "Enter the password number to ".bright_purple(),
            "delete".bold().red()
        );
        stdout().flush().unwrap();

        let mut number_string = String::new();

        stdin().read_line(&mut number_string).unwrap();

        let number = match number_string.trim().parse::<i16>() {
            Ok(p) => p,
            Err(_) => {
                Password::tm_clear();
                println!("{}","The entered expression is not correct. You must enter the password number as a number".red());
                self.main_menu(false);
                return;
            }
        };

        let item = self.find_password_by_index(number, &mut list, false);

        match item {
            Some(item) => {
                let mut rng = rand::thread_rng();
                let src: String = (0..4).map(|_| rng.gen_range(0..=9).to_string()).collect();
                println!(
                    "(You are removing the password named '{}')",
                    item.name.bold()
                );
                print!("Send the number {} to remove the password:", src);
                stdout().flush().unwrap();

                let mut get_sec = String::new();
                stdin().read_line(&mut get_sec).unwrap();

                if src.trim() == get_sec.trim() {
                    fs::remove_file(Config::get_path_keys().join(item.name)).unwrap();

                    Password::tm_clear();
                    println!("{}", "Password removed".green().bold());
                    self.main_menu(false);
                    return;
                } else {
                    Password::tm_clear();
                    println!("{}", "The security number sent was incorrect".red());
                    self.main_menu(false);
                    return;
                }
            }
            None => {
                Password::tm_clear();
                println!("{}", "Password not found".red());
                self.main_menu(false);
                return;
            }
        }
    }

    pub fn edit_a_password(&self) {
        let mut list = self.show_list_of_passwords();
        // println!("");
        print!("\n0) Back\nEnter the password number to edit: ",);
        stdout().flush().unwrap();

        let mut number_string = String::new();

        stdin().read_line(&mut number_string).unwrap();

        let number = match number_string.trim().parse::<i16>() {
            Ok(p) => p,
            Err(_) => {
                Password::tm_clear();
                println!("{}","The entered expression is not correct. You must enter the password number as a number".red());
                self.main_menu(false);
                return;
            }
        };

        let item = self.find_password_by_index(number, &mut list, true);

        match item {
            Some(item) => {
                println!("Current password {}: {}", item.name.bold(), item.content);
                print!("Enter new password to edit {}:", item.name.bold());
                stdout().flush().unwrap();

                let password = Password::get_input("");

                let password2 = rpassword::prompt_password("Repeat the password: ").unwrap();

                if password.trim() == password2.trim() {
                    Password::tm_clear();

                    let mut file =
                        File::create(Config::get_path_keys().join(item.name.trim())).unwrap();
                    let ciphertext = Config::encode(&self.password, &password);
                    file.write_all(&ciphertext.as_bytes()).unwrap();

                    Password::tm_clear();
                    println!(
                        "{} {} {}\n",
                        "The password for".green(),
                        item.name.green().bold(),
                        "was edited".green()
                    );
                    self.main_menu(false);
                    return;
                } else {
                    Password::tm_clear();
                    println!("{}", "The password does not match its repetition".red());
                    self.main_menu(false);
                }
            }
            None => {
                Password::tm_clear();
                println!("{}", "Password not found".red());
                self.main_menu(false);
                return;
            }
        }
    }

    fn error_access_message() -> &'static str {
        "Access Denied: Unable to create file or directory."
    }

    fn get_path_config() -> PathBuf {
        let config_path = dirs::config_dir().expect(Password::error_access_message());
        return config_path.join("epass");
    }

    pub fn init_config() {
        let config_epass = Password::get_path_config();

        let path = Path::new(config_epass.as_path());
        let display = path.display();

        let error = format!("{}:{}", Password::error_access_message(), display);
        fs::create_dir_all(config_epass).expect(error.trim());
        fs::create_dir_all(Config::get_path_keys()).expect(error.trim());
    }

    fn create_new_password(&self) {
        Password::tm_clear();
        println!("{}", "0) Back");
        print!("{}", "The title of the new password:".blue().bold());
        stdout().flush().unwrap();

        let name = Password::get_input("");

        if name.trim() == "0" {
            self.main_menu(true);
        } else {
            if Config::get_path_keys().join(name.trim()).exists() {
                Password::tm_clear();
                println!(
                    "{}",
                    "The password with this title has already been saved".red()
                );
                self.main_menu(false);
                return;
            }

            println!("{}", "0) Back");
            println!("{}", "1) Generate a random password");
            print!("{}", "Enter your password:".yellow());
            stdout().flush().unwrap();


            let mut password = Password::get_input("");
            let mut password2 = String::new();
            let mut force = false;

            if password.trim() == "0"{
                self.main_menu(true);
            }
            else if password.trim() =="1"{

                let mut generate_password = self.generate_random_password(12);
                self.confirm_random_password(&mut generate_password);

                password = generate_password;
                force = true;
            }
            else{
                password2 = rpassword::prompt_password("Repeat the password:").unwrap();
            }



            if password.trim() == password2.trim() || force {
                Password::tm_clear();

                let mut file = File::create(Config::get_path_keys().join(name.trim())).unwrap();
                let ciphertext = Config::encode(&self.password, &password);
                file.write_all(&ciphertext.as_bytes()).unwrap();

                Password::tm_clear();
                println!("{}\n", "✅ Password saved".green().bold());
                self.main_menu(false);
            } else {
                Password::tm_clear();
                println!("{}", "The password does not match its repetition".red());
                self.main_menu(false);
            }
        }
    }

    fn generate_random_password(&self, length: usize) -> String {
        let characters: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ12345678904";
        let mut rng = rand::thread_rng();
        let password: String = (0..length)
            .map(|_| {
                let index = rng.gen_range(0..characters.len());
                characters[index] as char
            })
            .collect();
        password
    }

    fn confirm_random_password(&self,password :&mut String){

        println!("Password created to use:{}", password.bold().blue());

        println!("If you have copied this password and want to use it, send the number '1'. If you want another random password to be generated, send the number '2'");
        print!("Confirm and continue password (default 1):");
        stdout().flush().unwrap();

        let mut answer = String::new();
        stdin().read_line(&mut answer).unwrap();

        if answer.trim()=="2"{
            *password = self.generate_random_password(14);
            self.confirm_random_password(password);
        }
    }
}
