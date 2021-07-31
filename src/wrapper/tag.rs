#[derive(Clone, Debug)]
pub struct Tag {
    pub negated: bool,
    pub name: String,
    pub namespace: Option<String>,
}

impl<S> From<S> for Tag
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let value = value.as_ref().trim();
        let negated = value.strip_prefix("-").is_some();
        if let Some((namespace, tag)) = value.split_once(":") {
            Self {
                negated,
                namespace: Some(namespace.to_string()),
                name: tag.to_string(),
            }
        } else {
            Self {
                negated,
                name: value.to_string(),
                namespace: None,
            }
        }
    }
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        let negation = if self.negated { "-" } else { "" };
        if let Some(namespace) = &self.namespace {
            format!("{}{}:{}", negation, namespace, self.name)
        } else {
            format!("{}{}", negation, self.name)
        }
    }
}
