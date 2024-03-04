use super::Sorter;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubblesort_works() {
    use super::sort;
    let mut things = vec![3, 2, 1, 4];
    sort::<_, BubbleSort>(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4]);
}
