mod drawing;

use chiprust_emu::Chip8State;
use crossterm::{
    cursor::{Hide, MoveTo},
    execute, queue,
    style::{Colorize, Print, SetBackgroundColor, SetForegroundColor, ResetColor},
    terminal::{size as terminal_size, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType}
};
use ctrlc::set_handler as set_ctrlc_handler;
use std::io::{stdout, Write};

const MINIMUM_SIZE: (u16, u16) = (143, 36);

pub fn exit(error_message: &str) {
    execute!(stdout(), LeaveAlternateScreen, ResetColor).expect("Error working with terminal");
    println!("{}", error_message);
    std::process::exit(0)
}

pub struct TermUI {
    term_size: (u16, u16),
    min_fits: bool,
}

impl TermUI {
    pub fn new() -> TermUI {
        // set up the terminal
        execute!(
            stdout(),
            EnterAlternateScreen,
            Hide,
            SetForegroundColor(drawing::TERMINAL_STYLE.0),
            SetBackgroundColor(drawing::TERMINAL_STYLE.1)
        )
        .expect("Error working with terminal");

        // add a ctrl-c handler to reset the terminal on ctrl-c
        set_ctrlc_handler(|| {
            exit("")
        })
        .expect("Error setting up ctrl-c handler");

        // add a panic hook to reset the terminal on panic
        // not sure if it should even exist
        std::panic::set_hook(Box::new(|panic_info| {
            match execute!(stdout(), LeaveAlternateScreen) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("error working with terminal: {:?}", e);
                }
            };
            if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                println!("panic occurred: {:?}", s);
            } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                println!("panic occurred: {:?}", s);
            } else {
                println!("panic occurred");
            }
            if let Some(location) = panic_info.location() {
                println!(
                    "panic occurred in file '{}' at line {}",
                    location.file(),
                    location.line(),
                );
            } else {
                println!("panic occurred but can't get location information...");
            }
        }));

        TermUI {
            term_size: (0, 0),
            min_fits: false,
        }
    }

    pub fn draw(&mut self, label: &str, chip: Chip8State, display: Option<[u128; 64]>) {
        let mut stdout = stdout();
        if self.term_size != terminal_size().unwrap() {
            queue!(stdout, Clear(ClearType::All)).expect("Error working with terminal");
            self.term_size = terminal_size().unwrap();
            self.min_fits = self.term_size >= MINIMUM_SIZE;
            if !self.min_fits {
                queue!(
                    stdout,
                    MoveTo(0, 0),
                    Print("Too small terminal size. Try lowering font size (Try Ctrl+-)".red())
                )
                .expect("Error working with terminal");
                stdout.flush().expect("Error flusing the stdout");
                return;
            }
            drawing::draw_frame(self.term_size, &mut stdout)
        }
        if let Some(d) = display {
            drawing::draw_screen(&mut stdout, &d)
        }
        drawing::draw_label(&mut stdout, label);
        drawing::draw_memory(self.term_size, &mut stdout, &chip);
        drawing::draw_regs(self.term_size, &mut stdout, &chip);
        stdout.flush().expect("Error flusing the stdout");
    }
}
