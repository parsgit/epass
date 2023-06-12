mod action;
pub mod aes;

mod pass;

use action::Action;
use pass::Pass;

fn main() {

    let chose_menu = Action::get_main_menu_action();

    if chose_menu == 3 {
        let config_menu = Action::get_config_menu_action();

        println!("menu: {}", config_menu);
        if config_menu == 1{
            Action::save_default_config();
        }
    }
    // println!("Hello, world!");

    // let mut pass = Pass();
    // pass.set_pass(String::from("my_password"));
    // pass.show_pass();

    // let mut pass2 = Pass{password:String::from("hello")};

    // let action = Action{};

    // let mut action = Action{};

    // action.show_main_menu();

    // pass2.show_pass();
    // Pass::show_pass();
    // Action::main_menu();
}
