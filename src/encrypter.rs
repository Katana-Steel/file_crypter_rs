/*
  File Crypter - encryption module
  copyright (C) 2024 Rene Kjellerup
  This software is licensed under the GNU GPL version 3.0 or later.
  see the LICENSE file for more information
*/
use super::sbox::SBox;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use sha2::{Sha224, Digest};

struct Encrypter {
    sbox: SBox,
}

impl Encrypter {
    pub fn new(key: &Path) -> Encrypter {
        Encrypter {
            sbox: SBox::initiate(key),
        }
    }

    pub fn encrypt(&self, input: &str, cur: u8) -> (Vec<u8>, u8) {
        let mut encrypted: Vec<u8> = Vec::<u8>::new();
        let mut last_byte: u8 = cur;
        for k in input.bytes() {
            let tmp = self.sbox.substitute(k as usize);
            let sub = tmp ^ self.sbox.substitute(last_byte as usize) ;
            encrypted.push(sub);
            if last_byte == 255 {
                last_byte = 0;
            } else {
                last_byte += 1;
            }
        }
        (encrypted, last_byte)
    }

    pub fn decrypt(&self, input: &Vec<u8>, cur: u8) -> (String, u8) {
        let mut decrypted: String = String::new();
        let mut last_byte: u8 = cur;
        for &k in input.iter() {
            let tmp: u8 = k ^ self.sbox.substitute(last_byte as usize);
            let sub = self.sbox.inv_substitute(tmp);
            decrypted.push(sub as char);
            if last_byte == 255 {
                last_byte = 0;
            } else {
                last_byte += 1;
            }
        }
        (decrypted, last_byte)
    }

    pub fn enc(&mut self, src: &Path, dest: &Path) {
        let mut src_file = File::open(src).unwrap();
        let mut dest_file = File::create(dest).unwrap();
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut dest_buffer: [u8; 1024] = [0; 1024];
        let mut last_byte: u8 = 0;
        loop {
            let bytes_read = src_file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            for i in 0..bytes_read {
                let tmp = self.sbox.substitute(buffer[i] as usize);
                dest_buffer[i] = tmp ^ self.sbox.substitute(last_byte as usize);
                if last_byte == 255 {
                    last_byte = 0;
                } else {
                    last_byte += 1;
                }
            }
            dest_file.write(&dest_buffer[0..bytes_read]).unwrap();
        }
    }

    pub fn dec(&mut self, src: &Path, dest: &Path) {
        let mut src_file = File::open(src).unwrap();
        let mut dest_file = File::create(dest).unwrap();
        let mut buffer: [u8; 1024] = [0; 1024];
        let mut dest_buffer: [u8; 1024] = [0; 1024];
        let mut last_byte: u8 = 0;
        loop {
            let bytes_read = src_file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            for i in 0..bytes_read {
                let tmp: u8 = buffer[i] ^ self.sbox.substitute(last_byte as usize);
                dest_buffer[i] = self.sbox.inv_substitute(tmp);
                if last_byte == 255 {
                    last_byte = 0;
                } else {
                    last_byte += 1;
                }
            }
            dest_file.write(&dest_buffer[0..bytes_read]).unwrap();
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypter() {
        let key: &Path = Path::new("sbox.key");
        let enc = Encrypter::new(key);
        let test_string = "Hello, world!";
        let (encrypted, _count) = enc.encrypt(test_string, 0);
        for &k in encrypted.iter() {
            assert_ne!(k, test_string.bytes().next().unwrap());
        }
        let (decrypted, _count) = enc.decrypt(&encrypted, 0);
        assert_eq!(test_string, decrypted);
    }

    #[test]
    fn test_files() {
        let key: &Path = Path::new("sbox.key");
        let mut enc = Encrypter::new(key);
        test_enc(&mut enc);
        test_dec(&mut enc);
        let src_hash = sha_hash(Path::new("src/main.rs"));
        let dest_hash = sha_hash(Path::new("./target/main.rs.dec"));
        assert_eq!(src_hash, dest_hash);
    }

    fn test_enc(enc: &mut Encrypter) { // filebased encryption test
        let src: &Path = Path::new("src/main.rs");
        let dest: &Path = Path::new("./target/main.rs.enc");
        enc.enc(src, dest);
    }

    fn test_dec(enc: &mut Encrypter) { // filebased decryption test
        let src: &Path = Path::new("./target/main.rs.enc");
        let dest: &Path = Path::new("./target/main.rs.dec");
        enc.dec(src, dest);
    }

    fn sha_hash(file: &Path) -> String {
        let mut file = File::open(file).unwrap();
        let mut hasher = Sha224::new();
        let mut buffer: [u8; 1024] = [0; 1024];
        loop {
            let bytes_read = file.read(&mut buffer).unwrap();
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[0..bytes_read]);
        }
        format!("{hasher:?}")
    }
}
