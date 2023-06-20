use std::{
    fs::{self, File},
    path::{Path, PathBuf}, os::unix::prelude::OpenOptionsExt,
};

use chrono::Local;
// use hex_literal::hex;
use sha3::{Digest, Sha3_256};

use aes_gcm::{
    aead::{generic_array::GenericArray, Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use zip::{write::FileOptions, CompressionMethod, ZipWriter};
// use zip::write::{FileOptions, ZipWriter};
// use zip::CompressionMethod;

use aes_gcm::aead::{ AeadInPlace};
use rand::{Rng, thread_rng};
use std::fs::{ OpenOptions};
use std::io::{Read, Write};

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

        let plaintext = cipher.decrypt(nd, cd.as_ref()).unwrap();
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






}
