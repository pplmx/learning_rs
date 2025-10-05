use crate::modules::llm::attn::MultiHeadAttention;
use crate::modules::llm::core::{FeedForward, LayerNorm};
use ndarray::Array2;

pub struct TransformerBlock {
    attn: MultiHeadAttention,
    feed_forward: FeedForward,
    norm1: LayerNorm,
    norm2: LayerNorm,
}

impl TransformerBlock {
    pub fn new(d_model: usize, num_heads: usize, d_ff: usize) -> Self {
        Self {
            attn: MultiHeadAttention::new(d_model, num_heads),
            feed_forward: FeedForward::new(d_model, d_ff),
            norm1: LayerNorm::new(d_model),
            norm2: LayerNorm::new(d_model),
        }
    }

    pub fn forward(&self, x: &Array2<f32>, mask: Option<&Array2<f32>>) -> Array2<f32> {
        // 1. Multi-Head Attention with residual connection and layer norm
        let attn_output = self.attn.forward(x, mask);
        let sublayer1_output = self.norm1.forward(&(x + attn_output));

        // 2. Feed-Forward with residual connection and layer norm
        let ff_output = self.feed_forward.forward(&sublayer1_output);
        self.norm2.forward(&(sublayer1_output + ff_output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray_rand::RandomExt;
    use ndarray_rand::rand_distr::Uniform;

    #[test]
    fn test_transformer_block_shape() {
        let d_model = 16;
        let num_heads = 4;
        let d_ff = 64;
        let seq_len = 10;

        let block = TransformerBlock::new(d_model, num_heads, d_ff);
        let input = Array2::random((seq_len, d_model), Uniform::new(-1.0, 1.0));

        let output = block.forward(&input, None); // Pass None for mask

        // The output shape should be the same as the input shape
        assert_eq!(output.shape(), &[seq_len, d_model]);
    }
}
