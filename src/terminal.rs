use std::io;
use std::mem::zeroed;

use libc::{ECHO, ICANON, TCSAFLUSH, tcgetattr, tcsetattr, termios};

pub struct TerminalRawMode {
    original: termios,
}

impl Drop for TerminalRawMode {
    fn drop(&mut self) {
        print!("\x1B[?25h");
        unsafe { tcsetattr(0, TCSAFLUSH, &self.original) };
    }
}

impl TerminalRawMode {
    pub fn enable() -> io::Result<Self> {
        let mut original: termios = unsafe { zeroed() };
        if let -1 = unsafe { tcgetattr(0, &mut original) } {
            return Err(io::Error::last_os_error());
        };

        let mut raw = original;
        raw.c_lflag &= !(ECHO | ICANON);
        if let -1 = unsafe { tcsetattr(0, TCSAFLUSH, &raw) } {
            return Err(io::Error::last_os_error());
        };

        Ok(Self { original })
    }
}
