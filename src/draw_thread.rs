
use chiprust_emu::Chip8;
use std::sync::{Arc, Mutex};
use spin_sleep::LoopHelper;

use crate::{CYCLE_RATE, DRAW_RATE, termui::TermUI};

pub fn draw_thread(chip: Arc<Mutex<Chip8>>, draw_freq: u32) {
    let mut loop_helper = LoopHelper::builder()
        .report_interval_s(0.5) 
        .build_with_target_rate(draw_freq);
    let mut termui = TermUI::new();
    loop {
        loop_helper.loop_start();
        if let Some(fps) = loop_helper.report_rate() {
            unsafe {DRAW_RATE = fps}
        }
        let (chip_state, display) = {
            let mut chip = chip.lock().unwrap();
            (chip.to_state(), if chip.display.dirty() {Some(*chip.display.read())} else {None})
        };
        termui.draw(format!("{: >5.1} cycles per second; {: >5.1} frames per second drawn", 
                        unsafe{CYCLE_RATE},
                        unsafe{DRAW_RATE}
                    ).as_str(), chip_state, display);
        loop_helper.loop_sleep()
    }
}
