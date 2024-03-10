use super::Sorter;

pub struct SelectionSort;

impl<T> Sorter<T> for SelectionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | not sorted ]
        for unsorted in 0..slice.len() {
            // functional programmming
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .map(|(i, _)| unsorted + i)
                .expect("slice is not empty");

            // directly loop
            // let mut smallest_in_rest = unsorted;
            // for i in (unsorted + 1)..slice.len() {
            //     if slice[i] < slice[unsorted] {
            //         smallest_in_rest = i;
            //     }
            // }

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![3, 2, 1, 4];
    SelectionSort.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4]);
}
