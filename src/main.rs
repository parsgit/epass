mod action;
pub mod aes;

mod password;

use colored::Colorize;
// use action::Action;
use password::Password;

fn main() {

    // Password::get_path_default_documents();

    Password::init_config();
    Password::check_current_pass();
    Password::init_save_keys_path();



    let get_result = Password::main_menu(true);
    









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
