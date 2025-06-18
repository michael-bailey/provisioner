use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub struct ExecutorError<T>
where
    T: Debug + Display,
{
    kind: T,
    // change this to be the username the dev is trying to modify
    message: String,
}

impl<T> ExecutorError<T>
where
    T: Debug + Display,
{
    pub fn new(kind: T, message: String) -> Self {
        Self { kind, message }
    }
}

impl<T> Display for ExecutorError<T>
where
    T: Debug + Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // todo: Change this be a sentence that uses username appropriately
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl<T> Error for ExecutorError<T> where T: Display + Debug {}
