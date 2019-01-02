use failure::Error as FailureError;

use accounting::UserId;
use registry::Registry;

pub fn category<'a, I>(
    _commands: &mut I,
    _registry: &Registry,
    _user: UserId,
) -> Result<String, FailureError>
where
    I: Iterator<Item = &'a str>,
{
    Err(crate::error::AppError::NotImplementedYet.into())
}
