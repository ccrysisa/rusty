use super::Sorter;

pub struct Mergesort;

fn merge<T: Ord + Clone>(
    slice: &mut [T],
    l1: usize,
    r1: usize,
    l2: usize,
    r2: usize,
    buf: &mut [T],
) {
    let mut i = l1;
    let mut j = l1;
    let mut k = l2;

    while j <= r1 && k <= r2 {
        if slice[j] < slice[k] {
            buf[i] = slice[j].clone();
            j += 1;
        } else {
            buf[i] = slice[k].clone();
            k += 1;
        }
        i += 1;
    }

    while j <= r1 {
        buf[i] = slice[j].clone();
        i += 1;
        j += 1;
    }
    while k <= r2 {
        buf[i] = slice[k].clone();
        i += 1;
        k += 1;
    }

    slice[l1..=r2].clone_from_slice(&buf[l1..=r2]);
}

fn mergesort<T: Ord + Clone>(slice: &mut [T], left: usize, right: usize, buf: &mut [T]) {
    if left >= right {
        return;
    }

    let mid = (left + right) / 2;
    mergesort(slice, left, mid, buf);
    mergesort(slice, mid + 1, right, buf);

    merge(slice, left, mid, mid + 1, right, buf);
}

impl<T> Sorter<T> for Mergesort
where
    T: Clone,
{
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut buf: Vec<T> = Vec::with_capacity(slice.len());
        buf.resize(slice.len(), slice[0].clone());
        mergesort(slice, 0, slice.len() - 1, &mut buf);
    }
}

#[test]
fn it_works() {
    let mut things = vec![3, 2, 1, 4];
    Mergesort.sort(&mut things);
    assert_eq!(things, vec![1, 2, 3, 4]);
}
