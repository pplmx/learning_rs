// std::ops::Add<Output=T> is a trait that allows the + operator to be used on values of the same type.
pub fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// PartialOrd is a trait that allows comparison between values of the same type.
pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn qs(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
        return arr;
    }
    let pivot = arr[0];
    let (low, high) = arr[1..].iter().partition(|&&x| x < pivot);
    return [qs(low), vec![pivot], qs(high)].concat();
}

// Directly change the input array to reduce memory usage
pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: PartialOrd>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;
    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}
