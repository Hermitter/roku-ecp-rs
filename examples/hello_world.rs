use async_std::task;
use roku_ecp;

fn main() {
    task::block_on(async {
        roku_ecp::test().await;
        println!("Hello World");
    });
}
