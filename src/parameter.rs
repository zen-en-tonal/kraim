pub mod bool;
pub mod date;
pub mod float;
pub mod int;
pub mod string;

use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use self::{
    bool::{BoolFormatter, BoolValueFactory},
    date::{DateFormatter, DateValueFactory},
    float::{FloatFormatter, FloatValueFactory},
    int::{IntFormatter, IntValueFactory},
    string::{StringFormatter, StringValueFactory},
};

pub type UrlParameter = HashMap<String, Value>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryParameter {
    key: String,
    value: Value,
}

impl QueryParameter {
    pub fn new(key: &str, value: Value) -> Self {
        Self {
            key: String::from(key),
            value,
        }
    }

    pub fn key(&self) -> &str {
        self.key.as_ref()
    }

    pub fn value(&self) -> &Value {
        &self.value
    }
}

type Pair = (String, String);

impl IntoIterator for QueryParameter {
    type Item = Pair;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.value
            .into_iter()
            .map(|x| (self.key.clone(), x))
            .collect_vec()
            .into_iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Value {
    Bool(Placeholder<BoolValueFactory, BoolFormatter>),
    String(Placeholder<StringValueFactory, StringFormatter>),
    Int(Placeholder<IntValueFactory, IntFormatter>),
    Float(Placeholder<FloatValueFactory, FloatFormatter>),
    Date(Placeholder<DateValueFactory, DateFormatter>),
}

impl Value {
    pub fn bool(factory: BoolValueFactory) -> Self {
        Value::Bool(Placeholder::new(factory, None))
    }

    pub fn bool_with_format(factory: BoolValueFactory, format: BoolFormatter) -> Self {
        Value::Bool(Placeholder::new(factory, Some(format)))
    }

    pub fn string(factory: StringValueFactory) -> Self {
        Value::String(Placeholder::new(factory, None))
    }

    pub fn int(factory: IntValueFactory) -> Self {
        Value::Int(Placeholder::new(factory, None))
    }

    pub fn float(factory: FloatValueFactory) -> Self {
        Value::Float(Placeholder::new(factory, None))
    }

    pub fn date(factory: DateValueFactory) -> Self {
        Value::Date(Placeholder::new(factory, None))
    }

    pub fn date_with_format(factory: DateValueFactory, format: DateFormatter) -> Self {
        Value::Date(Placeholder::new(factory, Some(format)))
    }
}

impl IntoIterator for Value {
    type Item = String;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Value::Bool(x) => x.into_iter(),
            Value::String(x) => x.into_iter(),
            Value::Int(x) => x.into_iter(),
            Value::Float(x) => x.into_iter(),
            Value::Date(x) => x.into_iter(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Placeholder<V, F> {
    factory: V,
    format: Option<F>,
}

impl<V, F> Placeholder<V, F> {
    pub fn new(factory: V, format: Option<F>) -> Self {
        Self { factory, format }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Choice<T>(T);

impl<T> IntoIterator for Choice<Vec<T>> {
    type Item = T;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Between<T, Step> {
    from: T,
    to: T,
    step: Step,
}

impl<T, Step> Between<T, Step> {
    pub fn new(from: T, to: T, step: Step) -> Self {
        Self { from, to, step }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Powerset<T>(Vec<T>);

impl<T> Powerset<T> {
    pub fn new(vec: Vec<T>) -> Self {
        Powerset(vec)
    }
}

impl<T> IntoIterator for Powerset<T>
where
    T: Clone,
{
    type Item = Vec<T>;

    type IntoIter = itertools::Powerset<std::vec::IntoIter<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter().powerset()
    }
}

#[cfg(test)]
mod tests {

    use crate::parameter::{bool::BoolValueFactory, Powerset, Value};

    use super::{string::StringValueFactory, QueryParameter};

    #[test]
    fn powerset() {
        let mut powerset = Powerset(vec![1, 2, 3]).into_iter();
        assert_eq!(powerset.next(), Some(vec![]));
        assert_eq!(powerset.next(), Some(vec![1]));
        assert_eq!(powerset.next(), Some(vec![2]));
        assert_eq!(powerset.next(), Some(vec![3]));
        assert_eq!(powerset.next(), Some(vec![1, 2]));
        assert_eq!(powerset.next(), Some(vec![1, 3]));
        assert_eq!(powerset.next(), Some(vec![2, 3]));
        assert_eq!(powerset.next(), Some(vec![1, 2, 3]));
    }

    #[test]
    fn serialize_date() {
        let date = QueryParameter {
            key: String::from("key"),
            value: Value::bool(BoolValueFactory::scala(true)),
        };
        println!("{}", serde_json::to_string(&date).unwrap())
    }

    #[test]
    fn parameter() {
        let p = QueryParameter::new(
            "key",
            Value::string(StringValueFactory::choice(&vec![
                String::from("a"),
                String::from("b"),
                String::from("c"),
            ])),
        );
        let mut iter = p.into_iter();
        assert_eq!(iter.next(), Some((String::from("key"), String::from("a"))));
        assert_eq!(iter.next(), Some((String::from("key"), String::from("b"))));
        assert_eq!(iter.next(), Some((String::from("key"), String::from("c"))));
        assert_eq!(iter.next(), None);
    }
}
