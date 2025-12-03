// 分词器使用示例

use rust_search::tokenizer::{SimpleTokenizer, Tokenizer};

fn main() {
    let tokenizer = SimpleTokenizer::new();
    
    // 示例文本
    let text = "Hello, World! This is a simple tokenizer demo. It works great!";
    
    println!("原文本:");
    println!("{}\n", text);
    
    // 分词
    let tokens = tokenizer.tokenize(text);
    
    println!("分词结果 (共 {} 个 tokens):", tokens.len());
    for token in tokens {
        println!("  [位置 {}] \"{}\"", token.position, token.text);
    }
}

