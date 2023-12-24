use std::{
    error::Error,
    fmt::Display,
    io::{self, Write},
};

macro_rules! eprintln_lock {
    ($($arg:tt)*) => {
        {
            let mut stderr = io::stderr().lock();
            writeln!(stderr, $($arg)*).unwrap();
        }
    };
}
pub(crate) use eprintln_lock;

pub fn try_or_feedback<D, F>(name: D, f: F)
where
    D: Display,
    F: FnOnce() -> Result<(), Box<dyn Error>>,
{
    eprintln_lock!("{name} - Starting");
    match f() {
        Ok(()) => eprintln_lock!("{name} - Finished"),
        Err(err) => eprintln_lock!("{name} - Failed: {err}"),
    }
}
