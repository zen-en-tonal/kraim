use itertools::Itertools;
use serde::{Deserialize, Serialize};

use super::{Between, Choice, Placeholder};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum FloatValueFactory {
    Scala(f32),
    Choice(Choice<Vec<f32>>),
    Between(Between<f32, f32>),
}

impl FloatValueFactory {
    pub fn scala(x: f32) -> Self {
        Self::Scala(x)
    }

    pub fn choice(x: Vec<f32>) -> Self {
        Self::Choice(Choice(x))
    }

    pub fn between(from: f32, to: f32, step: f32) -> Self {
        Self::Between(Between::new(from, to, step))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FloatFormatter(String);

impl<S: Into<f32>> IntoIterator for Between<f32, S> {
    type Item = f32;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = std::vec![];
        let mut current = self.from;
        let step: f32 = self.step.into();
        loop {
            vec.push(current);
            current = current + step;
            if current.abs() > self.to.abs() {
                break vec.into_iter();
            }
        }
    }
}

impl IntoIterator for Placeholder<FloatValueFactory, FloatFormatter> {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        fn format(formatter: Option<FloatFormatter>, x: f32) -> String {
            match formatter {
                Some(f) => format!("{}", x),
                None => x.to_string(),
            }
        }
        match self.factory {
            FloatValueFactory::Scala(s) => vec![format(self.format, s)].into_iter(),
            FloatValueFactory::Choice(c) => c
                .into_iter()
                .map(|x| format(self.format.clone(), x))
                .collect_vec()
                .into_iter(),
            FloatValueFactory::Between(b) => b
                .into_iter()
                .map(|x| format(self.format.clone(), x))
                .collect_vec()
                .into_iter(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parameter::Between;

    #[test]
    fn between_positive_f32() {
        let mut between = Between {
            from: 0.0,
            to: 1.0,
            step: 0.5,
        }
        .into_iter();
        assert_eq!(between.next(), Some(0.0));
        assert_eq!(between.next(), Some(0.5));
        assert_eq!(between.next(), Some(1.0));
        assert_eq!(between.next(), None);
    }

    #[test]
    fn between_negative_f32() {
        let mut between = Between {
            from: 0.0,
            to: -1.0,
            step: -0.5,
        }
        .into_iter();
        assert_eq!(between.next(), Some(0.0));
        assert_eq!(between.next(), Some(-0.5));
        assert_eq!(between.next(), Some(-1.0));
        assert_eq!(between.next(), None);
    }
}
