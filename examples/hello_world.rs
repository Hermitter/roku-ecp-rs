use async_std::task;
use roku_ecp;

fn main() {
    task::block_on(async {
        let roku = roku_ecp::Device::new("192.168.1.138");

        // roku.ping().await.unwrap();

        // let device_info = roku.device_info().await;
        // println!("{:#?}", device_info);

        // let media_player = roku.media_player().await;
        // println!("{:#?}", media_player);

        // let app_icon = roku.app_icon(12).await;
        // println!("{:#?}", app_icon);
    });
}
