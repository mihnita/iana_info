use atty::Stream;
use std::process::exit;

#[derive(Debug)]
pub struct Flags {
	pub show_utf:bool,
		pub show_dec:bool,
		pub show_utf8:bool,
		pub show_utf16:bool,
		pub show_utf32:bool,
    pub show_unicode_data:bool,
		pub show_unicode_data_na:bool,
		pub show_unicode_data_gc:bool,
		pub show_unicode_data_ccc:bool,
		pub show_unicode_data_bc:bool,
		pub show_unicode_data_dm:bool,
		pub show_unicode_data_ddv:bool,
		pub show_unicode_data_dv:bool,
		pub show_unicode_data_nv:bool,
		pub show_unicode_data_mir:bool,
		pub show_unicode_data_na1:bool,
		pub show_unicode_data_iso:bool,
		pub show_unicode_data_suc:bool,
		pub show_unicode_data_slc:bool,
		pub show_unicode_data_stc:bool,
	pub show_block_info:bool,
	pub show_name_info:bool,
    pub show_color:bool,
	pub rest: Vec<String>,
}

impl Flags {
    pub fn new(show_default:bool) -> Flags {
        Flags {
			show_utf: show_default,
				show_dec: show_default,
				show_utf8: show_default,
				show_utf16: show_default,
				show_utf32: show_default,
			show_unicode_data: show_default,
				show_unicode_data_na: show_default,
				show_unicode_data_gc: show_default,
				show_unicode_data_ccc: show_default,
				show_unicode_data_bc: show_default,
				show_unicode_data_dm: show_default,
				show_unicode_data_ddv: show_default,
				show_unicode_data_dv: show_default,
				show_unicode_data_nv: show_default,
				show_unicode_data_mir: show_default,
				show_unicode_data_na1: show_default,
				show_unicode_data_iso: show_default,
				show_unicode_data_suc: show_default,
				show_unicode_data_slc: show_default,
				show_unicode_data_stc: show_default,
			show_block_info: show_default,
			show_name_info: show_default,
			show_color: atty::is(Stream::Stdout),
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
    println!("  -ud        : show info from Unicode data (from UnicodeData.txt)");
	println!("    -udna    : show Unicode data - Character Name");
    println!("    -udgc    : show Unicode Data - General Category");
    println!("    -udccc   : show Unicode Data - Canonical Combining Class");
    println!("    -udbc    : show Unicode Data - Bidirectional Class");
    println!("    -uddm    : show Unicode Data - Decomposition Mapping");
    println!("    -udddv   : show Unicode Data - Decimal Digit Value");
    println!("    -uddv    : show Unicode Data - Digit Value");
    println!("    -udnv    : show Unicode Data - Numeric Value");
    println!("    -udmir   : show Unicode Data - Mirrored");
    println!("    -udna1   : show Unicode Data - Unicode 1.0 Name");
    println!("    -udiso   : show Unicode Data - 10646 Comment Field");
    println!("    -udsuc   : show Unicode Data - Simple Uppercase Mapping");
    println!("    -udslc   : show Unicode Data - Simple Lowercase Mapping");
    println!("    -udstc   : show Unicode Data - Simple Titlecase Mapping");
	println!("  -b        : show Unicode block info (from Blocks.txt)");
	println!("  -n        : show name info (from NamesList.txt)");
    println!("--color=always   : force to always use colors");
    println!("--color=never    : force to never use colors");
    println!("where the value can be a substring or exact match if it starts with '='");
    exit(1);
}

pub fn parse_args(args: &Vec<String>) -> Flags {
    let mut result:Flags = Flags::new(false);

	let mut show_all = true;
	let mut show_color = atty::is(Stream::Stdout);
	let mut rest: Vec<String> = Vec::new();

	for i in 1..args.len() {
		let arg = &args[i];
        match arg.as_str() {
			"-u"             => { show_all = false; result.show_utf = true },
				"-d"             => { show_all = false; result.show_dec = true },
				"-u8"            => { show_all = false; result.show_utf8 = true },
				"-u16"           => { show_all = false; result.show_utf16 = true },
				"-u32"           => { show_all = false; result.show_utf32 = true },
			"-ud"            => { show_all = false; result.show_unicode_data = true },
				"-udna"          => { show_all = false; result.show_unicode_data_na  = true },
				"-udgc"          => { show_all = false; result.show_unicode_data_gc  = true },
				"-udccc"         => { show_all = false; result.show_unicode_data_ccc = true },
				"-udbc"          => { show_all = false; result.show_unicode_data_bc  = true },
				"-uddm"          => { show_all = false; result.show_unicode_data_dm  = true },
				"-udddv"         => { show_all = false; result.show_unicode_data_ddv = true },
				"-uddv"          => { show_all = false; result.show_unicode_data_dv  = true },
				"-udnv"          => { show_all = false; result.show_unicode_data_nv  = true },
				"-udmir"         => { show_all = false; result.show_unicode_data_mir = true },
				"-udna1"         => { show_all = false; result.show_unicode_data_na1 = true },
				"-udiso"         => { show_all = false; result.show_unicode_data_iso = true },
				"-udsuc"         => { show_all = false; result.show_unicode_data_suc = true },
				"-udslc"         => { show_all = false; result.show_unicode_data_slc = true },
				"-udstc"         => { show_all = false; result.show_unicode_data_stc = true },
			"-b"             => { show_all = false; result.show_block_info = true },
			"-n"             => { show_all = false; result.show_name_info = true },
			"--color=always" => { show_color = true },
			"--color=never"  => { show_color = false },
            "-h" | "--help"  => print_help(),
            _                => rest.push(arg.to_string())
        };
    }
	if show_all {
		result = Flags::new(true);
	} else {
		if result.show_utf {
			result.show_dec = true;
			result.show_utf8 = true;
			result.show_utf16 = true;
			result.show_utf32 = true;
		} else {
			result.show_utf = result.show_dec || result.show_utf8 || result.show_utf16 || result.show_utf32;
		}

		if result.show_unicode_data {
			result.show_unicode_data_na  = true;
			result.show_unicode_data_gc  = true;
			result.show_unicode_data_ccc = true;
			result.show_unicode_data_bc  = true;
			result.show_unicode_data_dm  = true;
			result.show_unicode_data_ddv = true;
			result.show_unicode_data_dv  = true;
			result.show_unicode_data_nv  = true;
			result.show_unicode_data_mir = true;
			result.show_unicode_data_na1 = true;
			result.show_unicode_data_iso = true;
			result.show_unicode_data_suc = true;
			result.show_unicode_data_slc = true;
			result.show_unicode_data_stc = true;
		} else {
			result.show_unicode_data = result.show_unicode_data_na
				|| result.show_unicode_data_gc || result.show_unicode_data_ccc
				|| result.show_unicode_data_bc || result.show_unicode_data_dm
				|| result.show_unicode_data_ddv || result.show_unicode_data_dv
				|| result.show_unicode_data_nv || result.show_unicode_data_mir
				|| result.show_unicode_data_na1 || result.show_unicode_data_iso
				|| result.show_unicode_data_suc || result.show_unicode_data_slc
				|| result.show_unicode_data_stc;
		}
	}
	result.show_color = show_color;
	result.rest = rest;
    // dbg!(&result);
    return result;
}
