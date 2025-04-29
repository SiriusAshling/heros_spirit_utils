use crate::{cli::FromPrompt, Result};

pub trait OptionExtension {
    type Value;

    fn unwrap_or_prompt(self) -> Result<Self::Value>;
}

impl<T: FromPrompt> OptionExtension for Option<T> {
    type Value = T;

    fn unwrap_or_prompt(self) -> Result<Self::Value> {
        match self {
            None => T::from_prompt(),
            Some(t) => Ok(t),
        }
    }
}
