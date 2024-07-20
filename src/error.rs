#[derive(Debug, Clone)]
pub struct Error(String);
impl<E: std::error::Error> From<E> for Error {
    fn from(value: E) -> Self {
        Error(format!("{:?}", value))
    }
}