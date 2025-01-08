use super::Sorter;

pub struct MaxHeap<'a, T> {
    slice: &'a mut [T],
    count: usize,
}

impl<'a, T> MaxHeap<'a, T> {
    pub fn new(slice: &'a mut [T]) -> Self {
        let count = slice.len();
        Self { slice, count }
    }

    pub fn max_heapify(&mut self, i: usize)
    where
        T: Ord,
    {
        let left = self.left(i).map_or(i, |j| j);
        let right = self.right(i).map_or(i, |j| j);

        let largest = if self.slice[left] > self.slice[i] {
            left
        } else {
            i
        };
        let largest = if self.slice[right] > self.slice[largest] {
            right
        } else {
            largest
        };

        if largest == i {
            return;
        } else {
            self.slice.swap(i, largest);
            self.max_heapify(largest);
        }
    }

    pub fn build(&mut self)
    where
        T: Ord,
    {
        for i in (0..=Self::parent(self.count - 1)).rev() {
            self.max_heapify(i);
        }
    }

    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn left(&self, i: usize) -> Option<usize> {
        let result = (i + 1) * 2 - 1;
        if result >= self.count {
            None
        } else {
            Some(result)
        }
    }

    fn right(&self, i: usize) -> Option<usize> {
        let result = (i + 1) * 2;

        if result >= self.count {
            None
        } else {
            Some(result)
        }
    }
}

pub struct HeapSort;

impl<T> Sorter<T> for HeapSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut heap = MaxHeap::new(slice);
        heap.build();

        while heap.count > 0 {
            heap.count -= 1;

            heap.slice.swap(0, heap.count);
            heap.max_heapify(0);
        }
    }
}

#[test]
fn it_works() {
    let mut things = vec![3, 2, 1, 4];
    HeapSort.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4]);
}
