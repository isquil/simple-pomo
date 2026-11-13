use std::io::{self, Read, Write};

use crate::terminal::TerminalRawMode;

pub fn menu(options: &[&str]) -> io::Result<usize> {
    let _raw = TerminalRawMode::enable()?;
    let mut selected = 0;

    print!("\x1B[?25l");

    loop {
        for (i, op) in options.iter().enumerate() {
            if i == selected {
                println!("\r\x1B[7m > {} \x1B[0m", op);
            } else {
                println!("\r   {} ", op);
            }
        }

        io::stdout().flush()?;

        let mut buffer = [0u8; 1];
        let _ = io::stdin().read_exact(&mut buffer);

        match buffer[0] {
            b'\n' | b'\r' => break,
            b'w' | b'k' => {
                selected = selected.saturating_sub(1);
            }
            b's' | b'j' => {
                selected = selected.saturating_add(1).min(options.len() - 1);
            }
            _ => {}
        }

        print!("\x1B[{}A", options.len());
    }

    Ok(selected)
}
