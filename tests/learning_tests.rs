#[cfg(test)]
mod tests {
    use learning_rs::modules::algorithms::fft::fft;
    use learning_rs::modules::algorithms::general::{add, largest};
    use learning_rs::modules::algorithms::sorting::{qs, quick_sort};
    use num_complex::Complex;

    #[test]
    fn test_add_i32() {
        assert_eq!(add(1, 2), 3)
    }

    #[test]
    fn test_add_f64() {
        assert_eq!(add(1.1, 2.5), 3.6)
    }

    #[test]
    fn test_largest() {
        let arr = vec![1, 3, 2, 5, 4];
        assert_eq!(largest(&arr), &5);
    }

    #[test]
    fn test_largest_using_max() {
        let arr = vec![1, 3, 2, 5, 4];
        assert_eq!(arr.iter().max(), Some(&5));
    }

    #[test]
    fn test_qs() {
        let arr = vec![1, 3, 2, 5, 4];
        assert_eq!(qs(arr), vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_with_num() {
        let mut arr = vec![1, 3, 2, 5, 4];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_quick_sort_with_char() {
        let mut arr = vec!['a', 'c', 'b', 'e', 'd'];
        quick_sort(&mut arr);
        assert_eq!(arr, vec!['a', 'b', 'c', 'd', 'e']);
    }

    #[test]
    fn test_fft_ifft() {
        // 定义测试数据
        let mut data = vec![
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];

        // 保存原始数据用于比较
        let original_data = data.clone();

        // 执行正向 FFT
        fft(&mut data, false);
        // 执行反向 FFT
        fft(&mut data, true);

        // 比较反向 FFT 的结果与原始数据
        for i in 0..data.len() {
            // 确保误差在容许范围内
            assert!((data[i] - original_data[i]).norm() < 1e-10);
        }
    }
}