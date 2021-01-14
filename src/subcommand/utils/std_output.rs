pub enum PrintColor {
    RED = 31,
    GREEN = 32,
    YELLOW = 33,
    BLUE = 34,
}

pub fn color_print(color: PrintColor, content: &str) -> String {
    let result = format!("\x1b[{}m{}\x1b[m", color as i32, content);
    return result;
}

pub fn print_info(color: PrintColor, content: &str, message: &str) {
    println!("[{}] {}", color_print(color, content), message);
}
