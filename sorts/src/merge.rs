use std::fmt::Debug;
#[allow(dead_code)]
pub fn merge_sort<T>(array: &mut Vec<T>) -> Vec<T>
    where T: Debug + PartialOrd + PartialEq + Clone {
    if array.len() > 1 {
        let high = array.len() - 1;
        let low = 0 as usize;
        merge_sort_slice(array, low, high);
        array.clone()
    } else {
        array.clone()
    }
}
#[allow(dead_code)]
fn merge_sort_slice<T>(vec: &mut Vec<T>, low: usize, high: usize)
    where T: Debug + PartialOrd + PartialEq + Clone {
    if low < high {
        let middle = (high + low) / 2;
        merge_sort_slice(vec, low, middle + 1);
        merge_sort_slice(vec, middle, high);
        merge(vec, low, middle, high);
    }
}
#[allow(dead_code)]
fn merge<T>(vec: &mut Vec<T>, low: usize, m: usize, high: usize)
    where T: Debug + PartialOrd + PartialEq + Clone {
    let _left = vec[low..m + 1].to_vec();
    let _right = vec[m + 1..high + 1].to_vec();
}