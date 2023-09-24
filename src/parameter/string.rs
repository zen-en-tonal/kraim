use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{Choice, Placeholder, Powerset};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StringValueFactory {
    Scala(String),
    Choice(Choice<Vec<String>>),
    Powerset(Powerset<String>),
}

impl StringValueFactory {
    pub fn scala(s: &str) -> StringValueFactory {
        Self::Scala(s.to_string())
    }

    pub fn choice(vec: &Vec<String>) -> StringValueFactory {
        Self::Choice(Choice(vec.clone()))
    }

    pub fn powerset(vec: &Vec<String>) -> StringValueFactory {
        Self::Powerset(Powerset(vec.clone()))
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
            StringValueFactory::Powerset(s) => s.into_iter().flatten().collect_vec().into_iter(),
        }
    }

    type Item = String;

    type IntoIter = std::vec::IntoIter<String>;
}
