use async_std::task;
use roku_ecp::Device;
use std::thread;
use std::time::Duration;

fn main() {
    task::block_on(async {
        let ip = std::env::var("ROKU_DEVICE_IP").expect("Could not find $ROKU_DEVICE_IP");
        let roku = Device::new(ip).unwrap();

        let apps = roku.apps().await.unwrap();

        // launch 3 already installed apps
        for app in apps.into_iter().take(3) {
            println!("Opening: {}", app.name);
            roku.launch(&app.id, None).await.unwrap();
            thread::sleep(Duration::from_secs(4));
        }
    });
}
