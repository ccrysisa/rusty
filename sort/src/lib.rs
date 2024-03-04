pub mod bubblesort;

pub trait Sorter<T> {
    fn sort(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter<T>,
{
    S::sort(slice);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl<T> Sorter<T> for StdSorter {
        fn sort(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort();
        }
    }

    #[test]
    fn std_works() {
        let mut things = vec![3, 2, 1, 4];
        sort::<_, StdSorter>(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4]);
    }
}
