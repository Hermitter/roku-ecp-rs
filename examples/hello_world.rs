use async_std::task;
use roku_ecp;

fn main() {
    task::block_on(async {
        let device = roku_ecp::Device::new("192.168.1.138");

        device.ping().await.unwrap();

        let device_info = device.device_info().await;
        println!("{:#?}", device_info);
    });
}
