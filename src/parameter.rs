mod bool;
mod date;
mod float;
mod int;
mod string;

pub use bool::*;
pub use date::*;
pub use float::*;
pub use int::*;
pub use string::*;

use serde::{Deserialize, Serialize};

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
