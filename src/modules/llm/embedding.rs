use ndarray::{s, Array, Array2, Axis};
use ndarray_rand::rand_distr::Uniform;
use ndarray_rand::RandomExt;

// --- Token Embedding ---

pub struct TokenEmbedding {
    weights: Array2<f32>,
}

impl TokenEmbedding {
    pub fn new(vocab_size: usize, d_model: usize) -> Self {
        Self {
            weights: Array2::random((vocab_size, d_model), Uniform::new(-0.1, 0.1)),
        }
    }

    pub fn forward(&self, token_ids: &[usize]) -> Array2<f32> {
        let indices = Array::from_vec(token_ids.to_vec());
        self.weights.select(Axis(0), indices.as_slice().unwrap())
    }
}

// --- Positional Encoding ---

pub struct PositionalEncoding {
    pe: Array2<f32>,
}

impl PositionalEncoding {
    pub fn new(max_seq_len: usize, d_model: usize) -> Self {
        let mut pe = Array2::<f32>::zeros((max_seq_len, d_model));
        let position = Array::range(0., max_seq_len as f32, 1.).insert_axis(Axis(1));
        let div_term = Array::range(0., d_model as f32, 2.)
            .mapv(|i| 1.0 / 10000.0_f32.powf(i / d_model as f32));

        pe.slice_mut(s![.., 0..;2])
            .assign(&(&position * &div_term).mapv(f32::sin));
        pe.slice_mut(s![.., 1..;2])
            .assign(&(&position * &div_term).mapv(f32::cos));

        Self { pe }
    }

    pub fn forward(&self, token_embeddings: &Array2<f32>) -> Array2<f32> {
        let seq_len = token_embeddings.shape()[0];
        token_embeddings + &self.pe.slice(s![..seq_len, ..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_embedding_shape() {
        let vocab_size = 100;
        let d_model = 16;
        let embedding = TokenEmbedding::new(vocab_size, d_model);
        let tokens = vec![10, 2, 99, 50];

        let output = embedding.forward(&tokens);

        assert_eq!(output.shape(), &[tokens.len(), d_model]);
    }

    #[test]
    fn test_positional_encoding_shape() {
        let max_seq_len = 50;
        let d_model = 16;
        let pos_encoding = PositionalEncoding::new(max_seq_len, d_model);

        let seq_len = 20;
        let embeddings = Array2::<f32>::zeros((seq_len, d_model));
        let output = pos_encoding.forward(&embeddings);

        assert_eq!(output.shape(), &[seq_len, d_model]);
    }

    #[test]
    fn test_positional_encoding_values() {
        let d_model = 4;
        let pos_encoding = PositionalEncoding::new(10, d_model);

        // pos=0, sin(0)=0, cos(0)=1
        assert_eq!(pos_encoding.pe[[0, 0]], 0.0);
        assert_eq!(pos_encoding.pe[[0, 1]], 1.0);
        assert_eq!(pos_encoding.pe[[0, 2]], 0.0);
        assert_eq!(pos_encoding.pe[[0, 3]], 1.0);

        // Check if values are not all zeros for other positions
        assert!(pos_encoding.pe.slice(s![1.., ..]).sum() != 0.0);
    }
}
