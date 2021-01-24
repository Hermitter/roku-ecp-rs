use async_std::task;
use roku_ecp::{Device, SearchRequest, SearchType};

fn main() {
    task::block_on(async {
        let ip = std::env::var("ROKU_DEVICE_IP").expect("Could not find $ROKU_DEVICE_IP");
        let roku = Device::new(ip).unwrap();

        let search = SearchRequest::new("Rick and Morty")
            .providers(&["Hulu", "HBO Max", "Netflix"])
            .search_type(SearchType::TvShow)
            .season(2)
            .match_any()
            .launch();

        roku.search(search).await.unwrap();
    });
}
