
use device_query::{DeviceState, keymap::Keycode};
use bimap::BiMap;
use std::io::{stdin, Read};
use lazy_static::lazy_static;
use std::str::FromStr;


lazy_static!(
    static ref DEVICE_STATE: DeviceState = DeviceState::new();
);

lazy_static!(
    static ref KEY_MAP: BiMap<&'static str, u8> = {
        let mut key_map = BiMap::new();
        key_map.insert("Key1", 1);
        key_map.insert("Key2", 2);
        key_map.insert("Key3", 3);
        key_map.insert("Key4", 0xC);
        key_map.insert("Q", 4);
        key_map.insert("W", 5);
        key_map.insert("E", 6);
        key_map.insert("R", 0xD);
        key_map.insert("A", 7);
        key_map.insert("S", 8);
        key_map.insert("D", 9);
        key_map.insert("F", 0xE);
        key_map.insert("Z", 0xA);
        key_map.insert("X", 0);
        key_map.insert("C", 0xB);
        key_map.insert("V", 0xF);
        key_map
    };
);

pub fn key_state_handler(key: u8) -> bool {
    let pressed = DEVICE_STATE.query_keymap();
    return pressed.contains(&Keycode::from_str(KEY_MAP.get_by_right(&key).unwrap()).unwrap());
}

pub fn key_wait_handler() -> u8 {
    loop {
        let _ = stdin().read(&mut [0u8]).unwrap();
        let pressed = DEVICE_STATE.query_keymap();
        if let Some(v) = KEY_MAP.get_by_left(pressed[0].to_string().as_str()) {
            return *v
        }
    }
}
