use crate::color_utils;
use crate::file_utils;
use crate::help;

const FILE_UNICODE_DATA:&str = "UnicodeData.txt";

const BAD_CODE_POINT:u32 = 0xffffffff;

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

fn dump_value2(label:&str, value:&str, disp_func: fn(&str) -> &str, flags:&help::Flags) {
    if !value.is_empty() {
        println!("  {} : {} ; {}", color_utils::label(&label.to_string(), flags), value, disp_func(value));
    }
}

fn dump_value1(label:&str, value:&str, flags:&help::Flags) {
    if !value.is_empty() {
        println!("  {} : {}", color_utils::label(&label.to_string(), flags), value);
    }
}

fn write_cp_entry(parts: Vec<&str>, flags:&help::Flags) {
    if flags.show_unicode_data_na {
        dump_value1("Character Name (na)            ", parts[1], flags);
    }
    if flags.show_unicode_data_gc {
        dump_value2("General Category (gc)          ", parts[2], gc_to_string, flags);
    }
    if flags.show_unicode_data_ccc {
        dump_value2("Canonical Combining Class (ccc)", parts[3], ccc_to_string, flags);
    }
    if flags.show_unicode_data_bc {
        dump_value2("Bidirectional Class (bc)       ", parts[4], bc_to_string, flags);
    }
    if flags.show_unicode_data_dm {
        dump_value1("Decomposition Mapping (dm)     ", parts[5], flags);
    }
    if flags.show_unicode_data_ddv {
        dump_value1("Decimal Digit Value (ddv)      ", parts[6], flags);
    }
    if flags.show_unicode_data_dv {
        dump_value1("Digit Value (dv)               ", parts[7], flags);
    }
    if flags.show_unicode_data_nv {
        dump_value1("Numeric Value (nv)             ", parts[8], flags);
    }
    if flags.show_unicode_data_mir {
        dump_value2("Mirrored (mir)                 ", parts[9], bool_to_string, flags);
    }
    if flags.show_unicode_data_na1 {
        dump_value1("Unicode 1.0 Name (na1)         ", parts[10], flags);
    }
    if flags.show_unicode_data_iso {
        dump_value1("10646 Comment Field (iso)      ", parts[11], flags);
    }
    if flags.show_unicode_data_suc {
        dump_value1("Simple Uppercase Mapping (suc) ", parts[12], flags);
    }
    if flags.show_unicode_data_slc {
        dump_value1("Simple Lowercase Mapping (slc) ", parts[13], flags);
    }
    if flags.show_unicode_data_stc {
        dump_value1("Simple Titlecase Mapping (stc) ", parts[14], flags);
    }
}

pub fn get_unicode_data(code_point:u32, flags:&help::Flags) {
    let cp_str = format!("{:04X}", code_point);
    let data_path = file_utils::get_data_path(&FILE_UNICODE_DATA.to_string());
    let lines = file_utils::read_lines(&data_path);
    let mut start_cp:u32 = BAD_CODE_POINT;
    for line in lines {
        let line_content = line.unwrap();
        if line_content.starts_with(&cp_str) {
            let parts: Vec<&str> = line_content.split(';').collect();
            if parts[0].eq(&cp_str) {
                write_cp_entry(parts, flags);
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
                    write_cp_entry(parts, flags);
                    return;
                }
            }
            start_cp = BAD_CODE_POINT;
        }
    }
}
