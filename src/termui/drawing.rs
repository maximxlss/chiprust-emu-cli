use std::hint::unreachable_unchecked;

use crossterm::{
    cursor::MoveTo,
    queue,
    style::{Color, ContentStyle, Print}
};

// delimiters
const FRAME_CORNERS: (&str, &str, &str, &str) = ("┌", "┐", "└", "┘");
const FRAME_CONNECTORS: (&str, &str, &str, &str, &str, &str, &str, &str) =
    ("╞", "╡", "╠", "╣", "╥", "╨", "╦", "╩");
const FRAME_VERTICAL: &str = "│";
const FRAME_HORIZONTAL: &str = "─";
const DOUBLE_FRAME_VERTICAL: &str = "║";
const DOUBLE_FRAME_HORIZONTAL: &str = "═";

// colors
pub const TERMINAL_STYLE: (Color, Color) = (Color::White, Color::DarkGrey);

const SCREEN_STYLE: ContentStyle = ContentStyle {
    foreground_color: Some(Color::White),
    background_color: Some(Color::DarkGrey),
    attributes: unsafe { std::mem::transmute(0) },
};
const BORDER_STYLE: ContentStyle = ContentStyle {
    foreground_color: Some(Color::DarkGrey),
    background_color: Some(Color::White),
    attributes: unsafe { std::mem::transmute(0) },
};
const MEMORY_DEBUG_STYLE: ContentStyle = ContentStyle {
    foreground_color: Some(Color::DarkGrey),
    background_color: Some(Color::Grey),
    attributes: unsafe { std::mem::transmute(0) },
};
const MEMORY_CURRENT_STYLE: ContentStyle = ContentStyle {
    foreground_color: Some(Color::Black),
    background_color: Some(Color::White),
    attributes: unsafe { std::mem::transmute(0) },
};
const REGISTER_DEBUG_STYLE: ContentStyle = ContentStyle {
    foreground_color: Some(Color::White),
    background_color: Some(Color::DarkGrey),
    attributes: unsafe { std::mem::transmute(0) },
};

pub fn get_screen(chip: &mut super::Chip8) -> Vec<String> {
    let mut result = Vec::with_capacity(32);
    for y in 0..32 {
        let mut row = String::new();
        for x in 0..128 {
            match chip.display.read_px(x, y * 2) {
                true => match chip.display.read_px(x, y * 2 + 1) {
                    true => row.push('█'),
                    false => row.push('▀'),
                },
                false => match chip.display.read_px(x, y * 2 + 1) {
                    true => row.push('▄'),
                    false => row.push(' '),
                },
            }
        }
        result.push(row);
    }
    result
}

pub fn get_visual_double_byte(b: u16) -> String {
    let mut result = String::new();
    for i in 0..8 {
        let upper = (b >> (7-i)) & 1;
        let lower = (b >> (15-i)) & 1;
        match upper{
            1 => match lower {
                1 => result.push('█'),
                0 => result.push('▀'),
                _ => unsafe {unreachable_unchecked()}
            },
            1 => match lower {
                1 => result.push('▄'),
                0 => result.push(' '),
                _ => unsafe {unreachable_unchecked()}
            },
            _ => unsafe {unreachable_unchecked()}
        }
    }
    result
}

pub fn draw_horizontal_delimiter(
    stdout: &mut std::io::Stdout,
    term_size: (u16, u16),
    x: u16,
    y: u16,
    length: u16,
) {
    let start = if x == 0 {
        FRAME_CONNECTORS.0
    } else {
        FRAME_CONNECTORS.2
    };
    let end = if length == term_size.0 {
        FRAME_CONNECTORS.1
    } else {
        FRAME_CONNECTORS.3
    };
    queue!(
        stdout,
        MoveTo(x, y),
        Print(BORDER_STYLE.apply(start)),
        Print(BORDER_STYLE.apply(DOUBLE_FRAME_HORIZONTAL.repeat(length as usize - 2))),
        Print(BORDER_STYLE.apply(end))
    )
    .expect("Error working with terminal");
}

pub fn draw_vertical_delimiter(
    stdout: &mut std::io::Stdout,
    term_size: (u16, u16),
    x: u16,
    y: u16,
    length: u16,
) {
    let start = if y == 0 {
        FRAME_CONNECTORS.4
    } else {
        FRAME_CONNECTORS.6
    };
    let end = if length == term_size.1 {
        FRAME_CONNECTORS.5
    } else {
        FRAME_CONNECTORS.7
    };
    queue!(stdout, MoveTo(x, y), Print(BORDER_STYLE.apply(start)))
        .expect("Error working with terminal");
    for y in (y + 1)..(y + length as u16 - 1) {
        queue!(
            stdout,
            MoveTo(x, y),
            Print(BORDER_STYLE.apply(DOUBLE_FRAME_VERTICAL))
        )
        .expect("Error working with terminal");
    }
    queue!(
        stdout,
        MoveTo(x, y + length as u16),
        Print(BORDER_STYLE.apply(end))
    )
    .expect("Error working with terminal");
}

