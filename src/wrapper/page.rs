#[derive(Clone)]
pub struct HydrusPage {
    pub id: PageIdentifier,
}

#[derive(Clone)]
pub enum PageIdentifier {
    Name(String),
    Key(String),
}

impl PageIdentifier {
    pub fn name<S: ToString>(name: S) -> Self {
        Self::Name(name.to_string())
    }

    pub fn key<S: ToString>(key: S) -> Self {
        Self::Key(key.to_string())
    }
}
