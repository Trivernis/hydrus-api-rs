use crate::utils::tag_list_to_string_list;
use crate::wrapper::tag::Tag;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct OrChain {
    tags: Vec<Tag>,
}

impl Eq for OrChain {}

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
        lazy_static! {
            static ref CHAIN_REGEX: Regex = Regex::new(r#"(\s|'|")or(\s|'|")"#).unwrap();
        }
        let s = s.as_ref().to_ascii_lowercase();
        let tags = CHAIN_REGEX
            .split(&s)
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
        log::debug!("String parsed to or-chain {:?}", tags);

        Self { tags }
    }
}
