use std::iter::Cycle;

#[derive(Debug, Clone)]
pub struct Cycler<T: Iterator> {
    inner: T,
    count: usize,
    current: Option<T::Item>,
    countup_by: usize,
}

impl<T: Iterator> Cycler<T> {
    pub fn new(inner: T, countup_by: usize) -> Self {
        Cycler {
            inner,
            count: 0,
            current: None,
            countup_by,
        }
    }
}

impl<T: Iterator> Iterator for Cycler<T>
where
    T::Item: Clone,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count % self.countup_by == 0 {
            self.current = self.inner.next()
        }
        self.count += 1;
        self.current.clone()
    }
}

pub struct Counter<T: Iterator + Clone> {
    cyclers: Vec<Cycle<Cycler<T>>>,
    last: Cycler<T>,
}

impl<T: Iterator + Clone> Counter<T>
where
    T::Item: Clone,
{
    pub fn new(mut vec: Vec<T>) -> Self {
        let last = vec.pop().unwrap();
        let mut countup_bys = vec
            .clone()
            .into_iter()
            .map(|x| x.count())
            .fold((1, vec![]), |a, b| {
                (a.0 * b, vec![a.1, vec![a.0 * b]].concat())
            })
            .1;
        countup_bys.insert(0, 1);
        let countup_last = countup_bys.pop().unwrap();
        let mut cyclers = vec![];
        for x in countup_bys.iter().enumerate() {
            cyclers.push(Cycler::new(vec.get(x.0).unwrap().clone(), *x.1).cycle());
        }
        let last = Cycler::new(last, countup_last);
        Self { cyclers, last }
    }
}

impl<T: Iterator + Clone> Iterator for Counter<T>
where
    T::Item: Clone,
{
    type Item = Vec<T::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut values = vec![];
        for v in self.cyclers.iter_mut() {
            values.push(v.next().unwrap())
        }
        let end = self.last.next();
        if end.is_none() {
            return None;
        } else {
            values.push(end.unwrap());
            return Some(values);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Counter;

    #[test]
    fn counter() {
        let mut counter = Counter::new(vec![
            vec![0, 1, 2].into_iter(),
            vec![0, 1].into_iter(),
            vec![0].into_iter(),
        ])
        .into_iter();
        assert_eq!(counter.next(), Some(vec![0, 0, 0]));
        assert_eq!(counter.next(), Some(vec![1, 0, 0]));
        assert_eq!(counter.next(), Some(vec![2, 0, 0]));
        assert_eq!(counter.next(), Some(vec![0, 1, 0]));
        assert_eq!(counter.next(), Some(vec![1, 1, 0]));
        assert_eq!(counter.next(), Some(vec![2, 1, 0]));
        assert_eq!(counter.next(), None);
    }
}
