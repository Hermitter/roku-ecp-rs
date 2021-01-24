use async_std::task;
use roku_ecp::{Device, Key};
use std::thread;
use std::time::Duration;

fn main() {
    task::block_on(async {
        let ip = std::env::var("ROKU_DEVICE_IP").expect("Could not find $ROKU_DEVICE_IP");
        let roku = Device::new(ip).unwrap();

        let key_combo = vec![
            Key::Home,
            Key::Right,
            Key::Right,
            Key::Right,
            Key::Down,
            Key::Left,
            Key::Left,
            Key::Up,
            Key::Left,
        ];

        for key in key_combo {
            println!("Pressing -> {:?}", key);
            roku.key_press(key).await.unwrap();
            thread::sleep(Duration::from_millis(500))
        }
    });
}
