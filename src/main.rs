mod config;
mod termui;

use chiprust_emu::Chip8;
use config::Config;
use std::time::{Duration, Instant};

fn run() {
    let config = match Config::load_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let mut chip = Chip8::new(&|| 0, &|_| false);

    chip.load(0x200, &config.program, None);

    let cpu_tick = if config.cpu_freq != 0 {
        Duration::from_micros(1_000_000 / config.cpu_freq as u64)
    } else {
        Duration::from_micros(0)
    };
    let timers_tick = if config.cpu_freq != 0 {
        Duration::from_micros(1_000_000 / config.cpu_freq as u64)
    } else {
        Duration::from_micros(0)
    };

    let mut termui = termui::TermUI::new();

    let mut last_cpu_tick = Instant::now();
    let mut last_timers_tick = last_cpu_tick;

    loop {
        let cpu_elapsed = last_cpu_tick.elapsed();
        if cpu_elapsed > cpu_tick {
            match chip.cpu_tick() {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            }
            termui.draw(&mut chip);
            last_cpu_tick = Instant::now()
        }
        if last_timers_tick.elapsed() > timers_tick {
            chip.timers_tick();
            if chip.is_sound_playing() {
                config.sink.play()
            } else {
                config.sink.pause()
            }
            last_timers_tick = Instant::now()
        }
    }
}

fn main() {
    run();
}
