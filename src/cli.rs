//! ## 用于反向链接推理的命令行入口

use super::{KB, ReasoningError, Symbol, Theta};
use crate::bc::bc;
use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
pub struct Cli {
    /// 为知识库传入JSON文件
    #[arg(long = "kbfile", action = ArgAction::SetTrue)]
    pub kbfile: bool,
    /// 用于推理的已知条件知识库
    pub knowledge_base: String,
    /// 为待证命题传入JSON文件
    #[arg(long = "file", action = ArgAction::SetTrue)]
    pub smfile: bool,
    /// 待证命题
    pub statement: String,
    /// 显示详细证明过程
    #[arg(long = "verbose", action = ArgAction::SetTrue)]
    pub verbose: bool,
}

/// ## 使用反向链接的逻辑证明器
pub fn prove(args: &Cli) -> Result<(), ReasoningError> {
    let mut kb: KB;
    if args.kbfile {
        let data = std::fs::read_to_string(&args.knowledge_base)
            .map_err(|_| ReasoningError::FileError(args.knowledge_base.clone()))?;
        kb = serde_json::from_str(&data)?;
    } else {
        kb = serde_json::from_str(&args.knowledge_base)?;
    }
    kb.standardize_var();
    let theorem: Symbol = if args.smfile {
        let data = std::fs::read_to_string(&args.statement)
            .map_err(|_| ReasoningError::FileError(args.statement.clone()))?;
        serde_json::from_str(&data)?
    } else {
        serde_json::from_str(&args.statement)?
    };
    let mut thetas = Vec::<Theta>::new();
    let mut stack = Vec::<Symbol>::new();
    bc(&kb, &theorem, &mut thetas, args.verbose, &mut stack, 0, 100)?;
    Ok(())
}
