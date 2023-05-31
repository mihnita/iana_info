use crate::color_utils;
use crate::help;

pub fn write_utf(code_point:u32, flags:&help::Flags) {
    if flags.show_dec {
        println!("  {} {}", color_utils::label(&"Decimal :".to_string(), flags), code_point);
    }

    let ch = char::from_u32(code_point).unwrap();

    // UTF-8
    if flags.show_utf8 {
        print!("{}", color_utils::label(&"  UTF-8   :".to_string(), flags));
        let mut utf8_bytes = [0; 10];
        ch.encode_utf8(&mut utf8_bytes);
        for i in 0..ch.len_utf8() {
            print!(" {:02X}", utf8_bytes[i]);
        }
        println!();
    }

    // UTF-16
    if flags.show_utf16 {
        print!("{}", color_utils::label(&"  UTF-16  :".to_string(), flags));
        let mut utf16_code_units = [0; 10];
        ch.encode_utf16(&mut utf16_code_units);
        for i in 0..ch.len_utf16() {
            print!(" {:04X}", utf16_code_units[i]);
        }
        println!();
    }

    // UTF-32
    if flags.show_utf32 {
        println!("{} {:08X}", color_utils::label(&"  UTF-32  :".to_string(), flags), code_point);
    }
}
