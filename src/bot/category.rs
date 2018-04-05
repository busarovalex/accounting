use registry::Registry;
use accounting::UserId;
use error::{ErrorKind, Error};

pub fn category<'a, I>(_commands: &mut I, _registry: &Registry, _user: UserId) -> Result<String, Error>
where
    I: Iterator<Item = &'a str>,
{
    Err(ErrorKind::NotImplementedYet.into())
}
