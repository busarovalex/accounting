use clap;
use clap::Arg;
use failure::Error as FailureError;

use std::path::PathBuf;

#[derive(Debug)]
pub struct App {
    pub config_path: Option<PathBuf>,
}

impl App {
    pub fn from_args() -> Result<App, FailureError> {
        let matches = clap::App::new("Personal accounting")
            .about("Does personal accounting")
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true),
            ).get_matches();

        let config_path = matches.value_of("config").map(PathBuf::from);
        Ok(App { config_path })
    }
}
