pub fn write_utf(code_point:u32) {
    println!("  \x1b[93mDecimal :\x1b[m {}", code_point);

    let ch = char::from_u32(code_point).unwrap();

    // UTF-8
    print!("  \x1b[93mUTF-8   :\x1b[m");
    let mut utf8_bytes = [0; 10];
    ch.encode_utf8(&mut utf8_bytes);
    for i in 0..ch.len_utf8() {
        print!(" {:02X}", utf8_bytes[i]);
    }
    println!();

    // UTF-16
    print!("  \x1b[93mUTF-16  :\x1b[m");
    let mut utf16_code_units = [0; 10];
    ch.encode_utf16(&mut utf16_code_units);
    for i in 0..ch.len_utf16() {
        print!(" {:04X}", utf16_code_units[i]);
    }
    println!();

    // UTF-32
    println!("  \x1b[93mUTF-32  :\x1b[m {:08X}", code_point);
}
