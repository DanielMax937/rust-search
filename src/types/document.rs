// 文档数据结构

use std::path::PathBuf;

/// 代表一个被索引的文档
#[derive(Debug, Clone, PartialEq)]
pub struct Document {
    /// 文档的唯一标识符
    pub id: usize,
    /// 文档路径
    pub path: PathBuf,
    /// 文档内容
    pub content: String,
}

impl Document {
    /// 创建新文档
    pub fn new(id: usize, path: PathBuf, content: String) -> Self {
        Self { id, path, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            1,
            PathBuf::from("/path/to/file.txt"),
            "test content".to_string(),
        );

        assert_eq!(doc.id, 1);
        assert_eq!(doc.path, PathBuf::from("/path/to/file.txt"));
        assert_eq!(doc.content, "test content");
    }

    #[test]
    fn test_document_clone() {
        let doc1 = Document::new(
            1,
            PathBuf::from("/path/to/file.txt"),
            "test content".to_string(),
        );

        let doc2 = doc1.clone();
        assert_eq!(doc1, doc2);
    }

    #[test]
    fn test_document_with_empty_content() {
        let doc = Document::new(
            0,
            PathBuf::from("/empty.txt"),
            String::new(),
        );

        assert_eq!(doc.content, "");
        assert_eq!(doc.id, 0);
    }

    #[test]
    fn test_document_with_multiline_content() {
        let content = "line 1\nline 2\nline 3".to_string();
        let doc = Document::new(
            42,
            PathBuf::from("/multi.txt"),
            content.clone(),
        );

        assert_eq!(doc.content, content);
        assert!(doc.content.contains('\n'));
    }
}

