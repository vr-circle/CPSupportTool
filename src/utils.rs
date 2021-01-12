pub enum PrintColor {
    ERROR = 1,
    SUCCESS = 2,
    INFO = 4,
}

pub fn print_info(color: PrintColor, content: &str, message: &str) {
    println!("[\x1b[{}m{}\x1b[m] {}", 30 + color as i32, content, message);
}
