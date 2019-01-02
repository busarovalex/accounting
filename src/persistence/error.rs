use std::path::PathBuf;

#[derive(Debug, Fail)]
pub enum PersistenceError {
    #[fail(display = "Path {:?} already in use", used_path)]
    AlreadyInUse { used_path: PathBuf },
    #[fail(display = "io error")]
    IoError,
    #[fail(display = "Json value is not object")]
    JsonValueIsNotObject,
    #[fail(display = "No such key in json value")]
    NoSuchKeyInJsonValue,
    #[fail(display = "Key was already in object")]
    KeyWasAlreadyInObject,
}
