use crate::Sorter;

pub struct InsertionSort {
    naive: bool,
}

impl<T> Sorter<T> for InsertionSort {
    fn sort(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ sorted | unsorted ]
        // We loop through the each item in the slice and insert it in its correct position in the
        // sorted section of the slice.
        //
        // [5 | 4, 3, 2, 1] - the first element in the slice will always be "sorted".
        // [4, 5 | 3, 2, 1] - 1st iter ...
        // [3, 4, 5 | 2, 1] - 2nd iter ...
        for unsorted_x in 1..slice.len() {
            if self.naive {
                // the naive implementation will just loop from the unsorted_x down to 0 while
                // swapping each element with the previous one (the item we are trying to sort)
                // if the next element is bigger then the previous one.
                // [3, 4, 5 | 2, 1] -> (5, 2) - swap since 2 is smaller then 5 -> (4, 2) - swap
                // since 4 is bigger then 2 -> (3, 2) - ... -> i will now be 0 and we will break.

                let mut i = unsorted_x;
                while i > 0 && slice[i] < slice[i-1] {
                    slice.swap(i, i-1);
                    i -= 1;
                }
            } else {
                // knowing that the insertion sort keeps the [ sorted | unsorted ] property true,
                // we can search efficiently in the sorted part of the slice (..unsorted_x) using a
                // binary search and just rotate the slice[i..=unsorted_x] by 1 to the right to
                // have i end up in unsorted_x's position and unsorted_x in i's position whilst
                // transmutating the itermediate values.
                
                let i = slice[..unsorted_x]
                    .binary_search(&slice[unsorted_x])
                    .unwrap_or_else(|i| i);
                // use rotation to swap the i index with unsorted_x index efficiently without
                // swapping all the indexes between since they are already in order.
                //
                // Example:
                // [2, 4, 5 | 3, 1] -> index of 3 in [2, 4, 5] = 1. -> slice[1..=3] = [4, 5, 3]
                // we rotate this slice to the right by one, by the property of rotations the 3
                // will wrap around to the left: [4, 5, 3] -> [3, 4, 5].
                // Now we look at the bigger slice: [2, [3, 4, 5] | 1].
                // We can safely do this rotation because the numbers are in [i..unsorted_x] are
                // already sorted.
                slice[i..=unsorted_x].rotate_right(1);
            }
        }
    }
}

#[test]
fn sorts_naive() {
    let mut slice = [5, 4, 3, 2, 1];
    InsertionSort{ naive: true }.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 4, 5]);
}

#[test]
fn sorts_smart() {
    let mut slice = [5, 4, 3, 2, 1];
    InsertionSort{ naive: false }.sort(&mut slice);
    assert_eq!(slice, [1, 2, 3, 4, 5]);
}
