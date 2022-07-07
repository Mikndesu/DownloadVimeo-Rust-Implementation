use clap::{Command, Arg};

pub struct Parser<'a> {
    pub command: Command<'a>,
}

impl Parser<'_> {
    pub fn origin() -> Parser<'static> {
        Parser {
            command: Command::new("download_vimeo")
                .version("0.1.0")
                .about("download vimeo videos even private")
                .arg(
                    Arg::new("url")
                        .long("url")
                        .help("give the segment file url")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("output")
                        .long("output")
                        .help("give the output file path")
                        .takes_value(true)
                        .required(true),
                ),
        }
    }
}
