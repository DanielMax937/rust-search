// Token（词条）数据结构

/// 代表文本分词后的一个词条
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    /// 词条文本（已规范化：小写、去除标点等）
    pub text: String,
    /// 词条在原文本中的位置
    pub position: usize,
}

impl Token {
    /// 创建新 token
    pub fn new(text: String, position: usize) -> Self {
        Self { text, position }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_token_creation() {
        let token = Token::new("hello".to_string(), 0);
        assert_eq!(token.text, "hello");
        assert_eq!(token.position, 0);
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new("test".to_string(), 5);
        let token2 = Token::new("test".to_string(), 5);
        let token3 = Token::new("test".to_string(), 6);
        let token4 = Token::new("other".to_string(), 5);

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
        assert_ne!(token1, token4);
    }

    #[test]
    fn test_token_hash() {
        let mut set = HashSet::new();
        let token1 = Token::new("word".to_string(), 0);
        let token2 = Token::new("word".to_string(), 0);
        let token3 = Token::new("word".to_string(), 1);

        set.insert(token1.clone());
        set.insert(token2.clone());
        set.insert(token3.clone());

        // token1 和 token2 相同，所以 set 中只有 2 个元素
        assert_eq!(set.len(), 2);
        assert!(set.contains(&token1));
        assert!(set.contains(&token3));
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new("clone".to_string(), 10);
        let token2 = token1.clone();

        assert_eq!(token1, token2);
        assert_eq!(token1.text, token2.text);
        assert_eq!(token1.position, token2.position);
    }
}

