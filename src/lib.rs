pub trait Sorter<T> {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord;
}

mod bubblesort;
mod insertionsort;
mod selectionsort;

pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
