/*
  File Crypter - SBox module
  copyright (C) 2024 Rene Kjellerup
  This software is licensed under the GNU GPL version 3.0 or later.
  see the LICENSE file for more information
*/
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
extern crate rand;
use rand::prelude::*;

pub struct SBox {
    sbox: [u8; 256],
}

impl SBox {
    pub fn initiate(sbox_file: &Path) -> SBox {
        let mut sb: [u8; 256] = [0; 256];
        match File::open(sbox_file)
        {
            Ok(mut file) => { // key file exists, read sbox from it.
                    let _ = file.read(&mut sb);
            },
            Err(_e) => sb = SBox::generate_sbox(), // key file does not exist, generate sbox.
        }
        match File::create(sbox_file)
        {
            Ok(mut file) => {
                let _ = file.write(&sb);
            },
            Err(e) => {
                println!("Error creating file: {}", e);
            }
        }
        SBox { sbox: sb }
    }

    fn is_contained(sb: &[u8; 256], candidate: u8, before: usize) -> bool {
        for j in 0..before {
            if sb[j] == candidate {
                return true;
            }
        }
        false
    }

    fn generate_sbox() -> [u8; 256] {
        let mut sb: [u8; 256] = [0; 256];
        for i in 0..256 {
            'get_rnd: loop {
                let r: u8 = random();
                if i != 0 && SBox::is_contained(&sb, r, i) {
                    continue 'get_rnd;
                }
                sb[i] = r;
                break 'get_rnd;
            }
        }
        sb
    }

    pub fn substitute(&self, input: usize) -> u8 {
        let idx = input & 0xFF;
        self.sbox[idx]
    }

    pub fn inv_substitute(&self, input: u8) -> u8 {
        for i in 0..255 {
            if self.substitute(i) == input {
                return i as u8;
            }
        }
        panic!("Error: Could not find inverse substitution for input: {}", input);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sbox() {
        let key: &Path = Path::new("sbox.key");
        let sb: SBox = SBox::initiate(key);
        for i in 0..255 {
            let sub = sb.substitute(i);
            assert_eq!(i as u8, sb.inv_substitute(sub));
        }
    }
}