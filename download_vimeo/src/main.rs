use download_vimeo::commandline::parser;

extern crate clap;

#[tokio::main]
async fn main() {
    let matches = parser::Parser::origin().command.get_matches();
    if let Some(o) = matches.value_of("url") {
        println!("{}", o);
    }
    if let Some(o) = matches.value_of("output") {
        println!("{}", o);
    }
}
