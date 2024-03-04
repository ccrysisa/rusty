pub fn flat_map<I, U, F>(iter: I, f: F) -> FlatMap<I::IntoIter, U, F>
where
    I: IntoIterator,
    U: IntoIterator,
    F: FnMut(I::Item) -> U,
{
    FlatMap::new(iter.into_iter(), f)
}

pub struct FlatMap<O, I, F>
where
    O: Iterator,
    I: IntoIterator,
    F: FnMut(O::Item) -> I,
{
    outer: O,
    func: F,
    front_iter: Option<I::IntoIter>,
    back_iter: Option<I::IntoIter>,
}

impl<O, I, F> FlatMap<O, I, F>
where
    O: Iterator,
    I: IntoIterator,
    F: FnMut(O::Item) -> I,
{
    pub fn new(iter: O, f: F) -> Self {
        Self {
            outer: iter,
            func: f,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O, I, F> Iterator for FlatMap<O, I, F>
where
    O: Iterator,
    I: IntoIterator,
    F: FnMut(O::Item) -> I,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                if let Some(i) = front_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }

            if let Some(next_front_iter) = self.outer.next() {
                self.front_iter = Some((self.func)(next_front_iter).into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O, I, F> DoubleEndedIterator for FlatMap<O, I, F>
where
    O: DoubleEndedIterator,
    I: IntoIterator,
    I::IntoIter: DoubleEndedIterator,
    F: FnMut(O::Item) -> I,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }

            if let Some(next_back_iter) = self.outer.next_back() {
                self.back_iter = Some((self.func)(next_back_iter).into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            flat_map(std::iter::empty::<Vec<()>>(), |_| std::iter::empty::<()>()).count(),
            0
        );
    }

    #[test]
    fn empty_wide() {
        assert_eq!(
            flat_map(vec![1, 2, 3].into_iter(), |_| std::iter::empty::<()>()).count(),
            0
        );
    }

    #[test]
    fn one() {
        assert_eq!(flat_map(std::iter::once(vec!["a"]), |x| x).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flat_map(std::iter::once(vec!["a", "b"]), |x| x).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flat_map(vec![vec!["a"], vec!["b"]], |x| x).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flat_map(std::iter::once(vec!["a", "b"]), |x| x)
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flat_map(vec![vec!["a"], vec!["b"]], |x| x)
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flat_map(vec![vec!["a1", "a2", "a3"], vec!["b1", "b2", "b3"]], |x| x);
        assert_eq!(iter.next(), Some("a1"));
        assert_eq!(iter.next_back(), Some("b3"));
        assert_eq!(iter.next(), Some("a2"));
        assert_eq!(iter.next_back(), Some("b2"));
        assert_eq!(iter.next(), Some("a3"));
        assert_eq!(iter.next_back(), Some("b1"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn infinite() {
        let mut iter = flat_map(0.., |i| 0..i);
        // 0 => 0..0 => empty
        // 1 => 0..1 => [0]
        // 2 => 0..2 => [0, 1]
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }
}
