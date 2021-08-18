pub mod messages;
pub mod paint;

pub fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}