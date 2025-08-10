use crate::{annotated, MarkedYaml, MarkedYamlOwned, Yaml, YamlData, YamlDataOwned, YamlOwned};

/// A trait to safely index into a structure with an `Accessor`.
/// This will never panic and return an `Option::None` on failure.
pub trait SafelyIndex<X = Self> {
    /// A trait to index into a structure with an `Index`
    fn get(&self, key: impl Into<Accessor>) -> Option<&X>;
}

pub enum Accessor {
    Field(String),
    Index(usize),
}

impl From<usize> for Accessor {
    fn from(val: usize) -> Self {
        Accessor::Index(val)
    }
}

impl From<String> for Accessor {
    fn from(val: String) -> Self {
        Accessor::Field(val)
    }
}

impl From<&str> for Accessor {
    fn from(val: &str) -> Self {
        Accessor::Field(val.to_string())
    }
}

impl<Z: SafelyIndex> SafelyIndex<Z> for Option<Z> {
    fn get(&self, key: impl Into<Accessor>) -> Option<&Z> {
        self.as_ref().and_then(|data| data.get(key))
    }
}

impl<Z: SafelyIndex> SafelyIndex<Z> for Option<&Z> {
    fn get(&self, key: impl Into<Accessor>) -> Option<&Z> {
        self.as_ref().and_then(|data| data.get(key))
    }
}

impl<T: SafelyIndex> SafelyIndex<T> for &T {
    fn get(&self, key: impl Into<Accessor>) -> Option<&T> {
        (*self).get(key)
    }
}

impl SafelyIndex for YamlOwned {
    fn get(&self, key: impl Into<Accessor>) -> Option<&YamlOwned> {
        match key.into() {
            Accessor::Field(f) => self.as_mapping_get(f.as_str()),
            Accessor::Index(i) => self.as_sequence_get(i),
        }
    }
}

impl<'input> SafelyIndex for Yaml<'input> {
    fn get(&self, key: impl Into<Accessor>) -> Option<&Yaml<'input>> {
        match key.into() {
            Accessor::Field(f) => self.as_mapping_get(f.as_str()),
            Accessor::Index(i) => self.as_sequence_get(i),
        }
    }
}

impl<N> SafelyIndex<N> for YamlDataOwned<N>
where
    N: annotated::AnnotatedNodeOwned
        + From<Self>
        + PartialEq<<N as annotated::AnnotatedNodeOwned>::HashKey>,
{
    fn get(&self, key: impl Into<Accessor>) -> Option<&N> {
        match key.into() {
            Accessor::Field(f) => self.as_mapping_get(f.as_str()),
            Accessor::Index(i) => self.as_sequence_get(i),
        }
    }
}

impl<N> SafelyIndex<N> for YamlData<'_, N>
where
    N: annotated::AnnotatedNode
        + From<Self>
        + for<'a> PartialEq<<N as annotated::AnnotatedNode>::HashKey<'a>>,
{
    fn get(&self, key: impl Into<Accessor>) -> Option<&N> {
        match key.into() {
            Accessor::Index(i) => self.as_sequence_get(i),
            Accessor::Field(f) => self.as_mapping_get(f.as_str()),
        }
    }
}

impl<'a> SafelyIndex for MarkedYaml<'a> {
    fn get(&self, key: impl Into<Accessor>) -> Option<&MarkedYaml<'a>> {
        match key.into() {
            Accessor::Field(f) => self.data.as_mapping_get(f.as_str()),
            Accessor::Index(i) => self.data.as_sequence_get(i),
        }
    }
}

impl SafelyIndex for MarkedYamlOwned {
    fn get(&self, key: impl Into<Accessor>) -> Option<&MarkedYamlOwned> {
        match key.into() {
            Accessor::Field(f) => self.data.as_mapping_get(f.as_str()),
            Accessor::Index(i) => self.data.as_sequence_get(i),
        }
    }
}
