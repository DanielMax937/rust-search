// 简单分词器实现

use crate::tokenizer::Tokenizer;
use crate::types::Token;

/// 简单分词器 - 按空格和标点符号分词
pub struct SimpleTokenizer;

impl SimpleTokenizer {
    pub fn new() -> Self {
        Self
    }

    /// 判断字符是否为分隔符（空格、标点等）
    fn is_delimiter(c: char) -> bool {
        c.is_whitespace() || c.is_ascii_punctuation()
    }

    /// 规范化 token 文本（转小写）
    fn normalize(text: &str) -> String {
        text.to_lowercase()
    }
}

impl Default for SimpleTokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer for SimpleTokenizer {
    fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_word = String::new();
        let mut position = 0;

        for ch in text.chars() {
            if Self::is_delimiter(ch) {
                // 遇到分隔符，保存当前词条
                if !current_word.is_empty() {
                    let normalized = Self::normalize(&current_word);
                    tokens.push(Token::new(normalized, position));
                    current_word.clear();
                    position += 1;
                }
            } else {
                // 累积字符
                current_word.push(ch);
            }
        }

        // 处理最后一个词条
        if !current_word.is_empty() {
            let normalized = Self::normalize(&current_word);
            tokens.push(Token::new(normalized, position));
        }

        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenization() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("Hello, World! This is a test.");
        
        let texts: Vec<String> = tokens.iter().map(|t| t.text.clone()).collect();
        assert_eq!(texts, vec!["hello", "world", "this", "is", "a", "test"]);
    }

    #[test]
    fn test_position_tracking() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("one two three");
        
        assert_eq!(tokens[0].position, 0);
        assert_eq!(tokens[1].position, 1);
        assert_eq!(tokens[2].position, 2);
    }

    #[test]
    fn test_empty_string() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_normalize() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("UPPERCASE lowercase MiXeD");
        
        assert_eq!(tokens[0].text, "uppercase");
        assert_eq!(tokens[1].text, "lowercase");
        assert_eq!(tokens[2].text, "mixed");
    }

    #[test]
    fn test_multiple_spaces() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("word1    word2     word3");
        
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].text, "word1");
        assert_eq!(tokens[1].text, "word2");
        assert_eq!(tokens[2].text, "word3");
    }

    #[test]
    fn test_various_punctuation() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("hello! world? test: demo; end.");
        
        let texts: Vec<String> = tokens.iter().map(|t| t.text.clone()).collect();
        assert_eq!(texts, vec!["hello", "world", "test", "demo", "end"]);
    }

    #[test]
    fn test_numbers() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("hello 123 world 456");
        
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[1].text, "123");
        assert_eq!(tokens[3].text, "456");
    }

    #[test]
    fn test_mixed_content() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("The year 2024, is great!");
        
        let texts: Vec<String> = tokens.iter().map(|t| t.text.clone()).collect();
        assert_eq!(texts, vec!["the", "year", "2024", "is", "great"]);
    }

    #[test]
    fn test_only_punctuation() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("!@#$%^&*()");
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_newlines_and_tabs() {
        let tokenizer = SimpleTokenizer::new();
        let tokens = tokenizer.tokenize("line1\nline2\tline3");
        
        let texts: Vec<String> = tokens.iter().map(|t| t.text.clone()).collect();
        assert_eq!(texts, vec!["line1", "line2", "line3"]);
    }
}

