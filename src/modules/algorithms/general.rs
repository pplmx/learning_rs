use std::ops::Add;

pub fn add<T: Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
