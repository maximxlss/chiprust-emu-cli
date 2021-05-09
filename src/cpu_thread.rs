use device_query::{DeviceState, keymap::Keycode};
use bimap::BiMap;
use std::{io::{stdin, Read}, sync::{Arc, Mutex, mpsc::Sender}, time::{Duration, Instant}};
use lazy_static::lazy_static;
use std::str::FromStr;


lazy_static!(
    static ref device_state: DeviceState = DeviceState::new();
);

lazy_static!(
    static ref KEY_MAP: BiMap<&'static str, u8> = {
        let mut key_map = BiMap::new();
        key_map.insert("Key1", 0);
        key_map.insert("Key2", 1);
        key_map.insert("Key3", 2);
        key_map.insert("Key1", 3);
        key_map.insert("Key1", 4);
        key_map.insert("Key1", 5);
        key_map.insert("Key1", 6);
        key_map.insert("Key1", 7);
        key_map.insert("Key1", 8);
        key_map.insert("Key1", 9);
        key_map.insert("Key1", 0xA);
        key_map.insert("Key1", 0xB);
        key_map.insert("Key1", 0xC);
        key_map.insert("Key1", 0xD);
        key_map.insert("Key1", 0xE);
        key_map.insert("Key1", 0xF);
        key_map
    };
);

pub fn key_state_handler(key: u8) -> bool {
    let pressed = device_state.query_keymap();
    return pressed.contains(&Keycode::from_str(KEY_MAP.get_by_right(&key).unwrap()).unwrap());
}

pub fn key_wait_handler() -> u8 {
    loop {
        let _ = stdin().read(&mut [0u8]).unwrap();
        let pressed = device_state.query_keymap();
        if let Some(v) = KEY_MAP.get_by_left(pressed[0].to_string().as_str()) {
            return *v
        }
    }
}
