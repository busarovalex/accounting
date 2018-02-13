use std::path::PathBuf;

use persistence;

error_chain!{
    foreign_links {
        Persistence(persistence::error::Error) #[doc = "Error during persistence"];
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
        IncorrectApplicationUse(cause: String) {
            description("incorrect use of application")
            display("incorrect use of application: {}", &cause)   
        }
    }
}
