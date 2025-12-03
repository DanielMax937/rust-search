// 搜索器模块

mod query_searcher;

pub use query_searcher::QuerySearcher;

/// 搜索器 trait - 定义搜索器的公共接口
pub trait Searcher {
    /// 执行搜索查询，返回匹配的文档 ID 列表
    fn search(&self, query: &str) -> Vec<usize>;
}

