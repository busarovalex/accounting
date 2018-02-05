use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "Accounting", about = "Личный учет финансов")]
pub struct App {
    #[structopt(short="d", long="data", help = "data folder", parse(from_os_str))]
    pub data: PathBuf,

    #[structopt(short="b", long="bot", help = "start telegram bot")]
    pub bot: bool,

    #[structopt(short="l", long="list", help = "list entries")]
    pub list: bool,

    #[structopt(name="ENTRY", help = "new entry")]
    pub entry: Option<String>
}