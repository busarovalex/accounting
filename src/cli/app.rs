use clap;
use clap::{Arg, ArgMatches, SubCommand};

use std::path::PathBuf;
use std::str::FromStr;

use accounting::statistics::TimePeriod;
use error::Error;

#[derive(Debug)]
pub struct App {
    pub config_path: Option<PathBuf>,
    pub command: Command,
}

#[derive(Debug)]
pub enum Command {
    Entry(EntryCmd),
    Migrate(MigrateCmd),
    User(UserCmd),
    Category(CategoryCmd),
    Report(TimePeriod, bool),
}

#[derive(Debug)]
pub enum EntryCmd {
    List,
    Add(String),
}

#[derive(Debug)]
pub enum MigrateCmd {
    Add(String, String),
    Remove(String),
    GenerateUid(String),
}

#[derive(Debug)]
pub enum UserCmd {
    List,
    Add(i64),
    SetTimezone(i32),
}

#[derive(Debug)]
pub enum CategoryCmd {
    List,
    Add(String, String),
}

impl App {
    pub fn from_args() -> Result<App, Error> {
        let matches = clap::App::new("Personal accounting")
            .about("Does personal accounting")
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true),
            ).subcommand(
                SubCommand::with_name("entry")
                    .about("controls accounting entries")
                    .arg(
                        Arg::with_name("add")
                            .short("a")
                            .long("add")
                            .help("adds new entry")
                            .value_name("INPUT")
                            .takes_value(true)
                            .conflicts_with("list")
                            .required_unless_one(&["list"]),
                    ).arg(
                        Arg::with_name("list")
                            .short("l")
                            .long("list")
                            .help("lists all entries")
                            .conflicts_with("add")
                            .required_unless_one(&["add"]),
                    ),
            ).subcommand(
                SubCommand::with_name("migrate")
                    .about("controls entries migration")
                    .arg(
                        Arg::with_name("add")
                            .short("a")
                            .long("add")
                            .help("adds a field with defined value to all entries")
                            .value_names(&["FIELD_NAME", "FIELD_VALUE"])
                            .number_of_values(2)
                            .takes_value(true)
                            .conflicts_with_all(&["remove", "generate"])
                            .required_unless_one(&["remove", "generate"]),
                    ).arg(
                        Arg::with_name("remove")
                            .short("r")
                            .long("remove")
                            .help("removes a field from all entries")
                            .value_name("FIELD_NAME")
                            .takes_value(true)
                            .conflicts_with_all(&["add", "generate"])
                            .required_unless_one(&["add", "generate"]),
                    ).arg(
                        Arg::with_name("generate")
                            .short("g")
                            .long("generate")
                            .help("generates a uid field from all entries")
                            .value_name("FIELD_NAME")
                            .takes_value(true)
                            .conflicts_with_all(&["add", "remove"])
                            .required_unless_one(&["add", "remove"]),
                    ),
            ).subcommand(
                SubCommand::with_name("user")
                    .about("controls users")
                    .arg(
                        Arg::with_name("list")
                            .short("l")
                            .long("list")
                            .help("lists all users")
                            .conflicts_with_all(&["add", "timezone"])
                            .required_unless_one(&["add", "timezone"]),
                    ).arg(
                        Arg::with_name("add")
                            .short("a")
                            .long("add")
                            .help("adds new user")
                            .value_name("TELEGRAM_ID")
                            .takes_value(true)
                            .conflicts_with_all(&["list", "timezone"])
                            .required_unless_one(&["list", "timezone"]),
                    ).arg(
                        Arg::with_name("timezone")
                            .short("t")
                            .long("timezone")
                            .help("sets current user timezone")
                            .value_name("TIMEZONE_OFFSET_MINUTES")
                            .takes_value(true)
                            .conflicts_with_all(&["list", "add"])
                            .required_unless_one(&["list", "add"]),
                    ),
            ).subcommand(
                SubCommand::with_name("category")
                    .about("controls categories")
                    .arg(
                        Arg::with_name("list")
                            .short("l")
                            .long("list")
                            .help("lists all categories")
                            .conflicts_with("add")
                            .required_unless_one(&["add"]),
                    ).arg(
                        Arg::with_name("add")
                            .short("a")
                            .long("add")
                            .help("adds new product-category assosiation")
                            .value_names(&["PRODUCT_NAME", "CATEGORY_NAME"])
                            .number_of_values(2)
                            .takes_value(true)
                            .conflicts_with("list")
                            .required_unless_one(&["list"]),
                    ),
            ).subcommand(
                SubCommand::with_name("report")
                    .about("generate a report")
                    .arg(
                        Arg::with_name("time_period")
                            .short("p")
                            .long("period")
                            .help("selects a time period")
                            .takes_value(true)
                            .possible_values(&["day", "week", "month", "year"]),
                    ).arg(
                        Arg::with_name("html")
                            .short("h")
                            .long("html")
                            .help("prints html representation"),
                    ),
            ).get_matches();

        let config_path = matches.value_of("config").map(PathBuf::from);

        let command = {
            if let Some(entry_matches) = matches.subcommand_matches("entry") {
                Command::Entry(entry(entry_matches))
            } else if let Some(migrate_matches) = matches.subcommand_matches("migrate") {
                Command::Migrate(migrate(migrate_matches))
            } else if let Some(user_matches) = matches.subcommand_matches("user") {
                Command::User(user(user_matches)?)
            } else if let Some(category_matches) = matches.subcommand_matches("category") {
                Command::Category(category(category_matches))
            } else if let Some(report_matches) = matches.subcommand_matches("report") {
                Command::Report(report(report_matches)?, report_matches.is_present("html"))
            } else {
                unreachable!()
            }
        };

        Ok(App {
            config_path,
            command,
        })
    }
}

fn report(matches: &ArgMatches) -> Result<TimePeriod, Error> {
    TimePeriod::from_str(matches.value_of("time_period").unwrap_or("month"))
}

fn entry(matches: &ArgMatches) -> EntryCmd {
    if let Some(new_entry) = matches.value_of("add") {
        EntryCmd::Add(new_entry.to_owned())
    } else if matches.is_present("list") {
        EntryCmd::List
    } else {
        unreachable!()
    }
}

fn migrate(matches: &ArgMatches) -> MigrateCmd {
    if let Some(mut add_input) = matches.values_of("add") {
        MigrateCmd::Add(
            add_input.next().unwrap().to_owned(),
            add_input.next().unwrap().to_owned(),
        )
    } else if let Some(field_name) = matches.value_of("remove") {
        MigrateCmd::Remove(field_name.to_owned())
    } else if let Some(field_name) = matches.value_of("generate") {
        MigrateCmd::GenerateUid(field_name.to_owned())
    } else {
        unreachable!()
    }
}

fn user(matches: &ArgMatches) -> Result<UserCmd, Error> {
    if let Some(telegram_id) = matches.value_of("add") {
        Ok(UserCmd::Add(i64::from_str(telegram_id)?))
    } else if matches.is_present("list") {
        Ok(UserCmd::List)
    } else if let Some(timezone) = matches.value_of("timezone") {
        Ok(UserCmd::SetTimezone(i32::from_str(timezone)?))
    } else {
        unreachable!()
    }
}

fn category(matches: &ArgMatches) -> CategoryCmd {
    if let Some(mut add_input) = matches.values_of("add") {
        CategoryCmd::Add(
            add_input.next().unwrap().to_owned(),
            add_input.next().unwrap().to_owned(),
        )
    } else if matches.is_present("list") {
        CategoryCmd::List
    } else {
        unreachable!()
    }
}
