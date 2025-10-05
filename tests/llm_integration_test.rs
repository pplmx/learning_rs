#[cfg(test)]
mod tests {
    // Import from the library crate
    use learning_rs::modules::llm::attn::SelfAttentionBuilder;

    #[test]
    fn test_attention_from_outside() {
        let attention = SelfAttentionBuilder::new().d_model(8).d_k(4).build();
        assert!(attention.is_ok());
    }
}
