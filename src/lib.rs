mod utils;
use rand;
use chrono::{Datelike, Timelike, Utc};

use wasm_bindgen::prelude::*; 

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;  

const PASSWORDS: [&'static str; 10] = [
    "8633D41B73FB7E5E",
    "CFF801D90DE6B118",
    "6CBC2DF4815ABE7C",
    "8C7B29210529736E",
    "C028A0575A0FE80C",
    "E4510D163F333D1F",
    "22101A5EDA86E42F",
    "405E9F7AD2A5A5E3",
    "3A09CA044F8EDD32",
    "6C35E809D2768C77"
];

#[wasm_bindgen]
pub fn get_password() -> String {
    let random : [u8; 8] = rand::random();

    let mut str_bytes = vec![0u8; 16];

    const ASCII_ZERO: u8 = '0' as u8;
    const ASCII_NINE: u8 = '9' as u8;
    const ASCII_NUMBERS_LETTERS_OFFSET: u8 = 'A' as u8 - '9' as u8 - 1;

    for i in 0..8 {
        let mut leading = random[i] / 16 + ASCII_ZERO;
        let mut trailing = random[i] % 16 + ASCII_ZERO;

        leading += ((leading > ASCII_NINE) as u8) * ASCII_NUMBERS_LETTERS_OFFSET;
        trailing += ((trailing > ASCII_NINE) as u8) * ASCII_NUMBERS_LETTERS_OFFSET;

        str_bytes[2 * i] = leading;
        str_bytes[2 * i + 1] = trailing;
    }

    unsafe { String::from_utf8_unchecked(str_bytes) } 
}

#[wasm_bindgen]
pub fn get_rotated_password() -> String  { 
    let now = Utc::now();
    let min = now.minute() as usize; 
    let index = min % 10;
    return PASSWORDS[index].to_string();
}

