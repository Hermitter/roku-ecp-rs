use async_std::task;
use roku_ecp::Device;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    task::block_on(async {
        let ip = std::env::var("ROKU_DEVICE_IP").expect("Could not find $ROKU_DEVICE_IP");
        let roku = Device::new(ip).unwrap();

        // create an image of the Netflix icon
        let icon = roku.icon("12").await.unwrap().bytes;
        let mut file = File::create("netflix_icon.png").unwrap();
        file.write_all(&icon).unwrap();

        // info on Roku device and media player state
        println!("{:#?}", roku.device_info().await.unwrap());
        println!("{:?}", roku.media_player().await.unwrap());
    });
}
