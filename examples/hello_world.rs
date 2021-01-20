use async_std::task;
use roku_ecp::{Device, Key};

fn main() {
    task::block_on(async {
        let roku = Device::new("192.168.1.138");

        // roku.ping().await.unwrap();

        // let device_info = roku.device_info().await;
        // println!("{:#?}", device_info);

        // let media_player = roku.media_player().await;
        // println!("{:#?}", media_player);

        // let app_icon = roku.icon(12).await;
        // println!("{:#?}", app_icon);

        // let active_app = roku.active_app().await;
        // println!("{:#?}", active_app);

        // let apps = roku.apps().await;
        // println!("{:#?}", apps);

        // roku.key_press(Key::Play).await.unwrap();
        // roku.send_string("Hello @user").await.unwrap();
        // roku.key_press(Key::VolumeDown).await.unwrap();
    });
}
