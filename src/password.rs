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
use sha2::{Digest, Sha256};

// use aes_gcm::{
//     aead::{Aead, AeadCore, KeyInit, OsRng},
//     Aes256Gcm,
//     Key, // Or `Aes128Gcm`
//     Nonce,
// };

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key,
};

use hex;
// use aes_gcm::{Aead, Aes256Gcm, Key, Nonce};
// use rand::rngs::OsRng;
// use rand::RngCore;
// use std::convert::TryInto;

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
            let mut idx: u16 = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let file_name = path.file_name().unwrap_or_default().to_str().unwrap();

                        idx += 1;
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

    pub fn show_list_of_passwords() -> Vec<Password> {
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

        list
        // println!("List {:?}", list);
    }

    pub fn get_password_index_and_show_content(list: &Vec<Password>) {
        print!("Please send password number: ");
        stdout().flush().unwrap();

        let get_index = Password::get_input("");

        let mut select_pass_index = 0;

        match get_index.trim().parse::<u16>() {
            Ok(number) => select_pass_index = number,
            Err(_) => eprintln!("Error: The input is not a valid integer"),
        }

        let password = Password::find_password_by_index(select_pass_index, &list);
    }

    // fn find_password(index: u16, passwords: &[Password]) -> Option<&Password> {
    //     for password in passwords.iter() {
    //         if password.index == index {
    //             return Some(password);
    //         }
    //     }
    //     None
    // }

    pub fn find_password_by_index(index: u16, list: &Vec<Password>) -> Option<&Password> {
        for pass in list.iter() {
            if (pass.index == index) {
                return Some(pass);
            }
        }

        None
    }

    pub fn tm_clear() {
        let mut stdout = stdout();
        stdout.execute(MoveTo(0, 0)).unwrap();
        stdout.execute(Clear(ClearType::All)).unwrap();
    }

    fn manage_menu(number: i8) {
        if number == 1 {
            let list = Password::show_list_of_passwords();
            Password::get_password_index_and_show_content(&list);
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


                let key: &[u8; 32] = &Password::text_to_bytes("hello ben");
                let key: &Key<Aes256Gcm> = key.into();
                let cipher = Aes256Gcm::new(&key);
                let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
                let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();

                file.write_all(&ciphertext).unwrap();

                println!("{}\n", "✅ Password saved".green().bold());
            }
        }
    }

    // pub fn encrypt_data() -> String {
    //     // The encryption key can be generated randomly:
    //     // let key = Aes256Gcm::generate_key("hello");

    //     // println!("key:{:?}", b"hello");
    //     // let key: &[u8; 32] = &[42; 32];
    //     // let key: &Key<Aes256Gcm> = b"hello".into();
    //     // // Transformed from a byte array:
    //     // let key: &[u8; 32] = &[42; 32];
    //     // let key: &Key<Aes256Gcm> = key.into();
    //     // println!("key:{:?}", key);

    //     // // Note that you can get byte array from slice using the `TryInto` trait:
    //     // let key: &[u8] = &[42; 32];
    //     // let key: [u8; 32] = key.try_into().unwrap();

    //     // Alternatively, the key can be transformed directly from a byte slice
    //     // (panicks on length mismatch):
    //     // let key = Key::<Aes256Gcm>::from_slice(b"hello");

    //     // let cipher = Aes256Gcm::new(&key);
    //     // let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
    //     // let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref()).unwrap();
    //     // let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();

    //     // println!("test:{:?}",plaintext);
    //     // assert_eq!(&plaintext, b"plaintext message");

    //     let my_key: &[u8; 32] = b"My super secret key       ";
    //     let key: &Key<Aes256Gcm> = my_key.into();

    //     return "".to_string();
    // }


    // pub fn decode(key: &str, ciphertext: &str) -> Result<String, &'static str> {
    //     // let key_arr = match hex::decode(key) {
    //     //     Ok(arr) => arr,
    //     //     Err(_) => return Err("Invalid hex key"),
    //     // };
    
    //     // let k: &[u8; 32] = match key_arr.as_slice().try_into() {
    //     //     Ok(k) => k,
    //     //     Err(_) => return Err("Invalid key length"),
    //     // };
    
    //     // let cipher = Aes256Gcm::new(&Key::<Aes256Gcm>::from_slice(k));
    //     // let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    //     // let ciphertext_arr = match hex::decode(ciphertext) {
    //     //     Ok(arr) => arr,
    //     //     Err(_) => return Err("Invalid hex ciphertext"),
    //     // };
    
    //     // let plaintext = cipher.decrypt(&nonce, ciphertext_arr.as_ref())
    //     //     .map_err(|_| "Decryption error")?;
    
    //     // Ok(String::from_utf8(plaintext).map_err(|_| "Cannot decode plaintext")?)
    // }
    // pub fn encrypt_data() -> Result<(), aes_gcm::Error> {
    //     let key: &[u8; 32] = &[42; 32];
    //     let key: &Key<Aes256Gcm> = key.into();

    //     let cipher = Aes256Gcm::new(key);
    //     let mut nonce_bytes = [0u8; 12];
    //     OsRng.fill_bytes(&mut nonce_bytes);
    //     let nonce = Nonce::from(nonce_bytes);
    //     let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
    //     let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
    //     assert_eq!(&plaintext, b"plaintext message");

    //     Ok(())
    // }

    // fn decrypt_data(key: &[u8], iv: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    //     let cipher = Aes256Gcm::new(Key::from_slice(key));
    //     let nonce = Nonce::from_slice(iv);
    //     match cipher.decrypt(nonce, data) {
    //         Ok(plaintext) => Ok(plaintext),
    //         Err(e) => Err(format!("Decryption failed: {}", e))
    //     }
    // }

    fn text_to_bytes(text: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        hasher.finalize().into()
    }
}
