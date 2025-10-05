use crate::modules::llm::embedding::{PositionalEncoding, TokenEmbedding};
use crate::modules::llm::transformer::TransformerBlock;
use ndarray::Array2;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;

pub struct LanguageModel {
    token_embedding: TokenEmbedding,
    positional_encoding: PositionalEncoding,
    transformer_blocks: Vec<TransformerBlock>,
    // The output layer is a linear transformation, represented by a weight matrix.
    // It maps the d_model dimension back to the vocab_size.
    output_layer: Array2<f32>,
}

impl LanguageModel {
    pub fn new(
        vocab_size: usize,
        d_model: usize,
        max_seq_len: usize,
        num_blocks: usize,
        num_heads: usize,
        d_ff: usize,
    ) -> Self {
        let transformer_blocks = (0..num_blocks)
            .map(|_| TransformerBlock::new(d_model, num_heads, d_ff))
            .collect();

        // The output layer maps from d_model to vocab_size
        let output_layer = Array2::random((d_model, vocab_size), Uniform::new(-0.1, 0.1));

        Self {
            token_embedding: TokenEmbedding::new(vocab_size, d_model),
            positional_encoding: PositionalEncoding::new(max_seq_len, d_model),
            transformer_blocks,
            output_layer,
        }
    }

    /// Generates a causal mask to prevent attention to future tokens.
    fn create_causal_mask(seq_len: usize) -> Array2<f32> {
        let mut mask = Array2::zeros((seq_len, seq_len));
        for i in 0..seq_len {
            for j in (i + 1)..seq_len {
                mask[[i, j]] = -1e9; // Large negative number
            }
        }
        mask
    }

    pub fn forward(&self, token_ids: &[usize]) -> Array2<f32> {
        let seq_len = token_ids.len();

        // 1. Create causal mask
        let mask = Self::create_causal_mask(seq_len);

        // 2. Get token embeddings
        let mut x = self.token_embedding.forward(token_ids);

        // 3. Add positional encodings
        x = self.positional_encoding.forward(&x);

        // 4. Pass through all transformer blocks
        for block in &self.transformer_blocks {
            x = block.forward(&x, Some(&mask));
        }

        // 5. Final linear layer to get logits
        x.dot(&self.output_layer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_model_shape() {
        let vocab_size = 100;
        let d_model = 16;
        let max_seq_len = 50;
        let num_blocks = 2;
        let num_heads = 4;
        let d_ff = 32;
        let seq_len = 10;

        let model = LanguageModel::new(
            vocab_size,
            d_model,
            max_seq_len,
            num_blocks,
            num_heads,
            d_ff,
        );

        let tokens = (0..seq_len).map(|i| i % vocab_size).collect::<Vec<_>>();
        let output = model.forward(&tokens);

        // Output shape should be (sequence_length, vocab_size)
        assert_eq!(output.shape(), &[seq_len, vocab_size]);
    }
}
