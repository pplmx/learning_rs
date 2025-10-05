use ndarray::{Array2, Axis};
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

/// Self-Attention 层
#[allow(dead_code)]
pub struct SelfAttention {
    d_model: usize,
    d_k: usize,
    d_v: usize,
    w_q: Array2<f32>,
    w_k: Array2<f32>,
    w_v: Array2<f32>,
}

impl SelfAttention {
    /// 创建新的 Self-Attention 层
    ///
    /// # 参数
    /// * `d_model` - 输入维度
    /// * `d_k` - Key/Query 的维度
    /// * `d_v` - Value 的维度
    pub fn new(d_model: usize, d_k: usize, d_v: usize) -> Self {
        // 使用 Xavier/Glorot 初始化
        let qk_scale = (2.0 / (d_model + d_k) as f32).sqrt();
        let v_scale = (2.0 / (d_model + d_v) as f32).sqrt();

        let w_q = Array2::random((d_model, d_k), Uniform::new(-qk_scale, qk_scale));
        let w_k = Array2::random((d_model, d_k), Uniform::new(-qk_scale, qk_scale));
        let w_v = Array2::random((d_model, d_v), Uniform::new(-v_scale, v_scale));

        Self {
            d_model,
            d_k,
            d_v,
            w_q,
            w_k,
            w_v,
        }
    }

    /// 前向传播
    ///
    /// # 参数
    /// * `x` - 输入矩阵 (seq_len, d_model)
    ///
    /// # 返回
    /// (输出矩阵 (seq_len, d_v), 注意力权重 (seq_len, seq_len))
    pub fn forward(
        &self,
        x: &Array2<f32>,
        mask: Option<&Array2<f32>>,
    ) -> (Array2<f32>, Array2<f32>) {
        // 计算 Q, K, V
        let q = x.dot(&self.w_q); // (seq_len, d_k)
        let k = x.dot(&self.w_k); // (seq_len, d_k)
        let v = x.dot(&self.w_v); // (seq_len, d_v)

        // 计算注意力分数: Q @ K^T / sqrt(d_k)
        let mut scores = q.dot(&k.t()) / (self.d_k as f32).sqrt();

        // 应用掩码 (如果提供)
        if let Some(m) = mask {
            scores += m;
        }

        // Softmax (沿最后一个维度)
        let attention_weights = Self::softmax(&scores);

        // 应用注意力权重到 V
        (attention_weights.dot(&v), attention_weights)
    }

    /// Softmax 函数 (沿行方向)
    fn softmax(x: &Array2<f32>) -> Array2<f32> {
        let mut result = x.clone();

        // 对每一行应用 softmax
        for mut row in result.axis_iter_mut(Axis(0)) {
            // 数值稳定性: 减去最大值
            let max = row.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
            row.mapv_inplace(|v| (v - max).exp());

            // 归一化
            let sum: f32 = row.sum();
            row /= sum;
        }

        result
    }

    /// 获取参数数量
    pub fn num_parameters(&self) -> usize {
        self.w_q.len() + self.w_k.len() + self.w_v.len()
    }
}

/// 构建器模式
pub struct SelfAttentionBuilder {
    d_model: Option<usize>,
    d_k: Option<usize>,
    d_v: Option<usize>,
}

impl SelfAttentionBuilder {
    pub fn new() -> Self {
        Self {
            d_model: None,
            d_k: None,
            d_v: None,
        }
    }

    pub fn d_model(mut self, d_model: usize) -> Self {
        self.d_model = Some(d_model);
        self
    }

    pub fn d_k(mut self, d_k: usize) -> Self {
        self.d_k = Some(d_k);
        self
    }

    pub fn d_v(mut self, d_v: usize) -> Self {
        self.d_v = Some(d_v);
        self
    }

    pub fn build(self) -> Result<SelfAttention, &'static str> {
        let d_model = self.d_model.ok_or("必须指定 d_model")?;
        let d_k = self.d_k.unwrap_or(d_model);
        let d_v = self.d_v.unwrap_or(d_model);
        Ok(SelfAttention::new(d_model, d_k, d_v))
    }
}

