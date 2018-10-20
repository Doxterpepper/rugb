use std::slice::*;
use std::str;

/// read the title from the cartridge
pub fn read_title(cartridge: Vec<u8>) -> String {
    let title_offset = 308;
    let title_slice = &cartridge[title_offset..title_offset+15];
    String::from(str::from_utf8(title_slice).unwrap())
}
