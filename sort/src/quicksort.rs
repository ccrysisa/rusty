use super::Sorter;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    // [ <= pivot ] [left] [ ... ] [right] [ > pivot ]
    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0; // current checking
    let mut right = rest.len() - 1; // current checking
    while left <= right {
        if &rest[left] <= pivot {
            // left already on the correct side
            left += 1;
        } else if &rest[right] > pivot {
            // right already on the correct side
            // avoid unnecessary swaps back and forth
            if right == 0 {
                // we must be done
                break;
            }
            right -= 1;
        } else {
            // left holds a right, and right holds a left, swap them.
            rest.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    // left points to the first `> pivot` element, and left - 1 points to the last `<= pivot` element
    // re-align left to account for the pivot at 0
    let left = left + 1;

    // place the pivot at its final location
    if 0 != left - 1 {
        slice.swap(0, left - 1);
    }

    // split_at_mut(mid: usize) -> (&mut [..mid), &mut [mid..])
    let (left, right) = slice.split_at_mut(left - 1);
    quicksort(left);
    quicksort(&mut right[1..]);
}

pub struct QuickSort;

impl<T> Sorter<T> for QuickSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        quicksort(slice);
    }
}

#[test]
fn it_works() {
    let mut things = vec![3, 2, 1, 4];
    QuickSort.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4]);
}
