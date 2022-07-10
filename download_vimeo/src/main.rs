use std::path::Path;

use download_vimeo::{commandline::parser, network::fetch_metadata};
use url::Url;

extern crate clap;

pub struct Params<'a> {
    url: Url,
    output_path: &'a Path
}

#[tokio::main]
async fn main() {
    let matches = parser::Parser::origin().command.get_matches();
    let params = Params{url:Url::parse(matches.value_of("url").unwrap()).unwrap(), output_path:Path::new(matches.value_of("output").unwrap())};
    let request_metadata = fetch_metadata::FetchMetadata::origin(params.url.clone());
    let json = request_metadata.perform().await.ok().unwrap();
    let base_url = params.url.join(json.base_url.as_str());
    println!("{}", base_url.unwrap());
}
