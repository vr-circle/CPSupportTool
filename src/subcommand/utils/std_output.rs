pub enum PrintColor {
    ERROR = 31,
    SUCCESS = 32,
    INFO = 34,
    WARN = 35,
}

pub fn print_info(color: PrintColor, content: &str, message: &str) {
    println!("[\x1b[{}m{}\x1b[m] {}", color as i32, content, message);
}
