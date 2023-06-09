use std::{
    alloc::System,
    fs::{self, File},
    io::stdin,
    path::{Path, PathBuf},
    process::exit,
};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key,
};
use chrono::Local;
use colored::Colorize;
use rand::RngCore;
use sha3::{digest::typenum::Same, Digest, Sha3_256};
use std::io::{Read, Write};
use zip::{ZipArchive, ZipWriter};

use native_dialog::{FileDialog, MessageDialog, MessageType};

use crate::password::Password;

pub struct Config {}

impl Config {
    pub fn read_file(path: PathBuf) -> Vec<u8> {
        let content = fs::read(path).expect("error read password file content");
        return content;
    }

    pub fn read_text_file(path: PathBuf) -> String {
        let content = fs::read_to_string(path).expect("error read password file content");
        return content;
    }

    pub fn make_password_hash(password_string: &String) -> String {
        // create a SHA3-256 object
        let mut hasher = Sha3_256::new();

        // write input message
        hasher.update(password_string.as_bytes());

        // read hash digest
        let result = hasher.finalize();

        format!("{:x}", result)
    }

    pub fn error_access_message() -> &'static str {
        "Access Denied: Unable to create file or directory."
    }

    pub fn main_config_dir_path() -> PathBuf {
        let config_path = dirs::config_dir().expect(Config::error_access_message());
        return config_path.join("epass");
    }

    pub fn config_file_password_hash_path() -> PathBuf {
        Config::main_config_dir_path().join("pass_hash")
    }

    pub fn get_path_keys() -> PathBuf {
        return Config::main_config_dir_path().join("List");
    }

    fn text_to_bytes(text: &str) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(text.as_bytes());
        hasher.finalize().into()
    }

    pub fn decode(key: &str, ciphertext: Vec<u8>) -> String {
        let text = String::from_utf8(ciphertext).unwrap();
        let array: Vec<&str> = text.split(":").collect();
        let nonce_str = array[0];
        let cipher_str = array[1];

        let key: &[u8; 32] = &Config::text_to_bytes(key);
        let key: &Key<Aes256Gcm> = key.into();

        let cipher = Aes256Gcm::new(&key);

        let cd = hex::decode(cipher_str).unwrap();
        let nn = hex::decode(nonce_str).unwrap();
        let nd = GenericArray::from_slice(&nn);

        let plaintext = cipher.decrypt(nd, cd.as_ref()).expect("A problem has occurred during decryption. It is possible that your password does not match the encrypted key.");
        return std::str::from_utf8(&plaintext).unwrap().to_string();
    }

    pub fn encode(key: &str, content: &str) -> String {
        let key: &[u8; 32] = &Config::text_to_bytes(key);
        let key: &Key<Aes256Gcm> = key.into();
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(&nonce, content.as_ref()).unwrap();

        let nonce_string = hex::encode(nonce);

        format!("{}:{}", nonce_string, hex::encode(&ciphertext))
    }

    pub fn export() {
        let dt = Local::now();
        let formatted = dt.format("%Y_%m_%d_%H_%M_%S").to_string();

        let storage_path = dirs::download_dir();
        let storage_path = match storage_path {
            Some(p) => p,
            None => dirs::home_dir().unwrap(),
        };

        let file_name_epass = storage_path.join(format!("archive_{}.epass", formatted));

        let path = Config::get_path_keys();

        // create zip file name
        let zip_file_name = format!("archive_{}.zip", formatted);
        let zip_file_save_path = storage_path.join(zip_file_name);
        let zip_file = File::create(&zip_file_save_path).unwrap();

        let mut zip = ZipWriter::new(zip_file);

        let _ = Config::add_dir_to_zip(path.as_path(), &mut zip, &path);

        let _ = zip.finish();

        let password_file = Config::config_file_password_hash_path();
        let hash = Config::read_text_file(password_file);
        let first_30_chars = &hash[0..29];

        let _ = Config::encrypt_file(
            first_30_chars,
            &zip_file_save_path.display().to_string().as_str(),
            file_name_epass.display().to_string().as_str(),
        );
        fs::remove_file(zip_file_save_path).unwrap();
        println!(
            "The file was saved in the '{}' path",
            file_name_epass.display().to_string().bold()
        );
    }

    pub fn import() -> bool {
        // Open Dialog for select backup file
        let path = FileDialog::new()
            .set_location(dirs::home_dir().unwrap().as_path())
            .add_filter("Epass file", &["epass"])
            .show_open_single_file();

        let path = match path {
            Ok(path) => path,
            Err(_) => {
                println!("Please send epass file path:");
                let mut path = String::new();
                stdin().read_line(&mut path).unwrap();
                Some(Path::new(&path).to_path_buf())
            }
        };

        let ok = match path {
            Some(_) => true,
            None => false,
        };

        if ok == false {
            return false;
        }

        let path = path.unwrap();
        // }

        if path.exists() == false {
            println!("{}", "File not found".red());
            std::process::exit(0);
        }

        // get orginal backup file name
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        // create zip file name
        let zip_file_name = file_name.replace("epass", "zip");

        // get password hash path
        let password_file = Config::config_file_password_hash_path();

        // get password hash string
        let hash = Config::read_text_file(password_file);

        // use first 30 chars of hash
        let first_30_chars = &hash[0..29];

        // init storage zip path
        let storage_path = dirs::download_dir();
        let storage_path = match storage_path {
            Some(p) => p,
            None => dirs::home_dir().unwrap(),
        };

        // create zip storage path with file name
        let zip_file_path = storage_path.join(zip_file_name);

        // decrypt backup file and make zip
        Config::decrypt_file(
            first_30_chars,
            path.display().to_string().as_str(),
            zip_file_path.display().to_string().as_str(),
        );

        let extract_passwords_list_path = Config::get_path_keys();

        Config::unzip_file(
            zip_file_path.as_path(),
            extract_passwords_list_path.as_path(),
        );

        true
    }

    fn add_dir_to_zip(
        root: &Path,
        zip: &mut ZipWriter<File>,
        path: &Path,
    ) -> zip::result::ZipResult<()> {
        for entry in path.read_dir()? {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(root).unwrap().to_str().unwrap();

            if path.is_dir() {
                zip.add_directory(name, Default::default())?;
                Config::add_dir_to_zip(root, zip, &path)?;
            } else {
                zip.start_file(name, Default::default())?;
                let mut file = File::open(&path)?;
                std::io::copy(&mut file, zip)?;
            }
        }

        Ok(())
    }

    fn unzip_file(zip_file_path: &Path, dest_dir_path: &Path) -> std::io::Result<()> {
        let file = File::open(zip_file_path)?;
        let mut archive = ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => path.to_owned(),
                None => continue,
            };

            let mut outpath = dest_dir_path.to_path_buf().join(outpath);
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }

            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        Ok(())
    }

    pub fn encrypt_file(key: &str, input_path: &str, output_path: &str) -> Result<(), String> {
        // let key = GenericArray::from_slice(key.as_bytes());

        let key: &[u8; 32] = &Config::text_to_bytes(key);
        let key: &Key<Aes256Gcm> = key.into();

        // Read the input file
        let input_data = match Config::c_read_file(input_path) {
            Ok(data) => data,
            Err(e) => return Err(format!("Error reading input file: {}", e)),
        };

        // Generate a random nonce
        let mut rng = OsRng;
        let mut nonce_data = [0u8; 12];
        rng.fill_bytes(&mut nonce_data);
        let nonce = GenericArray::from(nonce_data);

        // Encrypt the data using AES-256-GCM cipher
        let cipher = Aes256Gcm::new(key);
        let ciphertext = match cipher.encrypt(&nonce, input_data.as_ref()) {
            Ok(data) => data,
            Err(e) => return Err(format!("Encryption error: {}", e)),
        };
        // Concatenate the nonce and ciphertext and write it to the output file
        let mut output_file = match File::create(Path::new(output_path)) {
            Ok(file) => file,
            Err(e) => return Err(format!("Error creating output file: {}", e)),
        };
        let mut encrypted_data = nonce.to_vec();
        encrypted_data.extend_from_slice(&ciphertext);
        match output_file.write_all(&encrypted_data) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error writing encrypted data to file: {}", e)),
        }
    }

    pub fn decrypt_file(key: &str, input_path: &str, output_path: &str) -> Result<(), String> {
        // let key = GenericArray::from_slice(key.as_bytes());

        let key: &[u8; 32] = &Config::text_to_bytes(key);
        let key: &Key<Aes256Gcm> = key.into();

        // Read the input file
        let input_data = match Config::c_read_file(input_path) {
            Ok(data) => data,
            Err(e) => return Err(format!("Error reading input file: {}", e)),
        };

        // Split the input data into nonce and ciphertext
        let nonce_size = 12; // We're using a 96-bit nonce
        let nonce = &input_data[..nonce_size];
        let ciphertext = &input_data[nonce_size..];

        let cipher = Aes256Gcm::new(key);
        let plaintext = match cipher.decrypt(GenericArray::from_slice(nonce), ciphertext) {
            Ok(data) => data,
            Err(e) => return Err(format!("Decryption error: {}", e)),
        };

        // Write the decrypted data to the output file
        match Config::c_write_file(output_path, &plaintext) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error writing decrypted data to file: {}", e)),
        }
    }

    fn c_read_file(path: &str) -> Result<Vec<u8>, String> {
        let mut file = match File::open(Path::new(path)) {
            Ok(file) => file,
            Err(e) => return Err(format!("Error opening file: {}", e)),
        };
        let mut data = Vec::new();
        match file.read_to_end(&mut data) {
            Ok(_) => Ok(data),
            Err(e) => Err(format!("Error reading file: {}", e)),
        }
    }

    fn c_write_file(path: &str, data: &[u8]) -> Result<(), String> {
        let mut file = match File::create(Path::new(path)) {
            Ok(file) => file,
            Err(e) => return Err(format!("Error creating file: {}", e)),
        };
        match file.write_all(data) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error writing to file: {}", e)),
        }
    }
}
