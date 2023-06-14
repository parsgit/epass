// use dirs;
// use std::fs::{self, File};
// use std::io::{stdin, stdout, Write};
// use std::path::Path;

// pub struct Action {}

// impl Action {
//     pub fn get_main_menu_action() -> i8 {
//         println!("Please select an option:");
//         println!("1) View Password");
//         println!("2) Encrypt Data");
//         println!("3) Change Configuration");

//         let mut choice = String::new();

//         stdin()
//             .read_line(&mut choice)
//             .expect("Failed to read line.");

//         let _ = stdout().flush();

//         match choice.trim().parse() {
//             Ok(1) => {
//                 return 1;
//             }
//             Ok(2) => {
//                 return 2;
//             }
//             Ok(3) => {
//                 return 3;
//             }
//             _ => {
//                 println!("Invalid choice, please try again.");
//                 return 0;
//             }
//         }
//     }

//     pub fn get_config_menu_action() -> usize {
//         loop {
//             println!("Choose an option:");
//             println!("1. Set default save location to Documents/Keys");
//             println!("2. Set custom save location");
//             print!("Enter your choice: ");

//             let mut choice = String::new();
//             stdin().read_line(&mut choice).expect("Failed to read line");

//             match choice.trim().parse() {
//                 Ok(num) => {
//                     if num == 1 || num == 2 {
//                         return num;
//                     }
//                 }
//                 Err(_) => (),
//             }
//             println!("Invalid input, please select 1 or 2");
//         }
//     }

//     pub fn save_default_config() {
//         Action::make_directorys(".epass");
//         Action::make_directorys("Documents/keys");

//         let home_dir = dirs::home_dir().expect("Failed to get home directory."); // دریافت مسیر پوشه‌ی home
//         let dir_path = home_dir.join(".epass/save_keys_apth"); // ترکیب مسیر پوشه‌ی home با نام دایرکتوری جدید
//         let dir_save = home_dir.join("Documents/keys");

//         Action::save_path_config(dir_path.to_str().unwrap(), dir_save.to_str().unwrap());
//     }

//     pub fn save_path_config(path: &str, content: &str) {
//         let path = Path::new(path);
//         let display = path.display();

//         let mut file = match File::create(path) {
//             Err(why) => panic!("Couldn't create {}: {}", display, why),
//             Ok(file) => file,
//         };

//         match file.write_all(content.as_bytes()) {
//             Err(why)=>panic!("Couldn't write to {}: {}", display, why),
//             Ok(_)=>println!("Save Config in {}", display),
//         } 
//     }
//     // pub fn main_menu(){

//     //     Action::make_directorys(".epass");
//     //     Action::make_directorys("Documents/keys");

//     // }

//     fn make_directorys(name: &str) {
//         let home_dir = dirs::home_dir().expect("Failed to get home directory."); // دریافت مسیر پوشه‌ی home
//         let dir_path = home_dir.join(name); // ترکیب مسیر پوشه‌ی home با نام دایرکتوری جدید

//         let result1: bool = Path::new(&dir_path).is_dir();

//         if result1 == false {
//             match fs::create_dir(&dir_path) {
//                 Ok(_) => {
//                     println!("Directory created successfully!");
//                 }
//                 Err(err) => {
//                     println!("Failed to create directory: {}", err);
//                 }
//             }
//         }
//     }

//     // pub fn view_password() {
//     //     println!("TODO: View Password");
//     // }

//     // fn encrypt_data() {
//     //     println!("TODO: Encrypt Data");
//     // }

//     // fn change_config() {
//     //     println!("TODO: Change Configuration");
//     // }
// }
