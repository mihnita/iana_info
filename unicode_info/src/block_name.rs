use crate::color_utils;
use crate::file_utils;
use crate::help;
use regex::Regex;

const FILE_BLOCKS:&str = "Blocks.txt";

pub fn get_block_name(code_point:u32, flags:&help::Flags) {
    let re_block_info:Regex = Regex::new(r"^([0-9A-F]{4,6})\.\.([0-9A-F]{4,6}); (.+)$").unwrap();
    let data_path = file_utils::get_data_path(&FILE_BLOCKS.to_string());
    let lines = file_utils::read_lines(&data_path);
    for line in lines {
        let line_content = line.unwrap();
        for cap in re_block_info.captures_iter(line_content.as_str()) {
            let start = u32::from_str_radix(&cap[1], 16).unwrap();
            let end = u32::from_str_radix(&cap[2], 16).unwrap();
            if code_point >= start && code_point <= end {
                println!("  {} {}", color_utils::label(&"Unicode Block:".to_string(), flags), &cap[3]);
                continue;
            }
        }
    }
}
