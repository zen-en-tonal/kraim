use std::collections::HashMap;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{counter::Counter, parameter::*};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Query<'a> {
    url: &'a str,
    query: Vec<(&'a str, Value)>,
    param: HashMap<&'a str, Value>,
}

impl<'a> IntoIterator for Query<'a> {
    type Item = String;

    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut vec = vec![];
        for p in self.param {
            vec.push(
                p.1.into_iter()
                    .map(|x| Type::Param(p.0.to_owned(), x))
                    .collect_vec()
                    .into_iter(),
            );
        }
        for q in self.query {
            vec.push(
                q.1.into_iter()
                    .map(|x| Type::Query(q.0.to_owned(), x))
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
enum Type {
    Query(String, String),
    Param(String, String),
}

pub struct Iter<'a> {
    url: &'a str,
    inner: Counter<std::vec::IntoIter<Type>>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(values) = self.inner.next() {
            let mut querys = vec![];
            let mut url = self.url.to_string();
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
    use crate::parameter::*;

    use super::Query;

    #[test]
    fn into_iter() {
        let query = Query {
            url: "http://example.com/{id}",
            query: vec![
                (
                    "filter[foo][]",
                    Value::string(StringValueFactory::choice(&vec![
                        String::from("a"),
                        String::from("b"),
                    ])),
                ),
                (
                    "filter[foo][]",
                    Value::string(StringValueFactory::choice(&vec![
                        String::from("a"),
                        String::from("b"),
                    ])),
                ),
            ],
            param: vec![("id", Value::int(IntValueFactory::between(1, 2, 1)))]
                .into_iter()
                .collect(),
        };

        let mut urls = query.into_iter();
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=a&filter[foo][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=a&filter[foo][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=b&filter[foo][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=b&filter[foo][]=a"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=a&filter[foo][]=b"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=a&filter[foo][]=b"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/1?filter[foo][]=b&filter[foo][]=b"
            ))
        );
        assert_eq!(
            urls.next(),
            Some(String::from(
                "http://example.com/2?filter[foo][]=b&filter[foo][]=b"
            ))
        );
        assert_eq!(urls.next(), None);
    }
}
