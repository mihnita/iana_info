use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

pub fn read_lines(data_path:&Path) -> io::Lines<BufReader<File>> {
    let file = File::open(data_path).unwrap(); 
    return io::BufReader::new(file).lines(); 
}

pub fn get_data_path(file_name:&String) -> PathBuf {
    let result = env::current_exe().unwrap()
        .parent().unwrap()
        .join("udata")
        .join(file_name);
    // print!("{:?}", &result);
    return result;
}
