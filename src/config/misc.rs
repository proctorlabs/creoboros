use super::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T: Clone> OneOrMany<T> {
    pub fn get(&self) -> Vec<T> {
        match self {
            OneOrMany::One(t) => vec![t.clone()],
            OneOrMany::Many(t) => t.clone(),
        }
    }
}

impl<T> Default for OneOrMany<T>
where
    T: Default,
{
    fn default() -> Self {
        OneOrMany::One(T::default())
    }
}
