use download_vimeo::network::network::{NetworkStuff};

#[tokio::main]
async fn main() {
    let network_stuff = NetworkStuff::origin("http://ipconfig.io/");
    _ = network_stuff.perform().await;
}