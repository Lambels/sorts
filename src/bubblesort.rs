use crate::Sorter;

pub struct BubbleSort;

impl<T> Sorter<T> for BubbleSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // loop through the entire array looking at each pair of indexes (i, i+1) and swap(i, i+1)
        // where i > i+1. Once the swap was achieved i is effectively "bubbled" up in the place of
        // i+1 where it will next be compared to i+2 untill it potentially reaches the top of the
        // slice or is "bubbled up".
        //
        // The bubble sort starts by assuming that the slice is sorted and at the first instance of
        // i > i + 1 sets the sorted flag to false since the slice wasn't sorted as we expected. It
        // then loops and continues comparisons while the sorted flag is false or the slice isnt
        // sorted.

        let mut sorted = false;
        while !sorted {
            sorted = true;
            // start from 1 since we cant compare the last element
            // (slice.len()-1) with anything since we are comparing downwards.
            //
            // Same behaviour could be achieved by 0..slice.len()-1 and comparing upwards:
            // slice[i] > slice[i+1] and swap(i, i+1)
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    sorted = false;
                }
            }
        }
    }
}

#[test]
fn sorts() {
    let mut slice = [5, 4, 3, 2, 1];
    BubbleSort.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 4, 5]);
}
