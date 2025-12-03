// 倒排索引实现

use crate::indexer::Indexer;
use crate::tokenizer::{SimpleTokenizer, Tokenizer};
use crate::types::Document;
use std::collections::{HashMap, HashSet};

/// 倒排索引 - 从词条到文档 ID 的映射
pub struct InvertedIndex {
    /// term -> set of document IDs
    index: HashMap<String, HashSet<usize>>,
    /// 存储所有文档
    documents: HashMap<usize, Document>,
    /// 分词器
    tokenizer: SimpleTokenizer,
}

impl InvertedIndex {
    /// 创建新的倒排索引
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            documents: HashMap::new(),
            tokenizer: SimpleTokenizer::new(),
        }
    }

    /// 获取文档数量
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    /// 获取索引中的词条数量
    pub fn term_count(&self) -> usize {
        self.index.len()
    }

    /// 根据 ID 获取文档
    pub fn get_document(&self, id: usize) -> Option<&Document> {
        self.documents.get(&id)
    }

    /// 获取包含指定词条的文档数量
    pub fn document_frequency(&self, term: &str) -> usize {
        self.index
            .get(term)
            .map(|docs| docs.len())
            .unwrap_or(0)
    }
}

impl Default for InvertedIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl Indexer for InvertedIndex {
    fn add_document(&mut self, document: Document) {
        let doc_id = document.id;
        let content = document.content.clone();

        // 对文档内容分词
        let tokens = self.tokenizer.tokenize(&content);

        // 将每个词条添加到倒排索引
        for token in tokens {
            self.index
                .entry(token.text)
                .or_insert_with(HashSet::new)
                .insert(doc_id);
        }

        // 存储文档
        self.documents.insert(doc_id, document);
    }

    fn search(&self, term: &str) -> Vec<usize> {
        // 规范化搜索词（转小写）
        let normalized_term = term.to_lowercase();

        // 查找包含该词条的所有文档 ID
        self.index
            .get(&normalized_term)
            .map(|doc_ids| {
                let mut ids: Vec<usize> = doc_ids.iter().copied().collect();
                ids.sort(); // 排序以保证结果顺序一致
                ids
            })
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn create_test_document(id: usize, content: &str) -> Document {
        Document::new(id, PathBuf::from(format!("/test{}.txt", id)), content.to_string())
    }

    #[test]
    fn test_create_empty_index() {
        let index = InvertedIndex::new();
        assert_eq!(index.document_count(), 0);
        assert_eq!(index.term_count(), 0);
    }

    #[test]
    fn test_add_single_document() {
        let mut index = InvertedIndex::new();
        let doc = create_test_document(1, "hello world");

        index.add_document(doc);

        assert_eq!(index.document_count(), 1);
        assert_eq!(index.term_count(), 2); // "hello" and "world"
    }

    #[test]
    fn test_search_single_term() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "hello world"));

        let results = index.search("hello");
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn test_search_nonexistent_term() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "hello world"));

        let results = index.search("nonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_multiple_documents() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "hello world"));
        index.add_document(create_test_document(2, "hello rust"));
        index.add_document(create_test_document(3, "goodbye world"));

        let results = index.search("hello");
        assert_eq!(results, vec![1, 2]);

        let results = index.search("world");
        assert_eq!(results, vec![1, 3]);

        let results = index.search("rust");
        assert_eq!(results, vec![2]);
    }

    #[test]
    fn test_search_case_insensitive() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "Hello World"));

        let results1 = index.search("hello");
        let results2 = index.search("HELLO");
        let results3 = index.search("HeLLo");

        assert_eq!(results1, vec![1]);
        assert_eq!(results2, vec![1]);
        assert_eq!(results3, vec![1]);
    }

    #[test]
    fn test_get_document() {
        let mut index = InvertedIndex::new();
        let doc = create_test_document(42, "test content");
        index.add_document(doc.clone());

        let retrieved = index.get_document(42);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, 42);
        assert_eq!(retrieved.unwrap().content, "test content");

        let nonexistent = index.get_document(999);
        assert!(nonexistent.is_none());
    }

    #[test]
    fn test_document_frequency() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "hello world"));
        index.add_document(create_test_document(2, "hello rust"));
        index.add_document(create_test_document(3, "goodbye world"));

        assert_eq!(index.document_frequency("hello"), 2);
        assert_eq!(index.document_frequency("world"), 2);
        assert_eq!(index.document_frequency("rust"), 1);
        assert_eq!(index.document_frequency("goodbye"), 1);
        assert_eq!(index.document_frequency("nonexistent"), 0);
    }

    #[test]
    fn test_duplicate_words_in_document() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "hello hello hello"));

        let results = index.search("hello");
        assert_eq!(results, vec![1]);
        assert_eq!(index.document_frequency("hello"), 1);
    }

    #[test]
    fn test_punctuation_handling() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "Hello, World!"));

        let results = index.search("hello");
        assert_eq!(results, vec![1]);

        let results = index.search("world");
        assert_eq!(results, vec![1]);
    }

    #[test]
    fn test_empty_document() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, ""));

        assert_eq!(index.document_count(), 1);
        assert_eq!(index.term_count(), 0);
    }

    #[test]
    fn test_multiple_documents_with_overlap() {
        let mut index = InvertedIndex::new();
        index.add_document(create_test_document(1, "rust programming language"));
        index.add_document(create_test_document(2, "python programming language"));
        index.add_document(create_test_document(3, "rust is fast"));

        let results = index.search("programming");
        assert_eq!(results, vec![1, 2]);

        let results = index.search("language");
        assert_eq!(results, vec![1, 2]);

        let results = index.search("rust");
        assert_eq!(results, vec![1, 3]);
    }
}

