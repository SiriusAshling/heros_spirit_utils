use std::{fmt::Display, fs, path::Path, io::{self, ErrorKind}};

pub fn feedback<D: Display, T, E: Display>(description: D, result: Result<T, E>) {
    match result {
        Ok(_) => { eprintln!("{} - Success", description); }
        Err(err) => { eprintln!("{} - Failure: {}", description, err); }
    };
}

pub fn feedback_and_then<D: Display, T, E: Display, F: FnOnce(T)>(description: D, result: Result<T, E>, then: F) {
    match result {
        Ok(ok) => {
            eprintln!("{} - Success", description);
            then(ok);
        }
        Err(err) => { eprintln!("{} - Failure: {}", description, err); }
    };
}

pub fn ensure_dir<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir(path).or_else(|err| if matches!(err.kind(), ErrorKind::AlreadyExists) { Ok(()) } else { Err(err) })
}
