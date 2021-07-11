#[derive(Clone, Debug)]
pub struct Tag {
    pub name: String,
    pub namespace: Option<String>,
}

impl<S> From<S> for Tag
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let value = value.as_ref();
        if let Some((namespace, tag)) = value.split_once(":") {
            Self {
                namespace: Some(namespace.to_string()),
                name: tag.to_string(),
            }
        } else {
            Self {
                name: value.to_string(),
                namespace: None,
            }
        }
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}:{}", namespace, self.name)
        } else {
            self.name.clone()
        }
    }
}
