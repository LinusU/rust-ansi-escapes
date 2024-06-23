#![no_std]

use core::fmt;

macro_rules! escape_code {
    ($doc:expr, $name:ident, $value:expr) => {
        #[doc = $doc]
        pub struct $name;

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, $value)
            }
        }
    };
}

/// Set the absolute position of the cursor. x=0 y=0 is the top left of the screen.
pub enum CursorTo {
    TopLeft,
    AbsoluteX(u16),
    AbsoluteXY(u16, u16),
}

impl fmt::Display for CursorTo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CursorTo::TopLeft => write!(f, "\x1B[{};{}H", 1, 1),
            CursorTo::AbsoluteX(x) => write!(f, "\x1B[{}G", x + 1),
            CursorTo::AbsoluteXY(x, y) => write!(f, "\x1B[{};{}H", y + 1, x + 1),
        }
    }
}

/// Set the position of the cursor relative to its current position.
pub enum CursorMove {
    X(i16),
    XY(i16, i16),
    Y(i16),
}

impl fmt::Display for CursorMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CursorMove::X(x) if x > 0 => write!(f, "\x1B[{}C", x),
            CursorMove::X(x) if x < 0 => write!(f, "\x1B[{}D", -x),
            CursorMove::X(_) => fmt::Result::Ok(()),

            CursorMove::XY(x, y) => {
                CursorMove::X(x).fmt(f)?;
                CursorMove::Y(y).fmt(f)?;
                fmt::Result::Ok(())
            }

            CursorMove::Y(y) if y > 0 => write!(f, "\x1B[{}B", y),
            CursorMove::Y(y) if y < 0 => write!(f, "\x1B[{}A", -y),
            CursorMove::Y(_) => fmt::Result::Ok(()),
        }
    }
}

/// Move cursor up a specific amount of rows.
pub struct CursorUp(pub u16);

impl fmt::Display for CursorUp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}A", self.0)
    }
}

/// Move cursor down a specific amount of rows.
pub struct CursorDown(pub u16);

impl fmt::Display for CursorDown {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}B", self.0)
    }
}

/// Move cursor forward a specific amount of rows.
pub struct CursorForward(pub u16);

impl fmt::Display for CursorForward {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}C", self.0)
    }
}

/// Move cursor backward a specific amount of rows.
pub struct CursorBackward(pub u16);

impl fmt::Display for CursorBackward {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1B[{}D", self.0)
    }
}

escape_code!("Move cursor to the left side.", CursorLeft, "\x1B[1000D");
escape_code!("Save cursor position.", CursorSavePosition, "\x1B[s");
escape_code!("Restore saved cursor position.", CursorRestorePosition, "\x1B[u");
escape_code!("Get cursor position.", CursorGetPosition, "\x1B[6n");
escape_code!("Move cursor to the next line.", CursorNextLine, "\x1B[E");
escape_code!("Move cursor to the previous line.", CursorPrevLine, "\x1B[F");
escape_code!("Hide cursor.", CursorHide, "\x1B[?25l");
escape_code!("Show cursor.", CursorShow, "\x1B[?25h");

/// Erase from the current cursor position up the specified amount of rows.
pub struct EraseLines(pub u16);

impl fmt::Display for EraseLines {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for idx in 0..self.0 {
            if idx > 0 {
                write!(f, "{}", CursorUp(1))?;
            }

            write!(f, "{}", CursorLeft)?;
            write!(f, "{}", EraseEndLine)?;
        }

        fmt::Result::Ok(())
    }
}

escape_code!("Erase from the current cursor position to the end of the current line.", EraseEndLine, "\x1B[K");
escape_code!("Erase from the current cursor position to the start of the current line.", EraseStartLine, "\x1B[1K");
escape_code!("Erase the entire current line.", EraseLine, "\x1B[2K");

escape_code!("Erase the screen from the current line down to the bottom of the screen.", EraseDown, "\x1B[J");
escape_code!("Erase the screen from the current line up to the top of the screen.", EraseUp, "\x1B[1J");
escape_code!("Erase the screen and move the cursor the top left position.", EraseScreen, "\x1B[2J");
escape_code!("Scroll display up one line.", ScrollUp, "\x1B[S");
escape_code!("Scroll display down one line.", ScrollDown, "\x1B[T");

escape_code!("Clear the terminal screen.", ClearScreen, "\u{001b}c");
escape_code!("Enter the [alternative screen](https://terminalguide.namepad.de/mode/p47/).", EnterAlternativeScreen, "\x1B[?1049h");
escape_code!("Exit the [alternative screen](https://terminalguide.namepad.de/mode/p47/).", ExitAlternativeScreen, "\x1B[?1049l");
escape_code!("Output a beeping sound.", Beep, "\u{0007}");

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use std::{io::Write, string::String, vec::Vec};

    macro_rules! assert_escape_output {
        ($name:ident, $code:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let mut buf = Vec::new();
                write!(buf, "{}", $code).unwrap();

                let result = String::from_utf8(buf).unwrap();
                assert_eq!(result, $expected);
            }
        };
    }

    assert_escape_output!(cursor_up_1, super::CursorUp(1), "\x1B[1A");
    assert_escape_output!(cursor_up_23, super::CursorUp(23), "\x1B[23A");

    assert_escape_output!(cursor_down_1, super::CursorDown(1), "\x1B[1B");
    assert_escape_output!(cursor_down_23, super::CursorDown(23), "\x1B[23B");

    assert_escape_output!(cursor_forward_1, super::CursorForward(1), "\x1B[1C");
    assert_escape_output!(cursor_forward_23, super::CursorForward(23), "\x1B[23C");

    assert_escape_output!(cursor_backward_1, super::CursorBackward(1), "\x1B[1D");
    assert_escape_output!(cursor_backward_23, super::CursorBackward(23), "\x1B[23D");

    assert_escape_output!(cursor_left, super::CursorLeft, "\x1B[1000D");
    assert_escape_output!(cursor_save_position, super::CursorSavePosition, "\x1B[s");
    assert_escape_output!(cursor_restore_position, super::CursorRestorePosition, "\x1B[u");
    assert_escape_output!(cursor_get_position, super::CursorGetPosition, "\x1B[6n");
    assert_escape_output!(cursor_next_line, super::CursorNextLine, "\x1B[E");
    assert_escape_output!(cursor_prev_line, super::CursorPrevLine, "\x1B[F");
    assert_escape_output!(cursor_hide, super::CursorHide, "\x1B[?25l");
    assert_escape_output!(cursor_show, super::CursorShow, "\x1B[?25h");

    assert_escape_output!(erase_lines_1, super::EraseLines(1), "\x1B[1000D\x1B[K");
    assert_escape_output!(erase_lines_2, super::EraseLines(2), "\x1B[1000D\x1B[K\x1B[1A\x1B[1000D\x1B[K");
    assert_escape_output!(erase_lines_3, super::EraseLines(3), "\x1B[1000D\x1B[K\x1B[1A\x1B[1000D\x1B[K\x1B[1A\x1B[1000D\x1B[K");
}
