# 测试驱动开发 (TDD) 指南

本项目采用 **测试驱动开发（TDD）** 方法，为所有核心模块提供全面的测试覆盖。

## 📊 测试统计

- **单元测试**: 40 个
- **集成测试**: 6 个
- **总计**: 46 个测试
- **覆盖率**: ✅ 100% 通过

## 🏗️ 测试结构

```
rust-search/
├── src/
│   ├── types/
│   │   ├── document.rs       # 4 个测试
│   │   └── token.rs          # 4 个测试
│   │
│   ├── tokenizer/
│   │   └── simple.rs         # 10 个测试
│   │
│   ├── indexer/
│   │   └── inverted_index.rs # 12 个测试
│   │
│   └── searcher/
│       └── query_searcher.rs # 10 个测试
│
└── tests/
    └── integration_test.rs   # 6 个集成测试
```

## 🧪 运行测试

### 运行所有测试
```bash
make test
# 或
cargo test
```

### 运行特定模块的测试
```bash
# 只测试分词器
cargo test tokenizer

# 只测试索引器
cargo test indexer

# 只测试搜索器
cargo test searcher

# 只测试类型模块
cargo test types

# 只运行集成测试
make test-integration
```

### 运行单个测试
```bash
cargo test test_simple_tokenization
```

### 显示测试输出
```bash
cargo test -- --nocapture
```

### TDD 监视模式
```bash
make tdd
# 文件改变时自动运行测试
```

## 📝 各模块测试覆盖

### 1. Types 模块 (8 个测试)

#### Document (4 个测试)
- ✅ `test_document_creation` - 文档创建
- ✅ `test_document_clone` - 文档克隆
- ✅ `test_document_with_empty_content` - 空内容文档
- ✅ `test_document_with_multiline_content` - 多行内容

#### Token (4 个测试)
- ✅ `test_token_creation` - Token 创建
- ✅ `test_token_equality` - Token 相等性比较
- ✅ `test_token_hash` - Token 哈希能力
- ✅ `test_token_clone` - Token 克隆

### 2. Tokenizer 模块 (10 个测试)

- ✅ `test_simple_tokenization` - 基本分词
- ✅ `test_position_tracking` - 位置跟踪
- ✅ `test_empty_string` - 空字符串处理
- ✅ `test_normalize` - 大小写规范化
- ✅ `test_multiple_spaces` - 多空格处理
- ✅ `test_various_punctuation` - 各种标点符号
- ✅ `test_numbers` - 数字处理
- ✅ `test_mixed_content` - 混合内容
- ✅ `test_only_punctuation` - 纯标点符号
- ✅ `test_newlines_and_tabs` - 换行符和制表符

### 3. Indexer 模块 (12 个测试)

- ✅ `test_create_empty_index` - 创建空索引
- ✅ `test_add_single_document` - 添加单个文档
- ✅ `test_search_single_term` - 单词搜索
- ✅ `test_search_nonexistent_term` - 搜索不存在的词
- ✅ `test_search_multiple_documents` - 多文档搜索
- ✅ `test_search_case_insensitive` - 大小写不敏感搜索
- ✅ `test_get_document` - 获取文档
- ✅ `test_document_frequency` - 文档频率
- ✅ `test_duplicate_words_in_document` - 文档内重复词
- ✅ `test_punctuation_handling` - 标点符号处理
- ✅ `test_empty_document` - 空文档
- ✅ `test_multiple_documents_with_overlap` - 重叠文档

### 4. Searcher 模块 (10 个测试)

- ✅ `test_search_or_single_term` - OR 单词搜索
- ✅ `test_search_or_multiple_terms` - OR 多词搜索
- ✅ `test_search_and_single_term` - AND 单词搜索
- ✅ `test_search_and_multiple_terms` - AND 多词搜索
- ✅ `test_search_and_no_results` - AND 无结果
- ✅ `test_search_empty_query` - 空查询
- ✅ `test_search_nonexistent_term` - 不存在的词
- ✅ `test_default_search_is_or` - 默认 OR 搜索
- ✅ `test_search_case_insensitive` - 大小写不敏感
- ✅ `test_search_with_punctuation` - 标点符号查询

### 5. 集成测试 (6 个测试)

- ✅ `test_end_to_end_search` - 端到端搜索流程
- ✅ `test_tokenizer_to_indexer_flow` - 分词器到索引器流程
- ✅ `test_large_document_set` - 大文档集（100 个文档）
- ✅ `test_search_result_retrieval` - 搜索结果检索
- ✅ `test_empty_index_search` - 空索引搜索
- ✅ `test_multilingual_support` - 多语言支持

## 🎯 TDD 工作流程

### 红-绿-重构循环

#### 1. 红色阶段 ❌ - 编写失败的测试
```rust
#[test]
fn test_new_feature() {
    let result = my_new_function();
    assert_eq!(result, expected_value);
}
```

#### 2. 运行测试（应该失败）
```bash
cargo test test_new_feature
```

#### 3. 绿色阶段 ✅ - 实现功能使测试通过
```rust
fn my_new_function() -> Type {
    // 实现最简单的代码使测试通过
}
```

#### 4. 再次运行测试（应该通过）
```bash
cargo test test_new_feature
```

#### 5. 重构阶段 🔧 - 优化代码
```rust
fn my_new_function() -> Type {
    // 重构和优化代码
}
```

#### 6. 确保测试仍然通过
```bash
cargo test
```

## 🔍 测试覆盖的场景

### 边界情况
- ✅ 空字符串/空文档
- ✅ 单个元素
- ✅ 大量数据（100+ 文档）
- ✅ 不存在的查询

### 数据处理
- ✅ 大小写规范化
- ✅ 标点符号处理
- ✅ 空格和换行符处理
- ✅ 数字处理

### 功能正确性
- ✅ 分词准确性
- ✅ 索引构建正确性
- ✅ 搜索结果准确性
- ✅ AND/OR 查询逻辑

### 性能
- ✅ 大文档集处理
- ✅ 重复词处理效率

## 💡 最佳实践

1. **先写测试，再写代码**
   - 明确功能需求
   - 避免过度设计
   
2. **保持测试简单**
   - 每个测试只测一个功能点
   - 测试名称要描述清楚
   
3. **使用有意义的断言**
   - 使用 `assert_eq!` 而不是 `assert!`
   - 提供清晰的错误信息

4. **频繁运行测试**
   - 使用 `make tdd` 进入监视模式
   - 每次修改后立即验证

5. **保持测试独立**
   - 每个测试应该能独立运行
   - 不依赖其他测试的状态

## 📈 持续集成建议

```yaml
# .github/workflows/test.yml 示例
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test --verbose
      - run: cargo clippy -- -D warnings
      - run: cargo fmt -- --check
```

## 🚀 下一步测试计划

- [ ] 添加基准测试（benchmarks）
- [ ] 添加性能回归测试
- [ ] 添加模糊测试（fuzzing）
- [ ] 测试并发场景
- [ ] 添加内存泄漏测试

## 📚 参考资源

- [Rust 测试官方文档](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [TDD 最佳实践](https://testdriven.io/)
- [Rust 测试模式](https://rust-lang.github.io/api-guidelines/)

