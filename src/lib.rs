use std::ops::Add;

// 这里使用了特征约束, 限制了泛型参数 T 的类型
// 如下的代码中, T 必须实现 std::ops::Add<Output=T> 这个 trait
// std::ops::Add<Output=T> 是一个 trait, 它定义了加法运算符 + 的行为
pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// PartialOrd 是一个 trait, 用于比较两个值的大小
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

// 快速排序: 直接在原数组上进行排序, 以减少内存开销
pub fn quick_sort<T: PartialOrd>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

// 这里的 where 和上面的 quick_sort<T: PartialOrd> 是等价的
// 当函数签名中有多个泛型参数时, where 语句更加清晰简洁
fn partition<T>(arr: &mut [T]) -> usize
where
    T: PartialOrd,
{
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
