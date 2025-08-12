//! Used to add colors to text when it prints to the terminal.

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BLACK: &str = "\x1b[30m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[33m";
const ANSI_BLUE: &str = "\x1b[34m";
const ANSI_PURPLE: &str = "\x1b[35m";
const ANSI_CYAN: &str = "\x1b[36m";
const ANSI_WHITE: &str = "\x1b[37m";

/// Make the text black.
pub fn black(text: &str) -> String {
    "".to_owned() + ANSI_BLACK + text + ANSI_RESET
}

/// Make the text red.
pub fn red(text: &str) -> String {
    "".to_owned() + ANSI_RED + text + ANSI_RESET
}

/// Make the text green.
pub fn green(text: &str) -> String {
    "".to_owned() + ANSI_GREEN + text + ANSI_RESET
}

/// Make the text yellow.
pub fn yellow(text: &str) -> String {
    "".to_owned() + ANSI_YELLOW + text + ANSI_RESET
}

/// Make the text blue.
pub fn blue(text: &str) -> String {
    "".to_owned() + ANSI_BLUE + text + ANSI_RESET
}

/// Make the text purple.
pub fn purple(text: &str) -> String {
    "".to_owned() + ANSI_PURPLE + text + ANSI_RESET
}

/// Make the text cyan.
pub fn cyan(text: &str) -> String {
    "".to_owned() + ANSI_CYAN + text + ANSI_RESET
}

/// Make the text white.
pub fn white(text: &str) -> String {
    "".to_owned() + ANSI_WHITE + text + ANSI_RESET
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_yellow() {
        let text: &str = "test";
        assert_eq!("\u{1b}[33mtest\u{1b}[0m", yellow(text));
    }
}
