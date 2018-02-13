use std::path::PathBuf;

#[derive(StructOpt, Debug)]
#[structopt(name = "Accounting", about = "Личный учет финансов")]
pub struct App {
    #[structopt(short="c", long="config", help = "path to config", parse(from_os_str))]
    pub config_path: Option<PathBuf>,

    #[structopt(subcommand)]
    pub command: Command
}


#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "add")]
    Add {
        #[structopt(name="ENTRY", help = "new entry")]
        input: Vec<String>
    },

    #[structopt(name = "bot")]
    Bot {

    },

    #[structopt(name = "list")]
    List {

    },

    #[structopt(name = "migrate")]
    Migrate {
        #[structopt(short = "a")]
        add: Vec<String>,
        #[structopt(short = "r")]
        remove: Option<String>
    },

    #[structopt(name = "user")]
    User {
        #[structopt(short="l", long="list")]
        list: bool,
        #[structopt(short="t", long="with-telegram-id")]
        telegram_id: Option<i64>
    }
}
