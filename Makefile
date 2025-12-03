# Rust Search Engine - 快捷命令

.PHONY: help test test-watch test-unit test-integration test-all build run clean lint fmt

help: ## 显示帮助信息
	@echo "可用命令："
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}'

test: ## 运行所有测试
	@cargo test

test-watch: ## 监视文件变化并自动运行测试
	@cargo watch -x test

test-unit: ## 只运行单元测试
	@cargo test --lib

test-integration: ## 只运行集成测试
	@cargo test --test integration_test

test-verbose: ## 运行测试并显示详细输出
	@cargo test -- --show-output

test-all: ## 运行所有测试（详细模式）
	@cargo test --verbose

build: ## 编译项目
	@cargo build

build-release: ## 编译发布版本
	@cargo build --release

run: ## 运行 CLI
	@cargo run

run-demo: ## 运行分词器演示
	@cargo run --example tokenizer_demo

clean: ## 清理构建文件
	@cargo clean

lint: ## 运行 linter
	@cargo clippy -- -D warnings

fmt: ## 格式化代码
	@cargo fmt

fmt-check: ## 检查代码格式
	@cargo fmt -- --check

check: ## 快速检查代码
	@cargo check

# TDD 工作流
tdd: ## TDD 模式：监视文件并自动测试
	@echo "进入 TDD 模式..."
	@cargo watch -x 'test --color always' -c -q

# 测试覆盖率（需要安装 cargo-tarpaulin）
coverage: ## 生成测试覆盖率报告
	@cargo tarpaulin --out Html --output-dir coverage

# 基准测试（需要安装 cargo-criterion）
bench: ## 运行基准测试
	@cargo bench

