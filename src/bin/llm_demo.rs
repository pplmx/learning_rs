use learning_rs::modules::llm::model::LanguageModel;

fn main() {
    println!("=== 运行完整的语言模型 ===\n");

    // --- 模型参数 ---
    let vocab_size = 1000;
    let d_model = 64;
    let max_seq_len = 100;
    let num_blocks = 4;
    let num_heads = 4;
    let d_ff = 256; // 通常是 d_model 的 4 倍

    // --- 输入 ---
    let input_tokens: Vec<usize> = vec![10, 25, 5, 99, 30, 72];
    let seq_len = input_tokens.len();

    println!("模型参数:");
    println!("  - 词汇表大小: {}", vocab_size);
    println!("  - 模型维度 (d_model): {}", d_model);
    println!("  - Transformer Blocks 数量: {}", num_blocks);
    println!("  - 注意力头数: {}", num_heads);
    println!("\n输入 Token IDs: {:?}", input_tokens);
    println!("输入序列长度: {}", seq_len);

    // --- 初始化并运行模型 ---
    let model = LanguageModel::new(
        vocab_size,
        d_model,
        max_seq_len,
        num_blocks,
        num_heads,
        d_ff,
    );

    let output_logits = model.forward(&input_tokens);

    // --- 打印输出信息 ---
    println!("\n模型前向传播完成!");
    println!("输出 Logits 形状: {:?}", output_logits.shape());
    println!(
        "\n预期形状: [sequence_length, vocab_size] -> [{}, {}]",
        seq_len,
        vocab_size
    );

    println!("\n✓ 演示完成!");
}