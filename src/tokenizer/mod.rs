// 分词器模块

mod simple;

pub use simple::SimpleTokenizer;

use crate::types::Token;

/// 分词器 trait - 定义分词器的公共接口
pub trait Tokenizer {
    /// 将文本分词为 tokens
    fn tokenize(&self, text: &str) -> Vec<Token>;
}

