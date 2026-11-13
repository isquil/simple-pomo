use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use self::screen::menu;

mod screen;
mod terminal;

fn main() -> io::Result<()> {
    let options = ["Pomo (25m)", "Break(5m)", "Quit"];

    loop {
        print!("\x1B[2J\x1B[H");

        let choice = menu(&options)?;

        match choice {
            0 => run_timer(25 * 60)?,
            1 => run_timer(5 * 60)?,
            2 => break,
            _ => continue,
        };
    }

    Ok(())
}

fn run_timer(total_secs: u64) -> io::Result<()> {
    let total_duration = Duration::from_secs(total_secs);
    let start = Instant::now();

    print!("\x1B[2J\x1B[H");
    print!("\x1B[?25l");

    while start.elapsed() < total_duration {
        let restante = total_duration.saturating_sub(start.elapsed());

        let mins = restante.as_secs() / 60;
        let secs = restante.as_secs() % 60;

        print!("{:02}:{:02}", mins, secs);
        let _ = io::stdout().flush();

        thread::sleep(Duration::from_millis(200));
        print!("\x1B[2J\x1B[H");
    }

    Ok(())
}
