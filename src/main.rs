mod config;
mod termui;
mod input;
mod draw_thread;

use chiprust_emu::Chip8;
use config::Config;
use rodio::Sink;
use std::{thread, sync::{Arc, Mutex}};
use spin_sleep::LoopHelper;

static mut CYCLE_RATE: f64 = 0.;
static mut DRAW_RATE: f64 = 0.;

pub fn cpu_thread(chip: Arc<Mutex<Chip8>>, cpu_freq: u32) {
    {
        let mut chip = chip.lock().unwrap();
        chip.set_handlers(&input::key_wait_handler, &input::key_state_handler);
    }

    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5) 
        .build_with_target_rate(cpu_freq);

    loop {
        loop_helper.loop_start();
        {
            let mut chip = chip.lock().unwrap();
            chip.cpu_tick().unwrap();
        };
        if let Some(fps) = loop_helper.report_rate() {
            unsafe {CYCLE_RATE = fps}
        }
        loop_helper.loop_sleep()
    }
}

fn timers_thread(chip: Arc<Mutex<Chip8>>, timers_freq: u32, sink: Sink) {
    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5) 
        .build_with_target_rate(timers_freq);
    loop {
        loop_helper.loop_start();
        {
            let mut chip = chip.lock().unwrap();
            chip.timers_tick();
            if chip.is_sound_playing() {
                sink.play()
            } else {
                sink.pause()
            }
        }
        loop_helper.loop_sleep()
    }
}

fn run() {
    // load args configuration
    let config = match Config::load_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    // create an emulator instance and load rom from the config
    let mut chip = Chip8::new
        ::<&'static (dyn Fn() -> u8 + Send + Sync + 'static),
        &'static (dyn Fn(u8) -> bool + Send + Sync + 'static)>
        (&|| 0, &|_| false);

    chip.load(0x200, &config.program, None);

    // wrap the instance into an arc mutex
    let chip = Arc::new(Mutex::new(chip));

    // clone the intance and needed constant values and start the cpu thread
    let chip_clone = chip.clone();
    let cpu_freq = config.cpu_freq;
    thread::spawn(move || cpu_thread(chip_clone, cpu_freq));
    // clone the intance and needed constant values and start the timers thread
    let chip_clone = chip.clone();
    let timers_freq = config.timers_freq;
    let sink = config.sink;
    thread::spawn(move || timers_thread(chip_clone, timers_freq, sink));
    // clone the needed constant values and start the draw thread
    let draw_freq = config.draw_freq;
    let handle = thread::spawn(move || draw_thread::draw_thread(chip, draw_freq));

    // keep running until the draw thread exits
    handle.join().unwrap();
}

fn main() {
    run();
}
