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
    Null,
    Array(Vec<PropertyValue>),
    Object(BTreeMap<String, PropertyValue>),
}

impl PropertyValue {
    #[must_use]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(v) => Some(*v),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Self::Int(v) => Some(*v),
            _ => None,
        }
    }

    #[must_use]
    pub const fn as_f64(&self) -> Option<f64> {
        match self {
            Self::Float(v) => Some(*v),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(v) => Some(v.as_str()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_array(&self) -> Option<&[PropertyValue]> {
        match self {
            Self::Array(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_object(&self) -> Option<&BTreeMap<String, PropertyValue>> {
        match self {
            Self::Object(v) => Some(v),
            _ => None,
        }
    }
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

    #[must_use]
    pub fn get_str(&self, key: &PropertyKey) -> Option<&str> {
        self.get(key).and_then(PropertyValue::as_str)
    }

    pub fn insert(&mut self, key: impl Into<PropertyKey>, value: PropertyValue) {
        self.entries.insert(key.into(), value);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&PropertyKey, &PropertyValue)> {
        self.entries.iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

