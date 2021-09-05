use crate::utils::tag_list_to_string_list;
use crate::wrapper::tag::Tag;

#[derive(Clone, Debug)]
pub struct OrChain {
    tags: Vec<Tag>,
}

impl OrChain {
    /// Creates a new or chain directly from a list of tags
    pub fn new(tags: Vec<Tag>) -> Self {
        Self { tags }
    }

    /// Returns the tags of this or chain
    pub fn tags(&self) -> &Vec<Tag> {
        &self.tags
    }

    pub(crate) fn into_string_list(self) -> Vec<String> {
        tag_list_to_string_list(self.tags)
    }
}

impl<S> From<S> for OrChain
where
    S: AsRef<str>,
{
    fn from(s: S) -> Self {
        let s = s.as_ref().to_ascii_lowercase();
        let tags = s
            .split("or")
            .map(|mut t| {
                t = t
                    .trim_start()
                    .trim_start_matches("'")
                    .trim_start_matches("\"");
                t = t.trim_end().trim_end_matches("'").trim_end_matches("\"");
                t
            })
            .map(Tag::from)
            .collect();

        Self { tags }
    }
}