pub fn draw_main_frame(term_size: (u16, u16), stdout: &mut std::io::Stdout) {
    // Upper side
    queue!(
        stdout,
        MoveTo(0, 0),
        Print(BORDER_STYLE.apply(FRAME_CORNERS.0)),
        Print(BORDER_STYLE.apply(FRAME_HORIZONTAL.repeat(term_size.0 as usize - 2))),
        Print(BORDER_STYLE.apply(FRAME_CORNERS.1))
    )
    .expect("Error working with terminal");
    // Right and left
    for i in 1..term_size.1 - 1 {
        queue!(
            stdout,
            MoveTo(0, i),
            Print(BORDER_STYLE.apply(FRAME_VERTICAL)),
            MoveTo(term_size.0 - 1, i),
            Print(BORDER_STYLE.apply(FRAME_VERTICAL))
        )
        .expect("Error working with terminal");
    }
    // Bottom side
    queue!(
        stdout,
        MoveTo(0, term_size.1 - 1),
        Print(BORDER_STYLE.apply(FRAME_CORNERS.2)),
        Print(BORDER_STYLE.apply((FRAME_HORIZONTAL).repeat(term_size.0 as usize - 2))),
        Print(BORDER_STYLE.apply(FRAME_CORNERS.3))
    )
    .expect("Error working with terminal");
}

pub fn draw_frame(term_size: (u16, u16), stdout: &mut std::io::Stdout) {
    // # Draw main frame
    draw_main_frame(term_size, stdout);
    // # Draw memory block delimiter
    draw_vertical_delimiter(stdout, term_size, 129, 0, term_size.1);
    // # Draw register block delimiter
    draw_horizontal_delimiter(stdout, term_size, 0, 33, 130)
}

pub fn draw_screen(stdout: &mut std::io::Stdout, chip: &mut super::Chip8) {
    for (i, row) in get_screen(chip).iter().enumerate() {
        queue!(
            stdout,
            MoveTo(1, i as u16 + 1),
            Print(SCREEN_STYLE.apply(row))
        )
        .expect("Error working with terminal");
    }
}

pub fn draw_memory(term_size: (u16, u16), stdout: &mut std::io::Stdout, chip: &mut super::Chip8) {
    let number_of_entries = term_size.1 - 2;
    let current_pos = number_of_entries / 2;
    let starting_with = chip.get_pc() + current_pos as usize - number_of_entries as usize;
    for i in 1..=number_of_entries as usize {
        queue!(stdout, MoveTo(130, i as u16)).expect("Error working with terminal");
        if i == current_pos as usize {
            queue!(
                stdout,
                Print(MEMORY_CURRENT_STYLE.apply(format!(
                    " ${:04x?}: {:04x?} ",
                    starting_with + i,
                    chip.get_memory(starting_with + i)
                )))
            )
        } else {
            queue!(
                stdout,
                Print(MEMORY_DEBUG_STYLE.apply(format!(
                    " {:04x?}: {:04x?} ",
                    starting_with + i,
                    chip.get_memory(starting_with + i)
                )))
            )
        }
        .expect("Error working with terminal");
    }
}

pub fn draw_regs(term_size: (u16, u16), stdout: &mut std::io::Stdout, chip: &mut super::Chip8) {
    let available_lines = term_size.1 - 34;
    let spacing = (available_lines - 2) / 3;
    let middle_space = (available_lines - 2) - spacing * 2;
    let regs = chip.get_regs();
    let mut p = 0;
    p += spacing;
    queue!(stdout, MoveTo(1, 34 + p)).expect("Error working with terminal");
    for (i, reg) in regs.iter().enumerate().take(9) {
        queue!(
            stdout,
            Print(REGISTER_DEBUG_STYLE.apply(format!("  V{:x?}= {:02x?}", i, reg)))
        )
        .expect("Error working with terminal");
    }
    p += middle_space;
    queue!(stdout, MoveTo(1, 34 + p)).expect("Error working with terminal");
    for (i, reg) in regs.iter().enumerate().skip(9) {
        queue!(
            stdout,
            Print(REGISTER_DEBUG_STYLE.apply(format!("  V{:x?}= {:02x?}", i, reg)))
        )
        .expect("Error working with terminal");
    }
    queue!(
        stdout,
        Print(REGISTER_DEBUG_STYLE.apply(format!("  DT= {:02x?}", chip.get_delay_timer()))),
        Print(REGISTER_DEBUG_STYLE.apply(format!("  ST= {:02x?}", chip.get_sound_timer()))),
        Print(REGISTER_DEBUG_STYLE.apply(format!("  I= {:04x?}", chip.get_i())))
    )
    .expect("Error working with terminal");
}
