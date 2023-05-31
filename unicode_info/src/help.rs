use atty::Stream;
use std::process::exit;

#[derive(Debug)]
pub struct Flags {
	pub show_utf:bool,
	pub show_dec:bool,
	pub show_utf8:bool,
	pub show_u16:bool,
	pub show_u32:bool,
    pub show_unicode_data:bool,
	pub show_block_info:bool,
	pub show_name_info:bool,
    pub show_color:bool,
	pub rest: Vec<String>,
}

impl Flags {
    pub fn new() -> Flags {
        Flags {
			show_utf: true,
			show_dec: true,
			show_utf8: true,
			show_u16: true,
			show_u32: true,
			show_unicode_data: true,
			show_block_info: true,
			show_name_info: true,
			show_color: true,
			rest: Vec::new(),
		}
	}
}

pub fn print_help() {
    println!("Usage: unicode_info [<flag> ...] <string_or_hex_code_point> ...");
    println!("");
    println!("Find and show Unicode info (from the standard Unicode data files)");
    println!("");
    println!("  -u      : show numeric values (decimal and UTF forms)");
	println!("    -d      : show decimal");
	println!("    -u8     : show UTF-8");
	println!("    -u16    : show UTF-16");
	println!("    -u32    : show UTF-32");
    println!("  -ud       : show info from Unicode data (from UnicodeData.txt)");
	println!("  -b        : show Unicode block info (from Blocks.txt)");
	println!("  -n        : show name info (from NamesList.txt)");
    println!("--color=always   : force to always use colors");
    println!("--color=never    : force to never use colors");
    println!("where the value can be a substring or exact match if it starts with '='");
    exit(1);
}

pub fn parse_args(args: &Vec<String>) -> Flags {
    let mut result:Flags = Flags::new();

	result.show_color = atty::is(Stream::Stdout);
	for i in 1..args.len() {
		let arg = &args[i];
        match arg.as_str() {
			"-u"             => result.show_utf = true,
			"-d"             => result.show_dec = true,
			"-u8"            => result.show_utf8 = true,
			"-u16"           => result.show_u16 = true,
			"-u32"           => result.show_u32 = true,
			"-ud"            => result.show_unicode_data = true,
			"-b"             => result.show_block_info = true,
			"-n"             => result.show_name_info = true,
			"--color=always" => result.show_color = true,
			"--color=never"  => result.show_color = false,
            "-h" | "--help"  => print_help(),
            _                => result.rest.push(arg.to_string())
        };
    }
    // dbg!(&result);
    return result;
}
