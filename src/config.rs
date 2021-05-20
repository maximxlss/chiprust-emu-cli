use clap::{App, Arg, ArgMatches};
#[cfg(feature = "sound")]
use rodio::{source::SineWave, OutputStream, Sink};
use std::io;
use std::{fs::File, io::Read};

fn get_matches() -> ArgMatches<'static> {
    App::new("ChipRust Emulator CLI")
                              .version("1.0")
                              .author("Maxim K. <maximxlssoff@gmail.com>")
                              .about("CLI for ChipRust Emulator")
                              .arg(Arg::with_name("tone")
                                   .short("t")
                                   .long("tone")
                                   .value_name("frequency")
                                   .help("Sets a custom audio tone")
                                   .default_value("900")
                                   .takes_value(true))
                              .arg(Arg::with_name("cpu_freq")
                                   .short("c")
                                   .long("cpu")
                                   .value_name("frequency")
                                   .help("Sets a custom cpu frequency. If zero, instructions would be executed ASAP.")
                                   .default_value("60")
                                   .takes_value(true))
                              .arg(Arg::with_name("draw_freq")
                                   .short("d")
                                   .long("draw")
                                   .value_name("frequency")
                                   .help("Sets a custom draw frequency. Recommended to keep equal to the monitor refresh rate.")
                                   .default_value("60")
                                   .takes_value(true))
                              .arg(Arg::with_name("speed")
                                   .short("s")
                                   .long("speed")
                                   .value_name("frequency")
                                   .help("Sets a custom speed (actually timers' tick frequency). If zero, timers would be decremented ASAP.")
                                   .default_value("60")
                                   .takes_value(true))
                              .arg(Arg::with_name("debug")
                                   .short("d")
                                   .long("debug")
                                   .takes_value(false)
                                   .help("Add this flag to print executed instructions"))
                              .arg(Arg::with_name("source")
                                   .help("Sets the rom file to execute")
                                   .required(true)
                                   .index(1))
                              .get_matches()
}

pub struct Config {
    pub draw_freq: u32,
    pub cpu_freq: u32,
    pub timers_freq: u32,
    #[cfg(feature = "sound")]
    pub sink: Option<Sink>,
    pub is_debug: bool,
    pub program: Vec<u8>,
}

impl Config {
    pub fn load_args() -> Result<Config, String> {
        let matches = get_matches();

        #[cfg(feature = "sound")]
        let sound_freq = matches.value_of("tone").unwrap();

        #[cfg(feature = "sound")]
        let sound_freq = match sound_freq.parse::<u32>() {
            Ok(v) => v,
            Err(_) => {
                return Err(format!(
                    "Can't parse {} to an unsigned integer.",
                    sound_freq
                ))
            }
        };

        let draw_freq = matches.value_of("draw_freq").unwrap();

        let draw_freq = match draw_freq.parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Err(format!("Can't parse {} to an unsigned integer.", draw_freq)),
        };

        let cpu_freq = matches.value_of("cpu_freq").unwrap();

        let cpu_freq = match cpu_freq.parse::<u32>() {
            Ok(v) => v,
            Err(_) => return Err(format!("Can't parse {} to an unsigned integer.", cpu_freq)),
        };

        let timers_freq = matches.value_of("speed").unwrap();

        let timers_freq = match timers_freq.parse::<u32>() {
            Ok(v) => v,
            Err(_) => {
                return Err(format!(
                    "Can't parse {} to an unsigned integer.",
                    timers_freq
                ))
            }
        };

        let is_debug = matches.occurrences_of("debug") > 0;

        let source = matches.value_of("source").unwrap();

        let mut f = match File::open(source) {
            io::Result::Ok(f) => f,
            io::Result::Err(e) => return Err(format!("{}", e)),
        };

        let mut buf: Vec<u8> = Vec::with_capacity(3583);

        match f.read_to_end(&mut buf) {
            Ok(v) => {
                if v >= 3583 {
                    return Err(format!("Source file is too big! Got {} bytes, while free memory is only 3583 bytes.", v));
                }
            }
            Err(e) => return Err(format!("{}", e)),
        }

        #[cfg(feature = "sound")]
        let sink = {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.pause();
            let source = SineWave::new(sound_freq);
            sink.append(source);
            Some(sink)
        };

        Ok(Config {
            draw_freq,
            cpu_freq,
            timers_freq,
            #[cfg(feature = "sound")]
            sink,
            is_debug,
            program: buf,
        })
    }
}
