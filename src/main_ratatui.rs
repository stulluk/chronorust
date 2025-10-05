use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{
    io::{self, stdout},
    time::{Duration, Instant},
};

struct Chronometer {
    start_time: Option<Instant>,
    lap_times: Vec<String>,
    is_running: bool,
    is_paused: bool,
    paused_duration: Duration,
}

impl Chronometer {
    fn new() -> Self {
        Self {
            start_time: None,
            lap_times: Vec::new(),
            is_running: false,
            is_paused: false,
            paused_duration: Duration::new(0, 0),
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_running = true;
        self.is_paused = false;
        self.paused_duration = Duration::new(0, 0);
    }

    fn reset(&mut self) {
        self.start_time = Some(Instant::now());
        self.lap_times.clear();
        self.is_running = true;
        self.is_paused = false;
        self.paused_duration = Duration::new(0, 0);
    }

    fn pause(&mut self) {
        if self.is_running && !self.is_paused {
            self.paused_duration += self.start_time.unwrap().elapsed();
            self.is_paused = true;
        }
    }

    fn resume(&mut self) {
        if self.is_running && self.is_paused {
            self.start_time = Some(Instant::now());
            self.is_paused = false;
        }
    }

    fn add_lap(&mut self) {
        if self.is_running {
            let elapsed = self.get_elapsed();
            let lap_time = self.format_duration(elapsed);
            self.lap_times.push(lap_time);
        }
    }

    fn get_elapsed(&self) -> Duration {
        if self.is_paused {
            self.paused_duration
        } else if let Some(start) = self.start_time {
            self.paused_duration + start.elapsed()
        } else {
            Duration::new(0, 0)
        }
    }

    fn format_duration(&self, duration: Duration) -> String {
        let total_ms = duration.as_millis();
        let hours = total_ms / 3_600_000;
        let minutes = (total_ms % 3_600_000) / 60_000;
        let seconds = (total_ms % 60_000) / 1_000;
        let milliseconds = total_ms % 1_000;

        format!(
            "{:02}:{:02}:{:02}.{:03}",
            hours, minutes, seconds, milliseconds
        )
    }

    fn display(&self) -> String {
        if self.is_running {
            self.format_duration(self.get_elapsed())
        } else {
            "00:00:00.000".to_string()
        }
    }
}

fn main() -> io::Result<()> {
    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut chronometer = Chronometer::new();
    chronometer.start();
    let mut running = true;

    // Main loop
    while running {
        terminal.draw(|f| ui(f, &chronometer))?;

        // Handle input
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        running = false;
                    }
                    KeyCode::Char('r') | KeyCode::Char('R') => {
                        chronometer.reset();
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        if chronometer.is_running {
                            chronometer.add_lap();
                        }
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') => {
                        if chronometer.is_paused {
                            chronometer.resume();
                        } else {
                            chronometer.pause();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    // Restore terminal
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    println!("ChronoRust stopped. Goodbye!");
    Ok(())
}

fn ui(f: &mut Frame, chronometer: &Chronometer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Length(3), // Time display
            Constraint::Min(5),    // Lap times
            Constraint::Length(3), // Controls
        ])
        .split(f.size());

    // Title
    let title = Paragraph::new("ChronoRust - High Precision Chronometer")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Time display
    let time_text = if chronometer.is_paused {
        format!("⏸️  {}", chronometer.display())
    } else {
        format!("⏱️  {}", chronometer.display())
    };
    
    let time_paragraph = Paragraph::new(time_text)
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Time"));
    f.render_widget(time_paragraph, chunks[1]);

    // Lap times
    let lap_items: Vec<ListItem> = chronometer
        .lap_times
        .iter()
        .enumerate()
        .map(|(i, lap_time)| {
            ListItem::new(format!("Lap {}: {}", i + 1, lap_time))
                .style(Style::default().fg(Color::Yellow))
        })
        .collect();

    let lap_list = List::new(lap_items)
        .block(Block::default().borders(Borders::ALL).title("Lap Times"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    f.render_widget(lap_list, chunks[2]);

    // Controls
    let controls_text = "Controls: R - Reset | L - Lap | S - Pause/Resume | Q - Quit";
    let controls_paragraph = Paragraph::new(controls_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(controls_paragraph, chunks[3]);
}
