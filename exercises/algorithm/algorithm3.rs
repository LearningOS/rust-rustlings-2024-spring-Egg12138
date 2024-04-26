/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::cmp::{PartialEq, PartialOrd};
use std::fmt::Debug;

fn bubble<T: PartialOrd + Debug>(array: &mut [T]) {
    for i in 0..array.len() {
        for j in 0..array.len() - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

fn partition<T: PartialOrd + Debug>(array: &mut [T], left: usize, right: usize) -> usize {
    let (mut l, mut r) = (left, right);
    let pivot = l;
    while l < r {
        while l < r && array[r] >= array[pivot] {
            r -= 1;
        }
        while l < r && array[l] <= array[pivot] {
            l += 1;
        }
        if l < r {
            array.swap(l, r);
        }
    }
    array.swap(pivot, l);
    l
}

fn quick_sort<T: PartialOrd + Debug>(array: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot = partition(array, left, right);
        if pivot != 0 {
            quick_sort(array, left, pivot - 1);
        }
        quick_sort(array, pivot + 1, right);
    }
}
fn qsort<T: PartialOrd + Debug>(array: &mut [T]) {
    quick_sort(array, 0, array.len() - 1);
    dbg!(array);
}

fn sort<T: PartialOrd + Debug>(array: &mut [T]) {
    //TODO
    if array.len() <= 1 {
        return;
    }
    // bubble(array);
    qsort(array);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
