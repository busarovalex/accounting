use serde_yaml;

use std::path::PathBuf;
use std::io;
use std::num::ParseIntError;

use persistence;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        Persistence(persistence::error::Error) #[doc = "Error during persistence"];
        Yaml(serde_yaml::Error) #[doc = "Error during yamd (de)serialization"];
        ParseIntError(ParseIntError) #[doc = "Error during parsing"];
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
    }
}
