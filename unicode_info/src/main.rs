use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use regex::Regex;

const FILE_UNICODE_DATA:&str = "UnicodeData.txt";
const FILE_BLOCKS:&str = "Blocks.txt";
const FILE_NAMES_LIST:&str = "NamesList.txt";

const BAD_CODE_POINT:u32 = 0xffffffff;

fn read_lines(data_path:&Path) -> io::Lines<BufReader<File>> {
    let file = File::open(data_path).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

fn get_data_path(file_name:&String) -> PathBuf {
    return env::current_exe().unwrap()
        .parent().unwrap()
        .join("udata")
        .join(file_name);
}

// Bidi_Class (bc)
fn bc_to_string(short_id:&str) -> &str {
    return match short_id.trim() {
        "AL" => "Arabic Letter",
        "AN" => "Arabic Number",
        "B" => "Paragraph Separator",
        "BN" => "Boundary Neutral",
        "CS"  => "Common Separator",
        "EN"  => "European Number",
        "ES"  => "European Separator",
        "ET"  => "European Terminator",
        "FSI" => "First Strong Isolate",
        "L"   => "Left To Right",
        "LRE" => "Left To Right Embedding",
        "LRI" => "Left To Right Isolate",
        "LRO" => "Left To Right Override",
        "NSM" => "Nonspacing Mark",
        "ON"  => "Other Neutral",
        "PDF" => "Pop Directional Format",
        "PDI" => "Pop Directional Isolate",
        "R"   => "Right To Left",
        "RLE" => "Right To Left Embedding",
        "RLI" => "Right To Left Isolate",
        "RLO" => "Right To Left Override",
        "S"   => "Segment Separator",
        "WS"  => "White Space",
        _     => "???"
    }
}

fn gc_to_string(short_id:&str) -> &str {
    return match short_id.trim() {
        "C"  => "Other # Cc | Cf | Cn | Co | Cs",
        "Cc" => "Control ; cntrl",
        "Cf" => "Format",
        "Cn" => "Unassigned",
        "Co" => "Private Use",
        "Cs" => "Surrogate",
        "L"  => "Letter # Ll | Lm | Lo | Lt | Lu",
        "LC" => "Cased Letter # Ll | Lt | Lu",
        "Ll" => "Lowercase Letter",
        "Lm" => "Modifier Letter",
        "Lo" => "Other Letter",
        "Lt" => "Titlecase Letter",
        "Lu" => "Uppercase Letter",
        "M"  => "Mark ; Combining Mark # Mc | Me | Mn",
        "Mc" => "Spacing Mark",
        "Me" => "Enclosing Mark",
        "Mn" => "Nonspacing Mark",
        "N"  => "Number # Nd | Nl | No",
        "Nd" => "Decimal Number ; digit",
        "Nl" => "Letter Number",
        "No" => "Other Number",
        "P"  => "Punctuation ; punct # Pc | Pd | Pe | Pf | Pi | Po | Ps",
        "Pc" => "Connector Punctuation",
        "Pd" => "Dash Punctuation",
        "Pe" => "Close Punctuation",
        "Pf" => "Final Punctuation",
        "Pi" => "Initial Punctuation",
        "Po" => "Other Punctuation",
        "Ps" => "Open Punctuation",
        "S"  => "Symbol # Sc | Sk | Sm | So",
        "Sc" => "Currency Symbol",
        "Sk" => "Modifier Symbol",
        "Sm" => "Math Symbol",
        "So" => "Other Symbol",
        "Z"  => "Separator # Zl | Zp | Zs",
        "Zl" => "Line Separator",
        "Zp" => "Paragraph Separator",
        "Zs" => "Space Separator",
        _     => "???"
    }
}

fn ccc_to_string(short_id:&str) -> &str {
    return match short_id.trim() {
        "0"   => "NR ; Not Reordered ; Spacing, split, enclosing, reordrant, and Tibetan subjoined",
        "1"   => "OV ; Overlays and interior",
        "6"   => "HANR ; Han Reading",
        "7"   => "NK ; Nuktas",
        "8"   => "KV ; Hiragana/Katakana voicing marks",
        "9"   => "VR ; Viramas",
        "10"  => "CCC10 ; Fixed position class",
        "11"  => "CCC11 ; Fixed position class",
        "12"  => "CCC12 ; Fixed position class",
        "13"  => "CCC13 ; Fixed position class",
        "14"  => "CCC14 ; Fixed position class",
        "15"  => "CCC15 ; Fixed position class",
        "16"  => "CCC16 ; Fixed position class",
        "17"  => "CCC17 ; Fixed position class",
        "18"  => "CCC18 ; Fixed position class",
        "19"  => "CCC19 ; Fixed position class",
        "20"  => "CCC20 ; Fixed position class",
        "21"  => "CCC21 ; Fixed position class",
        "22"  => "CCC22 ; Fixed position class",
        "23"  => "CCC23 ; Fixed position class",
        "24"  => "CCC24 ; Fixed position class",
        "25"  => "CCC25 ; Fixed position class",
        "26"  => "CCC26 ; Fixed position class",
        "27"  => "CCC27 ; Fixed position class",
        "28"  => "CCC28 ; Fixed position class",
        "29"  => "CCC29 ; Fixed position class",
        "30"  => "CCC30 ; Fixed position class",
        "31"  => "CCC31 ; Fixed position class",
        "32"  => "CCC32 ; Fixed position class",
        "33"  => "CCC33 ; Fixed position class",
        "34"  => "CCC34 ; Fixed position class",
        "35"  => "CCC35 ; Fixed position class",
        "36"  => "CCC36 ; Fixed position class",
        "84"  => "CCC84 ; Fixed position class",
        "91"  => "CCC91 ; Fixed position class",
        "103" => "CCC103 ; Fixed position class",
        "107" => "CCC107 ; Fixed position class",
        "118" => "CCC118 ; Fixed position class",
        "122" => "CCC122 ; Fixed position class",
        "129" => "CCC129 ; Fixed position class",
        "130" => "CCC130 ; Fixed position class",
        "132" => "CCC132 ; Fixed position class",
        "133" => "CCC133 ; Fixed position class # RESERVED ",
        "200" => "ATBL ; Attached Below Left",
        "202" => "ATB ; Attached Below",
        "204" => "ATBR ; Attached Below Right",
        "208" => "ATL; Attached Left (reordrant around single base character)",
        "210" => "ATR ; Attached Right",
        "212" => "ATAL ; Attached Above Left",
        "214" => "ATA ; Attached Above",
        "216" => "ATAR ; Attached Above Right",
        "218" => "BL ; Below Left",
        "220" => "B ; Below",
        "222" => "BR ; Below Right",
        "224" => "L ; Left",
        "226" => "R ; Right",
        "228" => "AL ; Above Left",
        "230" => "A ; Above",
        "232" => "AR ; Above Right",
        "233" => "DB ; Double Below",
        "234" => "DA ; Double Above",
        "240" => "IS ; Iota Subscript",
        _     => "???"
    }
}

fn bool_to_string(short_id:&str) -> &str {
    return match short_id.trim() {
        "N"   => "No ; F ; False",
        "Y"   => "Yes ; T ; True",
        _     => "???"
    }
}

fn dump_value2(label:&str, value:&str, disp_func: fn(&str) -> &str) {
    if !value.is_empty() {
        println!("  \x1b[93m{} :\x1b[m {} ; {}", label, value, disp_func(value));
    }
}

fn dump_value1(label:&str, value:&str) {
    if !value.is_empty() {
        println!("  \x1b[93m{} :\x1b[m {}", label, value);
    }
}

fn write_cp_entry(parts: Vec<&str>) {
    dump_value1("Character name                 ", parts[1]);
    dump_value2("General Category (gc)          ", parts[2], gc_to_string);
    dump_value2("Canonical Combining Class (ccc)", parts[3], ccc_to_string);
    dump_value2("Bidirectional Class (bc)       ", parts[4], bc_to_string);
    dump_value1("Decomposition Mapping (dm)     ", parts[5]);
    dump_value1("Decimal digit value            ", parts[6]);
    dump_value1("Digit value                    ", parts[7]);
    dump_value1("Numeric value (nv)             ", parts[8]);
    dump_value2("Mirrored (Bidi_M)              ", parts[9], bool_to_string);
    dump_value1("Unicode 1.0 Name (na1)         ", parts[10]);
    dump_value1("10646 comment field            ", parts[11]);
    dump_value1("Simple Uppercase mapping (suc) ", parts[12]);
    dump_value1("Simple Lowercase mapping (slc) ", parts[13]);
    dump_value1("Simple Titlecase mapping (stc) ", parts[14]);
}

fn get_unicode_data(code_point:u32) {
    let cp_str = format!("{:04X}", code_point);
    let data_path = get_data_path(&FILE_UNICODE_DATA.to_string());
    let lines = read_lines(&data_path);
    let mut start_cp:u32 = BAD_CODE_POINT;
    for line in lines {
        let line_content = line.unwrap();
        if line_content.starts_with(&cp_str) {
            let parts: Vec<&str> = line_content.split(';').collect();
            if parts[0].eq(&cp_str) {
                write_cp_entry(parts);
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
                    write_cp_entry(parts);
                    return;
                }
            }
            start_cp = BAD_CODE_POINT;
        }

    }
}

