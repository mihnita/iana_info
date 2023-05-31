use crate::help;

pub fn label(msg:&String, flags:&help::Flags) -> String {
    if flags.show_color {
        return "\x1b[93m".to_string() + msg + "\x1b[m";
    } else {
        return msg.to_string();
    }
}
