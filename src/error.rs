use std::error::Error;
use std::fmt::{self, Debug, Display};

#[derive(Debug)]
pub struct SimpleError<D: Display + Debug>(pub D);
impl<D: Display + Debug> Display for SimpleError<D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
impl<D: Display + Debug> Error for SimpleError<D> {}
