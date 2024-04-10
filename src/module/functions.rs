
pub enum Mode {
    Default,
    Windows,
    None
}

pub fn set_mode(args: &Vec<String>) -> Mode {
    if args[1] == "windows" {
        return Mode::Windows;
    }
    return Mode::None;
}