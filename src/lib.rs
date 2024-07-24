extern crate num_complex;
use std::f64::consts::PI;
use std::ops::Add;

use num_complex::Complex;

// 这里使用了特征约束, 限制了泛型参数 T 的类型
// 如下的代码中, T 必须实现 std::ops::Add<Output=T> 这个 trait
// std::ops::Add<Output=T> 是一个 trait, 它定义了加法运算符 + 的行为
// 也就是说, T 类型的值可以使用 + 运算符进行相加
// 例如, i32 类型的值可以相加, f32 类型的值也可以相加
// 但是, i32 类型的值和 f32 类型的值不能相加
// 所以, T 必须实现 Add<Output=T> 这个 trait, 以保证泛型参数 T 的值可以相加
// 这样, add 函数就可以接受任意类型的参数, 只要这个类型实现了 Add<Output=T> 这个 trait
// 这就是泛型编程的优势, 可以编写出更加通用的代码
/**
[`add`] 这里是一个函数跳转链接
```
// DocTest
use learning_rs::add;

let a = 1;
let b = 2;
assert_eq!(add(a, b), 3);
```
 */
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

/// 通用的 FFT 函数
///
/// # 参数
///
/// - `input`: 待变换的复数数组
/// - `inverse`: 标志是否进行反向变换 (true 表示 IFFT, false 表示 FFT)
pub fn fft(input: &mut [Complex<f64>], inverse: bool) {
    let n = input.len();
    if n <= 1 {
        return;
    }

    // 分离为偶数和奇数下标的元素
    let mut even = input.iter().step_by(2).cloned().collect::<Vec<_>>();
    let mut odd = input.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>();

    // 递归计算 FFT
    fft(&mut even, inverse);
    fft(&mut odd, inverse);

    // 根据是否是反向变换决定指数的符号
    let sign = if inverse { 2.0 } else { -2.0 };

    for i in 0..n / 2 {
        // 计算复数的旋转因子
        let t = Complex::from_polar(1.0, sign * PI * i as f64 / n as f64) * odd[i];
        // 合并偶数和奇数部分的结果
        input[i] = even[i] + t;
        input[i + n / 2] = even[i] - t;
    }

    // 如果是反向变换，结果需要除以 2
    if inverse {
        for x in input.iter_mut() {
            *x = *x / 2.0;
        }
    }
}
