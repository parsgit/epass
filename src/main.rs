mod action;
pub mod aes;

mod password;

use colored::Colorize;
// use action::Action;
use password::Password;

fn main() {

    Password::tm_clear();
    println!("{}", "About:".bold());
    println!("{}","epass is a simple and secure program for saving, viewing, and managing passwords locally and offline");
    println!("repo: {}","https://github.com/parsgit/epass");
    println!("version: {}\n\n","1.0.0".bold());

    let get_result = Password::main_menu(false);
    
    // Action::manage_menu(get_result);

    // let chose_menu = Action::get_main_menu_action();

    // if chose_menu == 3 {
    //     let config_menu = Action::get_config_menu_action();

    //     println!("menu: {}", config_menu);
    //     if config_menu == 1{
    //         Action::save_default_config();
    //     }
    // }
    // println!("Hello, world!");

    // let mut pass = Pass();
    // pass.set_pass(String::from("my_password"));
    // pass.show_pass();
    // pub stract Password{}
    // let mut pass2 = Pass{password:String::from("hello")};

    // let action = Action{};

    // let mut action = Action{};

    // action.show_main_menu();

    // pass2.show_pass();
    // Pass::show_pass();
    // Action::main_menu();
}
