pub mod bubblesort;
pub mod heapsort;
pub mod insertionsort;
pub mod mergesort;
pub mod quicksort;
pub mod selectionsort;

pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

pub struct StdSorter;

impl<T> Sorter<T> for StdSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort();
    }
}

pub struct StdUnstableSorter;

impl<T> Sorter<T> for StdUnstableSorter {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort_unstable();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_works() {
        let mut things = vec![3, 2, 1, 4];
        StdSorter.sort(&mut things);
        assert_eq!(things, vec![1, 2, 3, 4]);
    }
}
