use crate::color_utils;
use crate::file_utils;
use crate::help;

const FILE_NAMES_LIST:&str = "NamesList.txt";

pub fn get_from_namelist(code_point:u32, flags:&help::Flags) {
    let cp_str = format!("{:04X}\t", code_point);
    let data_path = file_utils::get_data_path(&FILE_NAMES_LIST.to_string());
    let lines = file_utils::read_lines(&data_path);
    let mut start_printing = false;
    for line in lines {
        let line_content = line.unwrap();
        if line_content.starts_with(&cp_str) {
            // println!("  ------------------");
            println!("  {}", color_utils::label(&"From NamesList.txt:".to_string(), flags));
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
