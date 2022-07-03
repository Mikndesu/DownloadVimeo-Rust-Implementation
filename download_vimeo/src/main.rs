use download_vimeo::{network::fetch_metadata};

#[tokio::main]
async fn main() {
    let url = "https://55vod-adaptive.akamaized.net/exp=1656824293~acl=%2F2cb85386-694c-4ba4-a388-07f86114d42e%2F%2A~hmac=8ebd674da36802dde1ef082c76191019490bd4f8fcfa1b332de5c29d939cc791/2cb85386-694c-4ba4-a388-07f86114d42e/sep/video/1ed70222,418fe6ac,e74334bd,c5b649e2/master.json?base64_init=1";
    let request_metadata = fetch_metadata::FetchMetadata::origin(url);
    let json = request_metadata.perform().await.ok().unwrap();
    println!("{}", json.base_url);
}