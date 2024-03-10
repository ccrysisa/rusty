use rand::prelude::*;
use sort::{
    bubblesort::BubbleSort, insertionsort::InsertionSort, quicksort::QuickSort,
    selectionsort::SelectionSort, Sorter, StdSorter, StdUnstableSorter,
};
use std::{cell::Cell, rc::Rc};

#[derive(Clone)]
struct SortEvaluator<T> {
    value: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.value.eq(&other.value)
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.value.partial_cmp(&other.value)
    }
}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.value.cmp(&other.value)
    }
}

fn bench<T: Ord + Clone, S: Sorter<SortEvaluator<T>>>(
    sorter: S,
    values: &[SortEvaluator<T>],
    couter: Rc<Cell<usize>>,
) -> (usize, f64) {
    let mut values = values.to_vec();
    couter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    let count = couter.get();

    for i in 1..values.len() {
        assert!(values[i - 1] <= values[i]);
    }

    (count, took.as_secs_f64())
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));

    println!("algorithm n comparisons time");
    for n in [10, 100, 1000, 10000, 50000] {
        let mut values = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                value: rand.gen::<i32>(),
                cmps: Rc::clone(&counter),
            });
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);

            let took = bench(BubbleSort, &mut values, Rc::clone(&counter));
            println!("{} {} {} {}", "bubble", n, took.0, took.1);
            let took = bench(InsertionSort::new(false), &mut values, Rc::clone(&counter));
            println!("{} {} {} {}", "insertion-dump", n, took.0, took.1);
            let took = bench(InsertionSort::new(true), &mut values, Rc::clone(&counter));
            println!("{} {} {} {}", "insertion-smart", n, took.0, took.1);
            let took = bench(SelectionSort, &mut values, Rc::clone(&counter));
            println!("{} {} {} {}", "selection", n, took.0, took.1);
            let took = bench(QuickSort, &mut values, Rc::clone(&counter));
            println!("{} {} {} {}", "quick", n, took.0, took.1);
            let took = bench(StdSorter, &mut values, Rc::clone(&counter));
            println!("{} {} {} {}", "std", n, took.0, took.1);
            let took = bench(StdUnstableSorter, &mut values, Rc::clone(&counter));
            println!("{} {} {} {}", "std-unstable", n, took.0, took.1);
        }
    }
}
