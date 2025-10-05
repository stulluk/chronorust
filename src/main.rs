use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    env,
    fs::File,
    io::{self, stdout, Write},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

struct Chronometer {
    start_time: Option<Instant>,
    lap_times: Vec<String>,
    lap_durations: Vec<Duration>,
    is_running: bool,
    is_paused: bool,
    paused_duration: Duration,
    log_file: Option<File>,
    start_timestamp: SystemTime,
}

impl Chronometer {
    fn new() -> Self {
        Self {
            start_time: None,
            lap_times: Vec::new(),
            lap_durations: Vec::new(),
            is_running: false,
            is_paused: false,
            paused_duration: Duration::new(0, 0),
            log_file: None,
            start_timestamp: SystemTime::now(),
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.is_running = true;
        self.is_paused = false;
        self.paused_duration = Duration::new(0, 0);
        self.start_timestamp = SystemTime::now();
    }

    fn enable_logging(&mut self) -> io::Result<()> {
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).unwrap();
        let timestamp = duration.as_secs();

        let datetime = chrono::DateTime::from_timestamp(timestamp as i64, 0).unwrap();
        let filename = format!(
            "ChronoRust-{}-log.txt",
            datetime.format("%d-%m-%y-%H-%M-%S")
        );

        let file = File::create(&filename)?;
        self.log_file = Some(file);

        // Write initial log entry
        if let Some(ref mut file) = self.log_file {
            writeln!(
                file,
                "ChronoRust Session Started: {}",
                datetime.format("%Y-%m-%d %H:%M:%S")
            )?;
            writeln!(file, "================================================")?;
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.start_time = Some(Instant::now());
        self.lap_times.clear();
        self.lap_durations.clear();
        self.is_running = true;
        self.is_paused = false;
        self.paused_duration = Duration::new(0, 0);
        self.start_timestamp = SystemTime::now();

        // Log reset event
        if let Some(ref mut file) = self.log_file {
            let now = SystemTime::now();
            let datetime = chrono::DateTime::from_timestamp(
                now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                0,
            )
            .unwrap();
            let _ = writeln!(file, "Reset at: {}", datetime.format("%Y-%m-%d %H:%M:%S"));
        }
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
            let lap_time_clone = lap_time.clone();
            self.lap_times.push(lap_time);
            self.lap_durations.push(elapsed);

            // Log lap event
            if let Some(ref mut file) = self.log_file {
                let now = SystemTime::now();
                let datetime = chrono::DateTime::from_timestamp(
                    now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
                    0,
                )
                .unwrap();
                let _ = writeln!(
                    file,
                    "Lap {} at: {} - Time: {}",
                    self.lap_times.len(),
                    datetime.format("%Y-%m-%d %H:%M:%S"),
                    lap_time_clone
                );
            }
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

    fn get_lap_differences(&self) -> Vec<String> {
        let mut differences = Vec::new();

        if self.lap_durations.len() <= 1 {
            return differences;
        }

        for i in 1..self.lap_durations.len() {
            let prev_lap = self.lap_durations[i - 1];
            let current_lap = self.lap_durations[i];
            let diff = current_lap - prev_lap;
            differences.push(self.format_duration(diff));
        }

        differences
    }
}

fn main() -> io::Result<()> {
    // Check for help flag
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!(
            "ChronoRust v{} - High Precision Chronometer",
            env!("CARGO_PKG_VERSION")
        );
        println!();
        println!("A high-precision chronometer for Linux terminal built with Rust.");
        println!("Provides millisecond-level timing accuracy and supports up to 99 hours of measurement.");
        println!();
        println!("USAGE:");
        println!("    chronorust [OPTIONS]");
        println!();
        println!("OPTIONS:");
        println!("    -C, --logging    Enable session logging");
        println!("    -h, --help       Show this help message");
        println!();
        println!("CONTROLS:");
        println!("    L               Record lap time");
        println!("    R               Reset chronometer and restart");
        println!("    S               Pause/Resume chronometer");
        println!("    Q               Quit application");
        println!();
        println!("FEATURES:");
        println!("    • High precision millisecond timing");
        println!("    • Lap time recording with differences");
        println!("    • Pause/Resume functionality");
        println!("    • Optional session logging");
        println!("    • Cross-platform support (Linux, Windows, macOS)");
        println!();
        println!("EXAMPLES:");
        println!("    chronorust              # Start chronometer");
        println!("    chronorust -C           # Start with logging enabled");
        println!();
        println!("For more information, visit: https://github.com/stulluk/chronorust");
        return Ok(());
    }

    // Check for logging flag
    let enable_logging = args.contains(&"-C".to_string());

    if enable_logging {
        println!("Logging enabled. Log file will be created in current directory.");
    }

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut chronometer = Chronometer::new();

    // Enable logging if requested
    if enable_logging {
        chronometer.enable_logging()?;
    }

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
    let title = Paragraph::new(format!(
        "ChronoRust v{} - High Precision Chronometer",
        env!("CARGO_PKG_VERSION")
    ))
    .style(
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    )
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
        .style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title("Time"));
    f.render_widget(time_paragraph, chunks[1]);

    // Lap times with differences
    let mut lap_items: Vec<ListItem> = Vec::new();
    let differences = chronometer.get_lap_differences();

    for (i, lap_time) in chronometer.lap_times.iter().enumerate() {
        let mut lap_text = format!("Lap {}: {}", i + 1, lap_time);

        // Add difference if available
        if i > 0 && i - 1 < differences.len() {
            lap_text.push_str(&format!(" (Δ: {})", differences[i - 1]));
        }

        lap_items.push(ListItem::new(lap_text).style(Style::default().fg(Color::Yellow)));
    }

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
