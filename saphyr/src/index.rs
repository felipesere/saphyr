use crate::{annotated, MarkedYaml, MarkedYamlOwned, Yaml, YamlData, YamlDataOwned, YamlOwned};

/// A trait to safely index into a structure with an `Accessor`.
/// This will never panic and return an `Option::None` on failure.
pub trait SafelyIndex<Y>: Sized {
    /// A trait to index into a structure with an `Index`
    fn get(&self, key: impl Accessor<Y>) -> Option<&Y>;
}

impl<YAML, Z: SafelyIndex<YAML>> SafelyIndex<YAML> for Option<Z> {
    fn get(&self, key: impl Accessor<YAML>) -> Option<&YAML> {
        self.as_ref().and_then(|data| data.get(key))
    }
}

impl<YAML, T: SafelyIndex<YAML>> SafelyIndex<YAML> for &T {
    fn get(&self, key: impl Accessor<YAML>) -> Option<&YAML> {
        (*self).get(key)
    }
}

impl SafelyIndex<YamlOwned> for YamlOwned {
    fn get(&self, key: impl Accessor<YamlOwned>) -> Option<&YamlOwned> {
        key.index_into(self)
    }
}

impl<'input> SafelyIndex<Yaml<'input>> for Yaml<'input> {
    fn get(&self, key: impl Accessor<Yaml<'input>>) -> Option<&Yaml<'input>> {
        key.index_into(self)
    }
}

impl<N> SafelyIndex<YamlDataOwned<N>> for YamlDataOwned<N>
where
    N: annotated::AnnotatedNodeOwned + From<Self>,
{
    fn get(&self, key: impl Accessor<YamlDataOwned<N>>) -> Option<&YamlDataOwned<N>> {
        key.index_into(self)
    }
}

impl<'input, N> SafelyIndex<YamlData<'input, N>> for YamlData<'input, N>
where
    N: annotated::AnnotatedNode + From<Self>,
{
    fn get(&self, key: impl Accessor<YamlData<'input, N>>) -> Option<&YamlData<'input, N>> {
        key.index_into(self)
    }
}

// These will have to be cleverer!
impl<'a> SafelyIndex<MarkedYaml<'a>> for MarkedYaml<'a> {
    fn get(&self, key: impl Accessor<MarkedYaml<'a>>) -> Option<&MarkedYaml<'a>> {
        key.index_into(self)
    }
}

impl SafelyIndex<MarkedYamlOwned> for MarkedYamlOwned {
    fn get(&self, key: impl super::Accessor<MarkedYamlOwned>) -> Option<&MarkedYamlOwned> {
        key.index_into(self)
    }
}

/// A trait to denote a type that can be used for indexing YAML
pub trait Accessor<Y> {
    /// something important
    fn index_into(self, yaml: &Y) -> Option<&Y>;
}

impl Accessor<MarkedYamlOwned> for usize {
    fn index_into(self, yaml: &MarkedYamlOwned) -> Option<&MarkedYamlOwned> {
        yaml.data.as_sequence_get(self)
    }
}

impl<'input> Accessor<MarkedYaml<'input>> for usize {
    fn index_into<'y>(self, yaml: &'y MarkedYaml<'input>) -> Option<&'y MarkedYaml<'input>> {
        yaml.data.as_sequence_get(self)
    }
}

impl Accessor<MarkedYamlOwned> for &str {
    fn index_into(self, yaml: &MarkedYamlOwned) -> Option<&MarkedYamlOwned> {
        yaml.data.as_mapping_get(self)
    }
}

impl<'input> Accessor<MarkedYaml<'input>> for &str {
    fn index_into<'y>(self, yaml: &'y MarkedYaml<'input>) -> Option<&'y MarkedYaml<'input>> {
        yaml.data.as_mapping_get(self)
    }
}

impl Accessor<MarkedYamlOwned> for String {
    fn index_into(self, yaml: &MarkedYamlOwned) -> Option<&MarkedYamlOwned> {
        self.as_str().index_into(yaml)
    }
}

impl<'input> Accessor<MarkedYaml<'input>> for String {
    fn index_into<'y>(self, yaml: &'y MarkedYaml<'input>) -> Option<&'y MarkedYaml<'input>> {
        self.as_str().index_into(yaml)
    }
}
