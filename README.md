# ChronoRust

A high-precision chronometer for Linux terminal built with Rust. This application provides millisecond-level timing accuracy and supports up to 99 hours of measurement.

## Features

- **High Precision**: Millisecond accuracy (3 decimal places)
- **Long Duration Support**: Up to 99 hours of timing
- **Real-time Display**: Live updating chronometer display
- **Lap Times**: Record multiple lap times during a session
- **Terminal UI**: Clean, colorful terminal interface
- **Keyboard Controls**: Simple key-based controls

## Installation

### Prerequisites

- Rust (latest stable version)
- Linux terminal

### Build from Source

```bash
git clone https://github.com/yourusername/chronorust.git
cd chronorust
cargo build --release
```

### Run

```bash
cargo run
```

Or run the release binary:

```bash
./target/release/chronorust
```

## Usage

### Controls

- **T** - Start chronometer or record lap time
- **R** - Reset chronometer and clear all lap times
- **Q** - Quit application

### Display Format

- **Time Format**: `HH:MM:SS.mmm` (hours:minutes:seconds.milliseconds)
- **Short Format**: `MM:SS.mmm` (for times under 1 hour)
- **Lap Times**: Numbered list of recorded lap times

### Example Session

1. Start the application: `cargo run`
2. Press **T** to start the chronometer
3. Press **T** again to record lap times
4. Press **R** to reset and start over
5. Press **Q** to quit

## Technical Details

- **Language**: Rust
- **Dependencies**: 
  - `crossterm` - Cross-platform terminal manipulation
  - `chrono` - Date and time handling
- **Precision**: Millisecond-level timing using `std::time::Instant`
- **Maximum Duration**: 99 hours (3,564,000,000 milliseconds)

## Development

### Project Structure

```
chronorust/
├── Cargo.toml          # Project configuration
├── src/
│   └── main.rs         # Main application code
└── README.md           # This file
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check code
cargo check
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Roadmap

- [ ] Save/load lap times to file
- [ ] Multiple chronometer sessions
- [ ] Sound alerts for lap times
- [ ] Export lap times to CSV
- [ ] Customizable display themes
- [ ] Split times functionality

## Screenshots

```
ChronoRust - High Precision Chronometer
Press 'r' to reset, 't' to lap, 'q' to quit
==================================================
Time: 00:05.234
Lap Times:
Lap 1: 00:02.156
Lap 2: 00:03.078
Controls: R - Reset | T - Lap | Q - Quit
```
