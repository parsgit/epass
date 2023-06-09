use std::io::{stdin, stdout, Write};
use std::fs;
use dirs;
use std::path::Path;

pub struct Action {}

impl Action{

    pub fn main_menu(){

        Action::make_directorys(".epass");
        Action::make_directorys("Documents/keys");


        loop {
            
            println!("Please select an option:");
            println!("1) View Password");
            println!("2) Encrypt Data");
            println!("3) Change Configuration");
    
            let mut choice = String::new();
    
            stdin().read_line(&mut choice)
            .expect("Failed to read line.");
    
            let _ = stdout().flush();
    
            match choice.trim().parse() {
                Ok(1) => Action::view_password(),
                Ok(2) => Action::encrypt_data(),
                Ok(3) => Action::change_config(),
                _ => {
                    println!("Invalid choice, please try again.");
                    continue;
                }
            }

            break;
        }

    }

    fn make_directorys(name: &str){
        let home_dir = dirs::home_dir().expect("Failed to get home directory."); // دریافت مسیر پوشه‌ی home
        let dir_path = home_dir.join(name); // ترکیب مسیر پوشه‌ی home با نام دایرکتوری جدید

        let result1: bool = Path::new(&dir_path).is_dir();



        if result1==false {
            match fs::create_dir(&dir_path) {
                Ok(_) => {
                    println!("Directory created successfully!");
                },
                Err(err) => {
                    println!("Failed to create directory: {}", err);
                }
            }
        }
    
    }
    
    pub fn view_password() {
        println!("TODO: View Password");
    }
    
    fn encrypt_data() {
        println!("TODO: Encrypt Data");
    }
    
    fn change_config() {
        println!("TODO: Change Configuration");
    }
}