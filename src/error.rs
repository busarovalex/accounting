use std::path::PathBuf;

#[derive(Debug, Fail)]
pub enum AppError {
    #[fail(display = "Actix error: {}", code)]
    ActixError { code: i32 },
    #[fail(display = "Not implemented yet")]
    NotImplementedYet,
    #[fail(display = "{}", text)]
    Any { text: &'static str },
    #[fail(display = "data path {:?} is not a directory", used_path)]
    InvalidRegistryPath { used_path: PathBuf },
    #[fail(display = "could not start bot after number of launches")]
    NumberOfLauchesExeeded,
    #[fail(
        display = "product \"{}\" already has a category:\"{}\"",
        product,
        category
    )]
    ProductAlreadyHasCategory { product: String, category: String },
    #[fail(display = "Error during calculation: {}", reason)]
    Calculation { reason: String },
    #[fail(display = "invalid enum variant was provided")]
    InvalidEnumVariant,
    #[fail(display = "no data for report in this period")]
    NoDataForPeriod,
    #[fail(display = "invalid date provided")]
    InvalidDate,
    #[fail(
        display = "email sending is not set up: add \"{}\" property in config",
        property
    )]
    EmailNotSetUp { property: String },
    #[fail(display = "wrong bot usage: {}", reason)]
    BotUsage { reason: String },
}
