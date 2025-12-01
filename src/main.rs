use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)] // 核心宏：将结构体转为 CLI 解析器
#[command(name = "rust-search")]
#[command(about = "一个简单的搜索和索引 CLI 工具", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   Index {
    #[arg(value_name = "FILE_PATH")]
    path: PathBuf,
   },
   Search {
    query: String,
   },
}

fn main() {
    let cli=Cli::parse();

    match &cli.command {
        Commands::Index { path } => {
            println!("Indexing file: {}", path.display());
        },
        Commands::Search { query } => {
            println!("Searching for: {}", query);
        },
    }
}