// 查询搜索器实现

use crate::indexer::{Indexer, InvertedIndex};
use crate::searcher::Searcher;
use crate::tokenizer::{SimpleTokenizer, Tokenizer};
use std::collections::HashSet;

/// 查询搜索器 - 支持多词查询
pub struct QuerySearcher<'a> {
    index: &'a InvertedIndex,
    tokenizer: SimpleTokenizer,
}

impl<'a> QuerySearcher<'a> {
    /// 创建新的查询搜索器
    pub fn new(index: &'a InvertedIndex) -> Self {
        Self {
            index,
            tokenizer: SimpleTokenizer::new(),
        }
    }

    /// AND 查询：返回包含所有查询词条的文档
    pub fn search_and(&self, query: &str) -> Vec<usize> {
        let tokens = self.tokenizer.tokenize(query);

        if tokens.is_empty() {
            return Vec::new();
        }

        // 获取第一个词条的结果作为初始集合
        let mut result_set: HashSet<usize> = self
            .index
            .search(&tokens[0].text)
            .into_iter()
            .collect();

        // 对剩余的词条求交集
        for token in tokens.iter().skip(1) {
            let docs: HashSet<usize> = self
                .index
                .search(&token.text)
                .into_iter()
                .collect();
            result_set = result_set.intersection(&docs).copied().collect();
        }

        let mut results: Vec<usize> = result_set.into_iter().collect();
        results.sort();
        results
    }

    /// OR 查询：返回包含任一查询词条的文档
    pub fn search_or(&self, query: &str) -> Vec<usize> {
        let tokens = self.tokenizer.tokenize(query);

        if tokens.is_empty() {
            return Vec::new();
        }

        // 收集所有匹配的文档 ID
        let mut result_set = HashSet::new();

        for token in tokens {
            let docs = self.index.search(&token.text);
            result_set.extend(docs);
        }

        let mut results: Vec<usize> = result_set.into_iter().collect();
        results.sort();
        results
    }
}

impl<'a> Searcher for QuerySearcher<'a> {
    /// 默认使用 OR 查询
    fn search(&self, query: &str) -> Vec<usize> {
        self.search_or(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Document;
    use std::path::PathBuf;

    fn create_test_index() -> InvertedIndex {
        let mut index = InvertedIndex::new();

        index.add_document(Document::new(
            1,
            PathBuf::from("/doc1.txt"),
            "rust programming language".to_string(),
        ));

        index.add_document(Document::new(
            2,
            PathBuf::from("/doc2.txt"),
            "python programming language".to_string(),
        ));

        index.add_document(Document::new(
            3,
            PathBuf::from("/doc3.txt"),
            "rust is fast and safe".to_string(),
        ));

        index.add_document(Document::new(
            4,
            PathBuf::from("/doc4.txt"),
            "java programming".to_string(),
        ));

        index
    }

    #[test]
    fn test_search_or_single_term() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        let results = searcher.search_or("rust");
        assert_eq!(results, vec![1, 3]);
    }

    #[test]
    fn test_search_or_multiple_terms() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        let results = searcher.search_or("rust python");
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[test]
    fn test_search_and_single_term() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        let results = searcher.search_and("rust");
        assert_eq!(results, vec![1, 3]);
    }

    #[test]
    fn test_search_and_multiple_terms() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        // 只有 doc1 和 doc2 同时包含 "programming" 和 "language"
        let results = searcher.search_and("programming language");
        assert_eq!(results, vec![1, 2]);

        // 只有 doc1 同时包含 "rust" 和 "programming"
        let results = searcher.search_and("rust programming");
        assert_eq!(results, vec![1]);

        // 只有 doc3 同时包含 "rust" 和 "fast"
        let results = searcher.search_and("rust fast");
        assert_eq!(results, vec![3]);
    }

    #[test]
    fn test_search_and_no_results() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        // 没有文档同时包含 "rust" 和 "python"
        let results = searcher.search_and("rust python");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_empty_query() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        let results = searcher.search_or("");
        assert!(results.is_empty());

        let results = searcher.search_and("");
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_nonexistent_term() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        let results = searcher.search_or("nonexistent");
        assert!(results.is_empty());
    }

    #[test]
    fn test_default_search_is_or() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        let results1 = searcher.search("rust python");
        let results2 = searcher.search_or("rust python");

        assert_eq!(results1, results2);
    }

    #[test]
    fn test_search_case_insensitive() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        let results1 = searcher.search_or("RUST");
        let results2 = searcher.search_or("rust");
        let results3 = searcher.search_or("RuSt");

        assert_eq!(results1, results2);
        assert_eq!(results2, results3);
    }

    #[test]
    fn test_search_with_punctuation() {
        let index = create_test_index();
        let searcher = QuerySearcher::new(&index);

        // "rust, programming!" 分词后是 ["rust", "programming"]
        // rust 在 doc1, doc3; programming 在 doc1, doc2, doc4
        // OR 查询应该返回 doc1, doc2, doc3, doc4
        let results = searcher.search_or("rust, programming!");
        assert_eq!(results, vec![1, 2, 3, 4]);
    }
}

