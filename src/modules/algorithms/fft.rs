use num_complex::Complex;
use std::f64::consts::PI;

pub fn fft(input: &mut [Complex<f64>], inverse: bool) {
    let n = input.len();
    if n <= 1 {
        return;
    }

    let mut even = input.iter().step_by(2).cloned().collect::<Vec<_>>();
    let mut odd = input.iter().skip(1).step_by(2).cloned().collect::<Vec<_>>();

    fft(&mut even, inverse);
    fft(&mut odd, inverse);

    let sign = if inverse { 2.0 } else { -2.0 };

    for i in 0..n / 2 {
        let t = Complex::from_polar(1.0, sign * PI * i as f64 / n as f64) * odd[i];
        input[i] = even[i] + t;
        input[i + n / 2] = even[i] - t;
    }

    if inverse {
        for x in input.iter_mut() {
            *x = *x / 2.0;
        }
    }
}
