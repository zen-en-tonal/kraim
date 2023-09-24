use chrono::NaiveDate;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{Between, Choice, Placeholder, Powerset};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DateValueFactory {
    Scala(NaiveDate),
    Choice(Choice<Vec<NaiveDate>>),
    Between(Between<NaiveDate, usize>),
    Powerset(Powerset<NaiveDate>),
}

impl DateValueFactory {
    pub fn scala(x: NaiveDate) -> Self {
        Self::Scala(x)
    }

    pub fn choice(x: Vec<NaiveDate>) -> Self {
        Self::Choice(Choice(x))
    }

    pub fn between(from: NaiveDate, to: NaiveDate, step: usize) -> Self {
        Self::Between(Between::new(from, to, step))
    }

    pub fn powerset(x: Vec<NaiveDate>) -> Self {
        Self::Powerset(Powerset::new(x))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DateFormatter(String);

impl IntoIterator for Placeholder<DateValueFactory, DateFormatter> {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        fn format(f: Option<DateFormatter>, x: NaiveDate) -> String {
            match f {
                Some(f) => format!("{}", x),
                None => x.to_string(),
            }
        }
        match self.factory {
            DateValueFactory::Scala(s) => vec![format(self.format, s)].into_iter(),
            DateValueFactory::Choice(c) => c
                .into_iter()
                .map(|x| format(self.format.clone(), x))
                .collect_vec()
                .into_iter(),
            DateValueFactory::Between(b) => b
                .into_iter()
                .map(|x| format(self.format.clone(), x))
                .collect_vec()
                .into_iter(),
            DateValueFactory::Powerset(p) => p
                .into_iter()
                .flatten()
                .map(|x| format(self.format.clone(), x))
                .collect_vec()
                .into_iter(),
        }
    }
}

impl<S: Into<usize>> IntoIterator for Between<NaiveDate, S> {
    type Item = NaiveDate;

    type IntoIter = std::iter::StepBy<std::iter::Take<chrono::naive::NaiveDateDaysIterator>>;

    fn into_iter(self) -> Self::IntoIter {
        let duration = (self.to - self.from).num_days();
        self.from
            .iter_days()
            .take(duration as usize)
            .step_by(self.step.into())
    }
}
