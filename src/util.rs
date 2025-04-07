use std::fmt::Display;
use std::fs::{self, File, ReadDir};
use std::io;
use std::path::Path;

use crate::Result;

pub fn feedback<T>(description: impl Display, result: Result<T>) -> Option<T> {
    match result {
        Ok(t) => {
            eprintln!("{description} - Success");
            Some(t)
        }
        Err(err) => {
            eprintln!("{description} - Failure: {err}");
            None
        }
    }
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    fn read(path: &Path) -> Result<Vec<u8>> {
        Ok(fs::read(path).map_err(|err| annotate_io_err(err, "read", path))?)
    }

    read(path.as_ref())
}

pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    fn read_to_string(path: &Path) -> Result<String> {
        Ok(fs::read_to_string(path).map_err(|err| annotate_io_err(err, "read", path))?)
    }

    read_to_string(path.as_ref())
}

pub fn read_dir<P: AsRef<Path>>(path: P) -> Result<ReadDir> {
    fn read_dir(path: &Path) -> Result<ReadDir> {
        Ok(fs::read_dir(path).map_err(|err| annotate_io_err(err, "read", path))?)
    }

    read_dir(path.as_ref())
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> Result<()> {
    fn write(path: &Path, contents: &[u8]) -> Result<()> {
        ensure_parent_dir(path)?;
        Ok(fs::write(path, contents).map_err(|err| annotate_io_err(err, "write", path))?)
    }

    write(path.as_ref(), contents.as_ref())
}

fn ensure_parent_dir(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| annotate_io_err(err, "create", path))?;
    }
    Ok(())
}

// pub fn read_dir_recursive<P: AsRef<Path>>(path: P) -> Result<ReadDirRecursive> {
//     ReadDirRecursive::new(path.as_ref())
// }

// pub struct ReadDirRecursive {
//     dir_stack: Vec<ReadDir>,
// }
// impl ReadDirRecursive {
//     fn new(path: &Path) -> Result<Self> {
//         Ok(Self {
//             dir_stack: vec![_read_dir(path)?],
//         })
//     }

//     fn check_dir_entry(
//         &mut self,
//         dir_entry: io::Result<DirEntry>,
//     ) -> Result<ControlFlow<DirEntry>> {
//         let dir_entry = dir_entry?;
//         if dir_entry.file_type()?.is_dir() {
//             self.dir_stack.push(fs::read_dir(dir_entry.path())?);
//             Ok(ControlFlow::Continue(()))
//         } else {
//             Ok(ControlFlow::Break(dir_entry))
//         }
//     }
// }
// impl Iterator for ReadDirRecursive {
//     type Item = Result<DirEntry>;

//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//             let current = self.dir_stack.last_mut()?;
//             match current.next() {
//                 None => {
//                     self.dir_stack.pop();
//                 }
//                 Some(dir_entry) => match self.check_dir_entry(dir_entry) {
//                     Ok(ControlFlow::Continue(())) => {}
//                     Ok(ControlFlow::Break(dir_entry)) => return Some(Ok(dir_entry)),
//                     Err(err) => return Some(Err(err)),
//                 },
//             }
//         }
//     }
// }

pub fn file_open<P: AsRef<Path>>(path: P) -> Result<File> {
    fn file_open(path: &Path) -> Result<File> {
        Ok(File::open(path).map_err(|err| annotate_io_err(err, "open", path))?)
    }

    file_open(path.as_ref())
}

pub fn file_create<P: AsRef<Path>>(path: P) -> Result<File> {
    fn file_create(path: &Path) -> Result<File> {
        ensure_parent_dir(path)?;
        Ok(File::create(path).map_err(|err| annotate_io_err(err, "create", path))?)
    }

    file_create(path.as_ref())
}

#[allow(clippy::needless_pass_by_value)]
fn annotate_io_err(err: io::Error, action: &str, path: &Path) -> String {
    format!("Failed to {action} \"{}\": {err}", path.display())
}

macro_rules! const_concat {
    ($left:expr, $right:expr) => {{
        const LEFT: &[u8] = $left.as_bytes();
        const RIGHT: &[u8] = $right.as_bytes();
        const LEN: usize = LEFT.len() + RIGHT.len();

        const BYTES: [u8; LEN] = {
            let mut bytes = [0_u8; LEN];
            let mut index = 0;
            while index < LEFT.len() {
                bytes[index] = LEFT[index];
                index += 1;
            }
            while index < LEN {
                bytes[index] = RIGHT[index - LEFT.len()];
                index += 1;
            }
            bytes
        };

        match std::str::from_utf8(&BYTES) {
            Ok(str) => str,
            Err(_) => panic!("const message concatenation produced invalid utf8"),
        }
    }};
}
pub(crate) use const_concat;
