use async_std::task;
use roku_ecp::{Device, SearchRequest, SearchType};

fn main() {
    task::block_on(async {
        let ip = std::env::var("ROKU_DEVICE_IP").expect("Could not find $ROKU_DEVICE_IP");
        let roku = Device::new(ip).unwrap();

        // get a list of installed apps
        let apps = roku.apps().await.unwrap();
        let apps: Vec<&str> = apps.iter().map(|a| String::as_str(&a.name)).collect();
        println!("Installed Apps: {:#?}", apps);

        // search for keyword with priority given amongst installed apps
        let search = SearchRequest::new("Solar Opposites")
            .providers(&apps)
            .search_type(SearchType::TvShow)
            .season(1)
            .match_any()
            .launch();

        roku.search(search).await.unwrap();
    });
}
