mod block_name;
mod color_utils;
mod file_utils;
mod help;
mod name_list;
mod unicode_data;
mod utf_info;

use std::env;
use help::Flags;
use regex::Regex;

fn get_char_info(code_point:u32, flags:&Flags) {
    let chr = char::from_u32(code_point);
    if chr == None {
        println!("===== Invalid code point: {:04X}h ==========", code_point);
    } else {
        println!("===== {} ==========", chr.unwrap());
        if flags.show_utf {
            utf_info::write_utf(code_point, flags);
            println!("----------------------");
        }
        if flags.show_unicode_data {
            unicode_data::get_unicode_data(code_point, flags);
            println!("----------------------");
        }
        if flags.show_block_info {
            block_name::get_block_name(code_point, flags);
            println!("----------------------");
        }
        if flags.show_name_info {
            name_list::get_from_namelist(code_point, flags);
            println!("----------------------");
        }
    }
}

fn main() {
    let re_hex_cp:Regex = Regex::new(r"^([0-9A-Fa-f]{4,6})$").unwrap();
    let args: Vec<String> = env::args().collect();

    let flags = help::parse_args(&args);
    for arg in &flags.rest {
        let sarg = arg.as_str();
        if re_hex_cp.is_match(sarg) {
            let code_point = u32::from_str_radix(&sarg, 16).unwrap();
            get_char_info(code_point, &flags);
        } else {
            for ch in sarg.chars() {
                get_char_info(ch as u32, &flags);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string() {
        let flags = help::Flags::new(true);
        for c in "Hello 日本語 💩".chars() {
            get_char_info(c as u32, &flags);
        }
    }

    #[test]
    fn test_cp() {
        let flags = help::Flags::new(true);
        get_char_info(0x0048, &flags); // LATIN CAPITAL LETTER H;Lu;0;L;;;;;N;;;;0068;
        get_char_info(0x00FB, &flags); // LATIN SMALL LETTER U WITH CIRCUMFLEX;Ll;0;L;0075 0302;;;;N;LATIN SMALL LETTER U CIRCUMFLEX;;00DB;;00DB
        get_char_info(0x00DA, &flags);
        get_char_info(0x0664, &flags); // ARABIC-INDIC DIGIT FOUR;Nd;0;AN;;4;4;4;N;;;;;
        get_char_info(0x0218, &flags); // LATIN CAPITAL LETTER S WITH COMMA BELOW;Lu;0;L;0053 0326;;;;N;;;;0219;
        get_char_info(0x2743, &flags); // HEAVY TEARDROP-SPOKED PINWHEEL ASTERISK;So;0;ON;;;;;N;;;;;
        get_char_info(0xA64F, &flags); // CYRILLIC SMALL LETTER NEUTRAL YER;Ll;0;L;;;;;N;;;A64E;;A64E
        get_char_info(0xFD13, &flags); // ARABIC LIGATURE AIN WITH ALEF MAKSURA FINAL FORM;Lo;0;AL;<final> 0639 0649;;;;N;;;;;
        get_char_info(0x1FBC3, &flags); // RIGHT THIRD WHITE RIGHT POINTING INDEX;So;0;ON;;;;;N;;;;;
        get_char_info(0x2F80B, &flags); // CJK COMPATIBILITY IDEOGRAPH-2F80B;Lo;0;L;50CF;;;;N;;;;;
        get_char_info(0xF0000, &flags); // <Plane 15 Private Use, First>;Co;0;L;;;;;N;;;;;
        get_char_info(0xFFF00, &flags); // <Plane 15 Private Use, ?>;Co;0;L;;;;;N;;;;;
        get_char_info(0xFFFFD, &flags); // <Plane 15 Private Use, Last>;Co;0;L;;;;;N;;;;;
        get_char_info(0x100000, &flags); // <Plane 16 Private Use, First>;Co;0;L;;;;;N;;;;;
        get_char_info(0x10FFFD, &flags); // <Plane 16 Private Use, Last>;Co;0;L;;;;;N;;;;;
        // Error, invalid code points
        get_char_info(0xD83D, &flags);
        get_char_info(0xDCA9, &flags);
        get_char_info(0x12FAFD, &flags);
    }
}

/* TODO:
- Command line
  - decimal: <123 342 11> (?)
  - utf-8  : <94 C3 D8 A1 65 49 32>
  - utf-16 : <94C3 D8A1 6542>
  - utf-32 : <000094C3 001D8A1>
 */
