pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

pub fn qs(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        return arr;
    }
    let pivot = arr[0];
    let (low, high) = arr[1..].iter().partition(|&&x| x < pivot);
    return [qs(low), vec![pivot], qs(high)].concat();
}
