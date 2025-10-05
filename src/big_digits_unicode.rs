// Unicode box drawing characters for better cross-platform support
pub fn get_big_digit_unicode(digit: char) -> Vec<&'static str> {
    match digit {
        '0' => vec![
            "┌─────┐",
            "│  ●  │",
            "│ ● ● │",
            "│  ●  │",
            "└─────┘",
        ],
        '1' => vec![
            "  ┌─┐  ",
            "  │ │  ",
            "  │ │  ",
            "  │ │  ",
            "  └─┘  ",
        ],
        '2' => vec![
            "┌─────┐",
            "    ┌─┘",
            "┌───┘  ",
            "└─────┐",
            "└─────┘",
        ],
        '3' => vec![
            "┌─────┐",
            "    ┌─┘",
            "┌───┘  ",
            "    ┌─┘",
            "└─────┘",
        ],
        '4' => vec![
            "┌─  ┌─┐",
            "│   │ │",
            "└───┴─┘",
            "    │ │",
            "    └─┘",
        ],
        '5' => vec![
            "┌─────┐",
            "│     │",
            "└─────┐",
            "    ┌─┘",
            "└─────┘",
        ],
        '6' => vec![
            "┌─────┐",
            "│     │",
            "├─────┤",
            "│  ●  │",
            "└─────┘",
        ],
        '7' => vec![
            "┌─────┐",
            "    ┌─┘",
            "   ┌─┘ ",
            "  ┌─┘  ",
            " └─┘   ",
        ],
        '8' => vec![
            "┌─────┐",
            "│  ●  │",
            "├─────┤",
            "│  ●  │",
            "└─────┘",
        ],
        '9' => vec![
            "┌─────┐",
            "│  ●  │",
            "├─────┤",
            "    ┌─┘",
            "└─────┘",
        ],
        ':' => vec![
            "       ",
            "   ●   ",
            "       ",
            "   ●   ",
            "       ",
        ],
        '.' => vec![
            "       ",
            "       ",
            "       ",
            "   ●   ",
            "       ",
        ],
        _ => vec![
            "       ",
            "       ",
            "       ",
            "       ",
            "       ",
        ],
    }
}

pub fn format_big_time_unicode(time_str: &str) -> Vec<String> {
    let mut lines = vec![String::new(); 5];
    
    for ch in time_str.chars() {
        let digit_lines = get_big_digit_unicode(ch);
        for (i, line) in digit_lines.iter().enumerate() {
            lines[i].push_str(line);
            lines[i].push(' ');
        }
    }
    
    lines
}
