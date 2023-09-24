use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{counter::Counter, parameter::*};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Query {
    url: String,
    query: Vec<QueryParameter>,
    param: UrlParameter,
}

impl IntoIterator for Query {
    type Item = String;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = vec![];
        for p in self.param {
            vec.push(
                p.1.into_iter()
                    .map(|x| Type::Param(p.0.clone(), x))
                    .collect_vec()
                    .into_iter(),
            );
        }
        for q in self.query {
            vec.push(
                q.into_iter()
                    .map(|x| Type::Query(x.0, x.1))
                    .collect_vec()
                    .into_iter(),
            );
        }
        let counter = Counter::new(vec);
        Iter {
            url: self.url,
            inner: counter,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Query(String, String),
    Param(String, String),
}

pub struct Iter {
    url: String,
    inner: Counter<std::vec::IntoIter<Type>>,
}

impl Iterator for Iter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(values) = self.inner.next() {
            let mut querys = vec![];
            let mut url = self.url.clone();
            for v in values {
                match v {
                    Type::Query(key, value) => querys.push(format!("{}={}", key, value)),
                    Type::Param(key, value) => {
                        let replace = format!("{{{}}}", key);
                        url = url.replace(&replace, &value);
                    }
                }
            }
            url = format!("{}?{}", url, querys.join("&"));
            return Some(url);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::parameter::{int::IntValueFactory, string::StringValueFactory, *};

    use super::Query;

    #[test]
    fn into_iter() {
        let mut p = HashMap::default();
        p.insert(
            String::from("id"),
            Value::int(IntValueFactory::between(1, 2, 1)),
        );
        let q = Query {
            url: String::from("http://example.com/{id}"),
            query: vec![
                QueryParameter::new(
                    "filter[foo][]",
                    Value::string(StringValueFactory::choice(&vec![
                        String::from("a"),
                        String::from("b"),
                    ])),
                ),
                QueryParameter::new(
                    "filter[bar][]",
                    Value::string(StringValueFactory::choice(&vec![
                        String::from("a"),
                        String::from("b"),
                    ])),
                ),
            ],
            param: p,
        };
        let mut urls = q.into_iter();
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=a&filter[bar][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=a&filter[bar][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=b&filter[bar][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=b&filter[bar][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=a&filter[bar][]=b"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=a&filter[bar][]=b"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=b&filter[bar][]=b"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=b&filter[bar][]=b"
            ))
        );
        assert_eq!(urls.next(), None);
    }
}
