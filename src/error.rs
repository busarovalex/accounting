use serde_yaml;

use std::path::PathBuf;
use std::io;
use std::num::ParseIntError;

use persistence;
use lettre_email::error::Error as EmailError;
use lettre::smtp::error::Error as SmtpError;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        Persistence(persistence::error::Error) #[doc = "Error during persistence"];
        Yaml(serde_yaml::Error) #[doc = "Error during yamd (de)serialization"];
        ParseIntError(ParseIntError) #[doc = "Error during parsing"];
        Email(EmailError) #[doc = "Error during sending email"];
        Smtp(SmtpError) #[doc = "Error during smtp"];
    }

    errors {
        InvalidRegistryPath(used_path: PathBuf) {
            description("An error occurred during registry work")
            display("data path {:?} is not a directory", used_path)
        }
        NumberOfLauchesExeeded {
            description("could not start bot after number of launches")
            display("could not start bot after number of launches")
        }
        ProductAlreadyHasCategory(product: String, category: String) {
            description("product already has a category")
            display("product \"{}\" already has a category:\"{}\"", product, category)   
        }
        Calculation(reason: String) {
            description("error during calculation")
            display("Error during calculation: {}", reason)   
        }
        InvalidEnumVariant {
            description("invalid enum variant")
            display("invalid enum variant was provided")   
        }
        NoDataForPeriod {
            description("no data for report in this period")
            display("no data for report in this period")   
        }
        InvalidDate {
            description("invalid date provided")
            display("invalid date provided")
        }
        EmailNotSetUp(property: String) {
            description("email sending is not set up")
            display("email sending is not set up: add \"{}\" property in config", property)
        }
        BotUsage(reason: String){
            description("wrong bot usage")
            display("Неверное использование бота: {}", reason)   
        }
    }
}
