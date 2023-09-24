use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{Between, Choice, Placeholder, Powerset};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IntValueFactory {
    Scala(i32),
    Choice(Choice<Vec<i32>>),
    Between(Between<i32, usize>),
    Powerset(Powerset<i32>),
}

impl IntValueFactory {
    pub fn scala(x: i32) -> Self {
        Self::Scala(x)
    }

    pub fn choice(x: Vec<i32>) -> Self {
        Self::Choice(Choice(x))
    }

    pub fn between(from: i32, to: i32, step: usize) -> Self {
        Self::Between(Between::new(from, to, step))
    }

    pub fn powerset(x: Vec<i32>) -> Self {
        Self::Powerset(Powerset::new(x))
    }
}

impl<S: Into<usize>> IntoIterator for Between<i32, S> {
    type Item = i32;

    type IntoIter = std::iter::StepBy<std::ops::Range<Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        (self.from..self.to + 1).step_by(self.step.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct IntFormatter;

impl IntoIterator for Placeholder<IntValueFactory, IntFormatter> {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self.factory {
            IntValueFactory::Scala(s) => vec![s.to_string()].into_iter(),
            IntValueFactory::Choice(c) => c
                .into_iter()
                .map(|x| x.to_string())
                .collect_vec()
                .into_iter(),
            IntValueFactory::Between(b) => b
                .into_iter()
                .map(|x| x.to_string())
                .collect_vec()
                .into_iter(),
            IntValueFactory::Powerset(p) => p
                .into_iter()
                .flatten()
                .map(|x| x.to_string())
                .collect_vec()
                .into_iter(),
        }
    }
}
