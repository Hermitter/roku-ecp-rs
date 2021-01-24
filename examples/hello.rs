use async_std::task;

use roku_ecp::{Device, Key, SearchRequest, SearchType};

fn main() {
    task::block_on(async {
        let roku = Device::new("192.168.1.138").unwrap(); // remember to change the IP.

        // Print information on the Roku device
        println!("{:?}", roku.device_info().await.unwrap());

        // Press the play button
        roku.key_press(Key::Play).await.unwrap();

        // Searching for something
        let search = SearchRequest::new("The Mandalorian")
            .search_type(SearchType::TvShow)
            .season(2);

        roku.search(search).await.unwrap();
    });
}
