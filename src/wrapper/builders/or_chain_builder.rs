use crate::wrapper::or_chain::OrChain;
use crate::wrapper::tag::Tag;

#[derive(Debug)]
pub struct OrChainBuilder {
    tags: Vec<Tag>,
}

impl OrChainBuilder {
    pub fn new() -> Self {
        Self { tags: Vec::new() }
    }

    /// Adds a tag to the or expression
    pub fn add_tag(mut self, tag: Tag) -> Self {
        self.tags.push(tag);
        self
    }

    /// Adds multiple tags to the or expression
    pub fn add_tags(mut self, mut tags: Vec<Tag>) -> Self {
        self.tags.append(&mut tags);
        self
    }

    /// Builds the or chain
    pub fn build(self) -> OrChain {
        OrChain::new(self.tags)
    }
}
