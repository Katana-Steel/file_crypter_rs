/*
  File Crypter - entry point
  copyright (C) 2024 Rene Kjellerup
  This software is licensed under the GNU GPL version 3.0 or later.
  see the LICENSE file for more information
*/
use std::path::Path;
use std::env;
pub mod sbox;
pub mod encrypter;


fn sub_and_print_string(sb: &sbox::SBox, test_string: &str) {
    let mut decrypted: String = String::new();
    for k in test_string.bytes() {
        let sub = sb.substitute(k as usize);
        decrypted.push(sb.inv_substitute(sub) as char);
        println!("sub 0x{:02x}", sub);
    }
    println!("Original:  {}", test_string);
    println!("Decrypted: {}", decrypted);
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let key: &Path = Path::new("sbox.key");
    let sb: sbox::SBox = sbox::SBox::initiate(key);
    sub_and_print_string(&sb, "Hello, world!");
    if args.len() > 1 {
        for test_input in args.iter().skip(1) {
            sub_and_print_string(&sb, &test_input);
        }
    }
}
