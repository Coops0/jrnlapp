use serde::Serialize;

pub struct JrnlIosError(anyhow::Error);
pub type JrnlIosResult<T> = Result<T, JrnlIosError>;

impl<E: Into<anyhow::Error>> From<E> for JrnlIosError {
    fn from(e: E) -> Self {
        JrnlIosError(e.into())
    }
}

impl Serialize for JrnlIosError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.to_string().serialize(serializer)
    }
}
