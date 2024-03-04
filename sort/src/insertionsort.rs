use super::Sorter;

pub struct InsertionSort;

impl<T> Sorter<T> for InsertionSort {
    fn sort(slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location in slice[..=unsorted]
            // [ 1 3 4 | 2 ]
            // [ 1 3 4 2 | ]
            // [ 1 3 2 4 | ]
            // [ 1 2 3 4 | ]
            let mut i = unsorted;
            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[test]
fn insertionsort_works() {
    use super::sort;
    let mut things = vec![3, 2, 1, 4];
    sort::<_, InsertionSort>(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4]);
}
