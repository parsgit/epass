use std::{
    fs,
    path::{Path, PathBuf},
};

use sha2::Sha256;
// use hex_literal::hex;
use sha3::{Digest, Sha3_256};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

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

    pub fn config_file_storage_path() -> PathBuf {
        Config::main_config_dir_path().join("storage_kyes_path")
    }

    pub fn config_file_password_hash_path() -> PathBuf {
        Config::main_config_dir_path().join("pass_hash")
    }

    pub fn default_storage_keys_path() -> PathBuf {
        let documents = dirs::home_dir().unwrap().join("Documents");
        return documents;
    }

    pub fn get_path_keys() -> PathBuf {
        let file = Config::config_file_storage_path();
        let content = fs::read_to_string(file).unwrap();
        return Path::new(content.trim()).to_path_buf();
    }

    fn text_to_bytes(text: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(text.as_bytes());
        hasher.finalize().into()
    }

    pub fn decode(key :&str, ciphertext:Vec<u8>)->String {
        let key: &[u8; 32] = &Config::text_to_bytes(key);
        let key: &Key<Aes256Gcm> = key.into();
        println!("1");
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        println!("2");

        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref()).unwrap();
        println!("{:?}",plaintext);
        return std::str::from_utf8(&plaintext).unwrap().to_string();
    }

    pub fn encode(key: &str, content: &str) -> Vec<u8> {
        let key: &[u8; 32] = &Config::text_to_bytes(key);
        let key: &Key<Aes256Gcm> = key.into();
        let cipher = Aes256Gcm::new(&key);
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(&nonce, content.as_ref()).unwrap();

        return ciphertext;
    }
}
