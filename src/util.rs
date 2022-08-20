use std::{fmt::Display, fs, path::Path, io::{self, ErrorKind}};

pub fn feedback<T>(description: impl Display, result: Result<T, impl Display>) {
    match result {
        Ok(_) => { eprintln!("{} - Success", description); }
        Err(err) => { eprintln!("{} - Failure: {}", description, err); }
    };
}

pub fn feedback_and_then<T>(description: impl Display, result: Result<T, impl Display>, then: impl FnOnce(T)) {
    match result {
        Ok(ok) => {
            eprintln!("{} - Success", description);
            then(ok);
        }
        Err(err) => { eprintln!("{} - Failure: {}", description, err); }
    };
}

pub fn ensure_dir(path: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(path).or_else(|err| if matches!(err.kind(), ErrorKind::AlreadyExists) { Ok(()) } else { Err(err) })
}
