use ndarray::{Array1, Array2, Axis};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

// --- Layer Normalization ---

pub struct LayerNorm {
    gamma: Array1<f32>,
    beta: Array1<f32>,
    epsilon: f32,
}

impl LayerNorm {
    pub fn new(d_model: usize) -> Self {
        Self {
            gamma: Array1::ones(d_model),
            beta: Array1::zeros(d_model),
            epsilon: 1e-5,
        }
    }

    pub fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        // 沿特征维度计算均值和方差
        let mean = x.mean_axis(Axis(1)).unwrap().insert_axis(Axis(1));
        let var = x.var_axis(Axis(1), 0.0).insert_axis(Axis(1));
        let inv_std = (var + self.epsilon).mapv(f32::sqrt).mapv(|v| 1.0 / v);

        // 标准化
        let x_norm = (x - &mean) * &inv_std;

        // 应用缩放 (gamma) 和平移 (beta)
        let gamma = self.gamma.view().insert_axis(Axis(0)); // Shape [1, d_model]
        let beta = self.beta.view().insert_axis(Axis(0)); // Shape [1, d_model]

        &x_norm * &gamma + &beta
    }
}

// --- Position-wise Feed-Forward Network ---

pub struct FeedForward {
    w1: Array2<f32>,
    b1: Array2<f32>,
    w2: Array2<f32>,
    b2: Array2<f32>,
}

impl FeedForward {
    pub fn new(d_model: usize, d_ff: usize) -> Self {
        let scale1 = (2.0 / (d_model + d_ff) as f32).sqrt();
        let scale2 = (2.0 / (d_ff + d_model) as f32).sqrt();

        Self {
            w1: Array2::random((d_model, d_ff), Uniform::new(-scale1, scale1)),
            b1: Array2::zeros((1, d_ff)),
            w2: Array2::random((d_ff, d_model), Uniform::new(-scale2, scale2)),
            b2: Array2::zeros((1, d_model)),
        }
    }

    pub fn forward(&self, x: &Array2<f32>) -> Array2<f32> {
        let mut hidden = x.dot(&self.w1) + &self.b1;
        hidden.mapv_inplace(|val| val.max(0.0)); // ReLU
        hidden.dot(&self.w2) + &self.b2
    }
}

// --- Tests ---

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_layer_norm_shape_and_mean_std() {
        let d_model = 4;
        let layer_norm = LayerNorm::new(d_model);
        let input =
            Array2::from_shape_vec((2, d_model), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
                .unwrap();

        let output = layer_norm.forward(&input);

        // Shape should be preserved
        assert_eq!(input.shape(), output.shape());

        // Mean of each output row should be close to 0
        for row in output.axis_iter(Axis(0)) {
            assert!(row.mean().unwrap().abs() < 1e-6);
        }

        // Standard deviation of each output row should be close to 1
        for row in output.axis_iter(Axis(0)) {
            assert!((row.std(0.0) - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn test_feed_forward_shape() {
        let d_model = 8;
        let d_ff = 32;
        let ff = FeedForward::new(d_model, d_ff);
        let input = Array2::random((10, d_model), Uniform::new(-1.0, 1.0));

        let output = ff.forward(&input);

        // Shape should be preserved
        assert_eq!(output.shape(), &[10, d_model]);
    }
}
