// 集成测试 - 测试整个搜索引擎流程

use rust_search::indexer::{Indexer, InvertedIndex};
use rust_search::searcher::{QuerySearcher, Searcher};
use rust_search::tokenizer::{SimpleTokenizer, Tokenizer};
use rust_search::types::Document;
use std::path::PathBuf;

#[test]
fn test_end_to_end_search() {
    // 1. 创建索引
    let mut index = InvertedIndex::new();

    // 2. 添加文档
    index.add_document(Document::new(
        1,
        PathBuf::from("/books/rust.txt"),
        "Rust is a systems programming language that runs blazingly fast.".to_string(),
    ));

    index.add_document(Document::new(
        2,
        PathBuf::from("/books/python.txt"),
        "Python is a high-level programming language.".to_string(),
    ));

    index.add_document(Document::new(
        3,
        PathBuf::from("/books/go.txt"),
        "Go is a statically typed programming language designed at Google.".to_string(),
    ));

    // 3. 创建搜索器
    let searcher = QuerySearcher::new(&index);

    // 4. 执行搜索
    let results = searcher.search("programming");
    assert_eq!(results.len(), 3);

    let results = searcher.search("rust");
    assert_eq!(results, vec![1]);

    let results = searcher.search_and("programming language");
    assert_eq!(results.len(), 3);

    let results = searcher.search_and("rust fast");
    assert_eq!(results, vec![1]);
}

#[test]
fn test_tokenizer_to_indexer_flow() {
    let tokenizer = SimpleTokenizer::new();
    let text = "Hello, World! Welcome to Rust.";

    // 分词
    let tokens = tokenizer.tokenize(text);
    assert_eq!(tokens.len(), 5);

    // 建立索引
    let mut index = InvertedIndex::new();
    index.add_document(Document::new(
        1,
        PathBuf::from("/test.txt"),
        text.to_string(),
    ));

    // 搜索
    let results = index.search("hello");
    assert_eq!(results, vec![1]);

    let results = index.search("rust");
    assert_eq!(results, vec![1]);
}

#[test]
fn test_large_document_set() {
    let mut index = InvertedIndex::new();

    // 添加多个文档
    for i in 1..=100 {
        let content = format!("Document {} contains some content about topic {}", i, i % 10);
        index.add_document(Document::new(
            i,
            PathBuf::from(format!("/doc{}.txt", i)),
            content,
        ));
    }

    assert_eq!(index.document_count(), 100);

    let searcher = QuerySearcher::new(&index);
    let results = searcher.search("document");
    assert_eq!(results.len(), 100);

    let results = searcher.search("topic");
    assert_eq!(results.len(), 100);
}

#[test]
fn test_search_result_retrieval() {
    let mut index = InvertedIndex::new();

    index.add_document(Document::new(
        1,
        PathBuf::from("/doc1.txt"),
        "first document".to_string(),
    ));

    index.add_document(Document::new(
        2,
        PathBuf::from("/doc2.txt"),
        "second document".to_string(),
    ));

    let searcher = QuerySearcher::new(&index);
    let results = searcher.search("document");

    // 验证可以通过搜索结果获取原始文档
    for doc_id in results {
        let doc = index.get_document(doc_id);
        assert!(doc.is_some());
        assert!(doc.unwrap().content.contains("document"));
    }
}

#[test]
fn test_empty_index_search() {
    let index = InvertedIndex::new();
    let searcher = QuerySearcher::new(&index);

    let results = searcher.search("anything");
    assert!(results.is_empty());
}

#[test]
fn test_multilingual_support() {
    let mut index = InvertedIndex::new();

    index.add_document(Document::new(
        1,
        PathBuf::from("/en.txt"),
        "English text".to_string(),
    ));

    index.add_document(Document::new(
        2,
        PathBuf::from("/mixed.txt"),
        "Mixed 123 text!".to_string(),
    ));

    let searcher = QuerySearcher::new(&index);
    let results = searcher.search("text");
    assert_eq!(results, vec![1, 2]);

    let results = searcher.search("123");
    assert_eq!(results, vec![2]);
}