impl Default for SelfAttentionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Multi-Head Attention (额外功能)
#[allow(dead_code)]
pub struct MultiHeadAttention {
    heads: Vec<SelfAttention>,
    w_o: Array2<f32>,
    num_heads: usize,
    d_model: usize,
}

impl MultiHeadAttention {
    pub fn new(d_model: usize, num_heads: usize) -> Self {
        assert_eq!(d_model % num_heads, 0, "d_model 必须能被 num_heads 整除");

        let d_k = d_model / num_heads;
        let d_v = d_model / num_heads;

        let heads = (0..num_heads)
            .map(|_| SelfAttention::new(d_model, d_k, d_v))
            .collect();

        // w_o 的输入维度是 num_heads * d_v = d_model
        let scale = (2.0 / (d_model + d_model) as f32).sqrt();
        let w_o = Array2::random((d_model, d_model), Uniform::new(-scale, scale));

        Self {
            heads,
            w_o,
            num_heads,
            d_model,
        }
    }

    pub fn forward(&self, x: &Array2<f32>, mask: Option<&Array2<f32>>) -> Array2<f32> {
        // 并行计算所有头
        let head_outputs: Vec<Array2<f32>> = self
            .heads
            .iter()
            .map(|head| head.forward(x, mask).0) // .0 获取最终输出, 忽略权重
            .collect();

        // 拼接所有头的输出 (num_heads * d_v -> d_model)
        let concatenated = ndarray::concatenate(
            Axis(1),
            &head_outputs.iter().map(|a| a.view()).collect::<Vec<_>>(),
        )
        .unwrap();

        // 最后的线性变换
        concatenated.dot(&self.w_o)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{Array2, array};
    use ndarray_rand::rand_distr::Uniform;

    #[test]
    fn test_self_attention_shape() {
        let x = Array2::random((3, 4), Uniform::new(0.0, 1.0));
        let attention = SelfAttention::new(4, 4, 5);
        // Update test to handle new return type and no mask
        let (output, _attn_weights) = attention.forward(&x, None);

        assert_eq!(output.shape(), &[3, 5]);
    }

    #[test]
    fn test_builder_pattern() {
        let attention = SelfAttentionBuilder::new()
            .d_model(8)
            .d_k(4)
            .d_v(6)
            .build()
            .unwrap();

        assert_eq!(attention.d_model, 8);
        assert_eq!(attention.d_k, 4);
        assert_eq!(attention.d_v, 6);
    }

    #[test]
    fn test_softmax() {
        let x = array![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]];
        let result = SelfAttention::softmax(&x);

        // 每行应该和为 1
        for row in result.axis_iter(Axis(0)) {
            let sum: f32 = row.sum();
            assert!((sum - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn test_multi_head_attention() {
        let x = Array2::random((3, 8), Uniform::new(0.0, 1.0));
        let mha = MultiHeadAttention::new(8, 4);
        let output = mha.forward(&x, None); // Pass None for the mask

        assert_eq!(output.shape(), &[3, 8]);
    }

    #[test]
    fn test_self_attention_with_mask() {
        let seq_len = 3;
        let d_model = 4;
        let attention = SelfAttention::new(d_model, d_model, d_model);
        let x = Array2::random((seq_len, d_model), Uniform::new(0.0, 1.0));

        // Create a look-ahead mask
        let mut mask = Array2::zeros((seq_len, seq_len));
        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                mask[[i, j]] = -1e9;
            }
        }

        // This call will fail to compile initially
        let (_output, attn_weights) = attention.forward(&x, Some(&mask));

        // The weights for masked positions should be close to 0
        assert!(attn_weights[[0, 1]] < 1e-6);
        assert!(attn_weights[[0, 2]] < 1e-6);
        assert!(attn_weights[[1, 2]] < 1e-6);

        // The weights for unmasked positions should be > 0
        assert!(attn_weights[[0, 0]] > 0.0);
        assert!(attn_weights[[1, 0]] > 0.0);
        assert!(attn_weights[[1, 1]] > 0.0);
    }
}
