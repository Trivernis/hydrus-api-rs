#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct Tag {
    pub negated: bool,
    pub name: String,
    pub namespace: Option<String>,
}

impl Eq for Tag {}

impl<S> From<S> for Tag
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let mut value = value.as_ref().trim();
        let negated = value.starts_with("-");
        value = value.trim_start_matches("-");
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
