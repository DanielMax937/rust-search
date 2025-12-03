// 索引器模块

mod inverted_index;

pub use inverted_index::InvertedIndex;

use crate::types::Document;

/// 索引器 trait - 定义索引器的公共接口
pub trait Indexer {
    /// 添加文档到索引
    fn add_document(&mut self, document: Document);
    
    /// 搜索包含指定词条的文档 ID
    fn search(&self, term: &str) -> Vec<usize>;
}

