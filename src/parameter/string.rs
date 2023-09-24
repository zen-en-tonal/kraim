use serde::{Deserialize, Serialize};

use super::{Choice, Placeholder};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StringValueFactory {
    Scala(String),
    Choice(Choice<Vec<String>>),
}

impl StringValueFactory {
    pub fn scala(s: &str) -> StringValueFactory {
        Self::Scala(s.to_string())
    }

    pub fn choice(vec: &Vec<String>) -> StringValueFactory {
        Self::Choice(Choice(vec.clone()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StringFormatter {}

impl IntoIterator for Placeholder<StringValueFactory, StringFormatter> {
    fn into_iter(self) -> std::vec::IntoIter<String> {
        match self.factory {
            StringValueFactory::Scala(s) => vec![s].into_iter(),
            StringValueFactory::Choice(s) => s.into_iter(),
        }
    }

    type Item = String;

    type IntoIter = std::vec::IntoIter<String>;
}
