use std::fmt::Display;

pub trait ResultExtension: Sized {
    type Output;

    fn ok_feedback<D: Display>(self, description: D) -> Option<Self::Output>;

    fn feedback<D: Display>(self, description: D) {
        self.ok_feedback(description);
    }
}

impl<T, E> ResultExtension for Result<T, E>
where
    E: Display,
{
    type Output = T;

    fn ok_feedback<D: Display>(self, description: D) -> Option<Self::Output> {
        match self {
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
}
