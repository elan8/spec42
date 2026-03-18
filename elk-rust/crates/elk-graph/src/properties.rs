use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PropertyKey(pub String);

impl From<&str> for PropertyKey {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PropertyValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropertyBag {
    entries: BTreeMap<PropertyKey, PropertyValue>,
}

impl PropertyBag {
    #[must_use]
    pub fn get(&self, key: &PropertyKey) -> Option<&PropertyValue> {
        self.entries.get(key)
    }

    pub fn insert(&mut self, key: impl Into<PropertyKey>, value: PropertyValue) {
        self.entries.insert(key.into(), value);
    }

    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = (&PropertyKey, &PropertyValue)> {
        self.entries.iter()
    }
}

