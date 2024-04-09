use learning_rs::{add, qs, quick_sort};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![1, 3, 2, 5, 4];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_qs() {
        let arr = vec![1, 3, 2, 5, 4];
        assert_eq!(qs(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_add_i32() {
        assert_eq!(add(1, 2), 3)
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(add(1.1, 2.5), 3.6)
    }
}
