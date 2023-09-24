use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{Choice, Placeholder};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BoolValueFactory {
    Scala(bool),
    Choice(Choice<bool>),
}

impl BoolValueFactory {
    pub fn scala(x: bool) -> Self {
        Self::Scala(x)
    }

    pub fn choice() -> Self {
        Self::Choice(Choice(true))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum BoolFormatter {
    ZeroOne,
}

impl BoolFormatter {
    pub fn format(&self, x: bool) -> String {
        match self {
            BoolFormatter::ZeroOne => zero_one(x),
        }
    }
}

fn zero_one(x: bool) -> String {
    if x == true {
        String::from("1")
    } else {
        String::from("0")
    }
}

impl IntoIterator for Choice<bool> {
    type Item = bool;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![true, false].into_iter()
    }
}

impl IntoIterator for Placeholder<BoolValueFactory, BoolFormatter> {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        fn format(format: Option<BoolFormatter>, x: bool) -> String {
            match format {
                Some(f) => f.format(x),
                None => x.to_string(),
            }
        }
        match self.factory {
            BoolValueFactory::Scala(s) => vec![format(self.format, s)].into_iter(),
            BoolValueFactory::Choice(c) => c
                .into_iter()
                .map(|x| format(self.format.clone(), x))
                .collect_vec()
                .into_iter(),
        }
    }
}
