use colored::*;
use std::io::{stdin, stdout};
// use std::io::stdout;
use rpassword;
use std::io::Write;
//use termion::{clear, cursor};
use crossterm::{terminal::{Clear, ClearType}, ExecutableCommand};
use crossterm::cursor::{MoveTo, position};

pub struct Password {}

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
            println!("1) View Password List");
            println!("2) Save New Password");
            println!("3) Edit Password");
            println!("4) Delete Password");
            println!("5) Set Password Storage Location");
            print!("\n{}", "Please select an option: ".cyan());
            stdout().flush().unwrap();

            let mut result = String::new();

            stdin()
                .read_line(&mut result)
                .expect("Invalid choice, please try again.");

            match result.trim().parse::<i8>() {
                Ok(num) => {
                    if num >= 1 && num <= 5 {
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

    pub fn tm_clear() {
        // print!("{}{}", clear::All, cursor::Goto(1, 1));
        // stdout().flush().unwrap();

        let mut stdout = stdout();
        stdout.execute(MoveTo(0, 0)).unwrap();
        stdout.execute(Clear(ClearType::All)).unwrap();
    }

    fn manage_menu(number: i8) {
        if number == 2 {
            Password::create_new_password();
        }
        // match number {
        //     1=>{ Password::create_new_password() }
        //     Err(_)=>{panic!("sss")}
        // }
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
                println!("{}\n", "✅ Password saved".green().bold());
                Password::main_menu(false);
            }
        }
    }
}
