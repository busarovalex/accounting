use serde_json;

use std::path::PathBuf;
use std::io;

error_chain!{
    foreign_links {
        Io(io::Error) #[doc = "Error during IO"];
        SerDe(serde_json::Error) #[doc = "Error during (de)serialization"];
    }

    errors {
        AlreadyInUse(used_path: PathBuf) {
            description("An error occurred during persistense")
            display("Path {:?} is already used for another table", used_path)
        }

        IoError {
            description("An IO error occurred during persistense")
            display("An IO error occurred during persistense")   
        }

        JsonValueIsNotObject {
            description("An IO error occurred during persistense")
            display("deserialized json value appears to be not an object")
        }

        NoSuchKeyInJsonValue {
            description("An IO error occurred during persistense")
            display("no such key found in json object")
        }

        KeyWasAlreadyInObject {
            description("An IO error occurred during persistense")
            display("key was already found in json object")
        }
    }
}