fn write_utf(code_point:u32) {
    println!("  \x1b[93mDecimal :\x1b[m {}", code_point);

    // UTF-8
    print!("  \x1b[93mUTF-8   :\x1b[m ");
    if code_point < 0x80 { // One byte UTF-8 char
        println!("{:02X}", code_point);
    } else if code_point < 0x0800 { // Two byte UTF-8 char
        println!("{:02X} {:02X}",
            0b1100_0000 | 0b0001_1111 & code_point >> 6,
            0b1000_0000 | 0b0011_1111 & code_point
        );
    } else if code_point < 0x010000 { // Three byte UTF-8 char
        println!("{:02X} {:02X} {:02X}",
            0b1110_0000 | 0b0000_1111 & code_point >> 12,
            0b1000_0000 | 0b0011_1111 & code_point >> 6,
            0b1000_0000 | 0b0011_1111 & code_point
        );
    } else if code_point < 0x110000 { // Four byte UTF-8 char
        println!("{:02X} {:02X} {:02X} {:02X}",
            0b1111_0000 | 0b0000_0111 & code_point >> 18,
            0b1000_0000 | 0b0011_1111 & code_point >> 12,
            0b1000_0000 | 0b0011_1111 & code_point >> 6,
            0b1000_0000 | 0b0011_1111 & code_point
        );
    } else {
        println!("Invalid codepoint (above 0x10FFFF)");
    }

    // UTF-16
    print!("  \x1b[93mUTF-16  :\x1b[m ");
    if code_point >= 0xD800 && code_point <= 0xDFFF {
        println!("Invalid codepoint (in the surrogate range)");
	} else if code_point > 0x10FFFF {
	    println!("Invalid codepoint (above 0x10FFFF)");
    } else if code_point <= 0xFFFF {
        println!("{:04X}", code_point);
	} else {
	    /* 0xFFFF - 0x10FFFF, surrogates excluded */
	    let code_point1 = code_point - 0x10000;
        println!("{:04X} {:04X}", 
	        ((code_point1 >> 10) + 0xD800),
	        ((code_point1 & 0x3FF) + 0xDC00));
    }

    println!("  \x1b[93mUTF-32  :\x1b[m {:08X}", code_point);
}

