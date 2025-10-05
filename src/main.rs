use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    style::{self, Stylize},
    terminal::{self, ClearType},
};
use std::{
    io::{self, stdout, Write},
    time::{Duration, Instant},
};

struct Chronometer {
    start_time: Option<Instant>,
    lap_times: Vec<String>,
    is_running: bool,
}

impl Chronometer {
    fn new() -> Self {
        Self {
            start_time: None,
            lap_times: Vec::new(),
            is_running: false,
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_running = true;
    }

    fn reset(&mut self) {
        self.start_time = None;
        self.lap_times.clear();
        self.is_running = false;
    }

    fn add_lap(&mut self) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            let lap_time = self.format_duration(elapsed);
            self.lap_times.push(lap_time);
        }
    }

    fn get_elapsed(&self) -> Duration {
        self.start_time.map(|start| start.elapsed()).unwrap_or_default()
    }

    fn format_duration(&self, duration: Duration) -> String {
        let total_ms = duration.as_millis();
        let hours = total_ms / 3_600_000;
        let minutes = (total_ms % 3_600_000) / 60_000;
        let seconds = (total_ms % 60_000) / 1_000;
        let milliseconds = total_ms % 1_000;

        if hours > 0 {
            format!("{:02}:{:02}:{:02}.{:03}", hours, minutes, seconds, milliseconds)
        } else {
            format!("{:02}:{:02}.{:03}", minutes, seconds, milliseconds)
        }
    }

    fn display(&self) -> String {
        if self.is_running {
            self.format_duration(self.get_elapsed())
        } else {
            "00:00.000".to_string()
        }
    }
}

fn main() -> io::Result<()> {
    // Terminal'i raw mode'a al
    terminal::enable_raw_mode()?;
    
    let mut stdout = stdout();
    
    // Ekranı temizle ve cursor'u gizle
    execute!(stdout, terminal::Clear(ClearType::All), cursor::Hide)?;

    let mut chronometer = Chronometer::new();
    chronometer.start(); // Otomatik olarak başlat
    let mut running = true;

    // Başlangıç mesajı
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        style::Print("ChronoRust - High Precision Chronometer"),
        cursor::MoveTo(0, 1),
        style::Print("Press 'r' to reset, 't' to lap, 'q' to quit"),
        cursor::MoveTo(0, 2),
        style::Print("=".repeat(50))
    )?;

    while running {
        // Ana chronometer ekranını güncelle
        let elapsed = chronometer.display();
        execute!(
            stdout,
            cursor::MoveTo(0, 4),
            terminal::Clear(ClearType::CurrentLine),
            style::Print(format!("Time: {}", elapsed).bold().green())
        )?;

        // Lap times'ları göster
        let start_row = 6;
        execute!(
            stdout,
            cursor::MoveTo(0, start_row),
            style::Print("Lap Times:")
        )?;

        for (i, lap_time) in chronometer.lap_times.iter().enumerate() {
            execute!(
                stdout,
                cursor::MoveTo(0, start_row + 1 + i as u16),
                terminal::Clear(ClearType::CurrentLine),
                style::Print(format!("Lap {}: {}", i + 1, lap_time))
            )?;
        }

        // Legend'ı göster
        let legend_row = start_row + 1 + chronometer.lap_times.len() as u16 + 2;
        execute!(
            stdout,
            cursor::MoveTo(0, legend_row),
            terminal::Clear(ClearType::CurrentLine),
            style::Print("Controls: "),
            style::Print("R".bold().red()),
            style::Print(" - Reset | "),
            style::Print("T".bold().yellow()),
            style::Print(" - Lap | "),
            style::Print("Q".bold().red()),
            style::Print(" - Quit")
        )?;

        stdout.flush()?;

        // Tuş girişini bekle
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        running = false;
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        chronometer.reset();
                    }
                    KeyCode::Char('t') | KeyCode::Char('T') => {
                        if chronometer.is_running {
                            chronometer.add_lap();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Terminal'i normal mode'a geri döndür
    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    
    println!("\nChronoRust stopped. Goodbye!");
    Ok(())
}
