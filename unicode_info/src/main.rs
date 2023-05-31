mod utf_info;
mod unicode_data;
mod file_utils;
mod help;

use std::env;
use help::Flags;
use regex::Regex;

const FILE_UNICODE_DATA:&str = "UnicodeData.txt";
const FILE_BLOCKS:&str = "Blocks.txt";
const FILE_NAMES_LIST:&str = "NamesList.txt";

const BAD_CODE_POINT:u32 = 0xffffffff;

fn get_unicode_data(code_point:u32) {
    let cp_str = format!("{:04X}", code_point);
    let data_path = file_utils::get_data_path(&FILE_UNICODE_DATA.to_string());
    let lines = file_utils::read_lines(&data_path);
    let mut start_cp:u32 = BAD_CODE_POINT;
    for line in lines {
        let line_content = line.unwrap();
        if line_content.starts_with(&cp_str) {
            let parts: Vec<&str> = line_content.split(';').collect();
            if parts[0].eq(&cp_str) {
                unicode_data::write_cp_entry(parts);
                return;
            }
        } else if line_content.contains(" First>;") {
            let parts: Vec<&str> = line_content.split(';').collect();
            start_cp = u32::from_str_radix(&parts[0], 16).unwrap();
        } else if line_content.contains(" Last>;") {
            let mut parts: Vec<&str> = line_content.split(';').collect();
            let end_cp = u32::from_str_radix(&parts[0], 16).unwrap();
            if start_cp != BAD_CODE_POINT {
                if start_cp <= code_point && code_point <= end_cp {
                    let tmp = parts[1].replace(", Last>", ">");
                    parts[1] = tmp.as_str();
                    unicode_data::write_cp_entry(parts);
                    return;
                }
            }
            start_cp = BAD_CODE_POINT;
        }
    }
}

fn get_block_name(code_point:u32) {
    let re_block_info:Regex = Regex::new(r"^([0-9A-F]{4,6})\.\.([0-9A-F]{4,6}); (.+)$").unwrap();
    let data_path = file_utils::get_data_path(&FILE_BLOCKS.to_string());
    let lines = file_utils::read_lines(&data_path);
    for line in lines {
        let line_content = line.unwrap();
        for cap in re_block_info.captures_iter(line_content.as_str()) {
            let start = u32::from_str_radix(&cap[1], 16).unwrap();
            let end = u32::from_str_radix(&cap[2], 16).unwrap();
            if code_point >= start && code_point <= end {
                println!("  \x1b[93mBlock   :\x1b[m {}", &cap[3]);
                continue;
            }
        }
    }
}

fn get_from_namelist(code_point:u32) {
    let cp_str = format!("{:04X}\t", code_point);
    let data_path = file_utils::get_data_path(&FILE_NAMES_LIST.to_string());
    let lines = file_utils::read_lines(&data_path);
    let mut start_printing = false;
    for line in lines {
        let line_content = line.unwrap();
        if line_content.starts_with(&cp_str) {
            println!("  ------------------");
            println!("  \x1b[93mFrom NamesList.txt:\x1b[m");
            println!("    {}", &line_content);
            start_printing = true;
        } else if start_printing {
            if line_content.starts_with("\t") || line_content.starts_with("@") {
                println!("    {}", &line_content);
            } else {
                return;
            }
        }
    }
}

fn get_char_info(code_point:u32, flags:&Flags) {
    let chr = char::from_u32(code_point);
    if chr == None {
        println!("===== Invalid code point: {:04X}h ==========", code_point);
    } else {
        println!("===== {} ==========", chr.unwrap());
        utf_info::write_utf(code_point);
        if flags.show_block_info {
            println!("  ------------------");
            get_block_name(code_point);
        }
        if flags.show_unicode_data {
            println!("  ------------------");
            get_unicode_data(code_point);
        }
        if flags.show_name_info {
            get_from_namelist(code_point);
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
        let flags = help::Flags::new();
        for c in "Hello 日本語 💩".chars() {
            get_char_info(c as u32, &flags);
        }
    }

    #[test]
    fn test_cp() {
        let flags = help::Flags::new();
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
  - String : "Hello world!"
- Handle `First>` and `Last>` in UnicodeData.txt, for example:
    AC00;<Hangul Syllable, First>;Lo;0;L;;;;;N;;;;;
    D7A3;<Hangul Syllable, Last>;Lo;0;L;;;;;N;;;;;
- Handle `NamesList.txt`
- Command line : specify what to output
- Color
 */
