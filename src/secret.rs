use serde::Deserialize;

#[derive(Deserialize)]
#[serde(from = "String")]
pub struct Secret {
    value: String,
}

impl Secret {
    #[allow(dead_code)]
    pub fn value(&self) -> &str {
        &self.value
    }
}

impl std::fmt::Debug for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Secret")
            .field("value", &"********")
            .finish()
    }
}

impl std::fmt::Display for Secret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "********")
    }
}

impl<T: Into<String>> From<T> for Secret {
    fn from(value: T) -> Self {
        Self {
            value: value.into(),
        }
    }
}