fn get_block_name(code_point:u32) {
    let re_block_info:Regex = Regex::new(r"^([0-9A-F]{4,6})\.\.([0-9A-F]{4,6}); (.+)$").unwrap();
    let data_path = get_data_path(&FILE_BLOCKS.to_string());
    let lines = read_lines(&data_path);
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
    let data_path = get_data_path(&FILE_NAMES_LIST.to_string());
    let lines = read_lines(&data_path);
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

fn get_char_info(code_point:u32) {
    println!("===== {} ==========", char::from_u32(code_point).unwrap());
    write_utf(code_point);
    println!("  ------------------");
    get_block_name(code_point);
    println!("  ------------------");
    get_unicode_data(code_point);
    get_from_namelist(code_point);
}

fn main() {
    let re_hex_cp:Regex = Regex::new(r"^([0-9A-Fa-f]{4,6})$").unwrap();
    let args: Vec<String> = env::args().collect();

    for arg in &args {
        let sarg = arg.as_str();
        if re_hex_cp.is_match(sarg) {
            let code_point = u32::from_str_radix(&sarg, 16).unwrap();
            get_char_info(code_point);
        } else {
            for ch in sarg.chars() {
                get_char_info(ch as u32);
            }
        }
    }

    // for c in "Hello 日本語 💩".chars() {
    //     get_char_info(c as u32);
    // }

    // get_char_info(0x0048); // LATIN CAPITAL LETTER H;Lu;0;L;;;;;N;;;;0068;
    // get_char_info(0x00FB); // LATIN SMALL LETTER U WITH CIRCUMFLEX;Ll;0;L;0075 0302;;;;N;LATIN SMALL LETTER U CIRCUMFLEX;;00DB;;00DB
    // get_char_info(0x00DA);
    // get_char_info(0x0664); // ARABIC-INDIC DIGIT FOUR;Nd;0;AN;;4;4;4;N;;;;;
    // get_char_info(0x0218); // LATIN CAPITAL LETTER S WITH COMMA BELOW;Lu;0;L;0053 0326;;;;N;;;;0219;
    // get_char_info(0x2743); // HEAVY TEARDROP-SPOKED PINWHEEL ASTERISK;So;0;ON;;;;;N;;;;;
    // get_char_info(0xA64F); // CYRILLIC SMALL LETTER NEUTRAL YER;Ll;0;L;;;;;N;;;A64E;;A64E
    // get_char_info(0xFD13); // ARABIC LIGATURE AIN WITH ALEF MAKSURA FINAL FORM;Lo;0;AL;<final> 0639 0649;;;;N;;;;;
    // get_char_info(0x1FBC3); // RIGHT THIRD WHITE RIGHT POINTING INDEX;So;0;ON;;;;;N;;;;;
    // get_char_info(0x2F80B); // CJK COMPATIBILITY IDEOGRAPH-2F80B;Lo;0;L;50CF;;;;N;;;;;
    // get_char_info(0xF0000); // <Plane 15 Private Use, First>;Co;0;L;;;;;N;;;;;
    // get_char_info(0xFFF00); // <Plane 15 Private Use, ?>;Co;0;L;;;;;N;;;;;
    // get_char_info(0xFFFFD); // <Plane 15 Private Use, Last>;Co;0;L;;;;;N;;;;;
    // get_char_info(0x100000); // <Plane 16 Private Use, First>;Co;0;L;;;;;N;;;;;
    // get_char_info(0x10FFFD); // <Plane 16 Private Use, Last>;Co;0;L;;;;;N;;;;;
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
