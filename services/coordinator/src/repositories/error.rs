use derive_more::Display;

#[derive(Debug, Display)]
pub enum RepoError {
    #[display(fmt = "io error: {}", _0)]
    IoError(String),
    #[display(fmt = "serialize error: {}", _0)]
    SerializeError(String),
    #[display(fmt = "deserialize error: {}", _0)]
    DeserializeError(String),
}
