use download_vimeo::commandline::parser;

extern crate clap;

pub struct Params {
    url: String,
    output_path: String
}

#[tokio::main]
async fn main() {
    let matches = parser::Parser::origin().command.get_matches();
    let params = Params{url:matches.value_of("url").unwrap().to_string(), output_path:matches.value_of("output").unwrap().to_string()};
}
