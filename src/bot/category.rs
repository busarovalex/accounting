use accounting::UserId;
use error::{Error, ErrorKind};
use registry::Registry;

pub fn category<'a, I>(
    _commands: &mut I,
    _registry: &Registry,
    _user: UserId,
) -> Result<String, Error>
where
    I: Iterator<Item = &'a str>,
{
    Err(ErrorKind::NotImplementedYet.into())
}
