//! ## 包含反向链接算法的一阶谓词逻辑实现

use serde::{Deserialize, Serialize};
use std::fmt::Display;
mod bc;
#[cfg(any(test, feature = "benchmark"))]
pub mod bench;
pub mod cli;
mod unify;

/// ## 错误类型
#[derive(Debug)]
pub enum ReasoningError {
    ThetaError,
    UnifyError,
    DepthLimitExceed,
    CycleProof,
    ProofNotFound,
    ParseError,
    FileError(String),
}

impl Display for ReasoningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasoningError::ThetaError => {
                write!(f, "只允许替换变量 (Symbol::Var)")
            }
            ReasoningError::UnifyError => {
                write!(f, "未找到合一")
            }
            ReasoningError::DepthLimitExceed => {
                write!(f, "推理递归深度超限")
            }
            ReasoningError::CycleProof => {
                write!(f, "发生循环论证")
            }
            ReasoningError::ParseError => {
                write!(f, "JSON格式错误")
            }
            ReasoningError::FileError(name) => {
                write!(f, "无法读取文件{}", name)
            }
            ReasoningError::ProofNotFound => {
                write!(f, "未找到有效证明路径")
            }
        }
    }
}

impl std::error::Error for ReasoningError {}

impl From<serde_json::Error> for ReasoningError {
    fn from(_value: serde_json::Error) -> Self {
        ReasoningError::ParseError
    }
}
/// ## 逻辑项
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum Symbol {
    /// 变量
    Var(String),
    /// 常量
    Val(String),
    /// 函数符号
    Func(String, Vec<Symbol>),
}

impl Symbol {
    pub fn var(name: impl Into<String>) -> Self {
        Symbol::Var(name.into())
    }
    pub fn val(name: impl Into<String>) -> Self {
        Symbol::Val(name.into())
    }
    pub fn func(name: impl Into<String>, args: Vec<Symbol>) -> Self {
        Symbol::Func(name.into(), args)
    }
    /// ## 判断符号中是否含有变量
    /// 更常见的用法是判断符号是否仅仅由常量和仅包含常量的函数组成，也即判断该方法是否返回false
    pub fn contains_var(&self) -> bool {
        match self {
            Self::Var(_) => true,
            Self::Val(_) => false,
            Self::Func(_, args) => {
                for arg in args {
                    if arg.contains_var() {
                        return true;
                    }
                }
                false
            }
        }
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Var(name) => {
                write!(f, "{}", name.to_uppercase())
            }
            Symbol::Val(name) => {
                write!(f, "{}", name)
            }
            Symbol::Func(name, args) => {
                write!(f, "{}(", name)?;
                let mut args_iter = args.iter();
                write!(f, "{}", args_iter.next().unwrap())?;
                for arg in args_iter {
                    write!(f, ",{arg}")?;
                }
                write!(f, ")")
            }
        }
    }
}

/// 原子公式
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
struct Atom {
    predicate: String,
    args: Vec<Symbol>,
}

/// ## 判断原子公式中是否含有变量
/// 更常见的用法是判断原子公式是否仅仅由常量和仅包含常量的函数组成，也即判断该方法是否返回false
impl Atom {
    fn contains_var(&self) -> bool {
        self.args.iter().any(|arg| arg.contains_var())
    }
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.predicate)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}

/// ## 常量构造函数
#[inline]
fn var(s: impl Into<String>) -> Symbol {
    Symbol::var(s)
}
/// ## 变量构造函数
#[inline]
fn val(s: impl Into<String>) -> Symbol {
    Symbol::val(s)
}
/// ## 函数构造函数
#[inline]
fn func(name: impl Into<String>, args: Vec<Symbol>) -> Symbol {
    Symbol::func(name, args)
}

/// ## 原子公式构造函数
fn pred(name: impl Into<String>, args: Vec<Symbol>) -> Atom {
    Atom {
        predicate: name.into(),
        args,
    }
}

/// ## 规则（霍恩子句）
/// 形如X^Y^Z=>W的语句。=>左侧为condition，右侧为conclusion
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Rule {
    pub condition: Vec<Atom>,
    pub conclusion: Atom,
}

impl Rule {
    /// ## 判断规则是否是无条件的常量事实
    pub fn is_fact(&self) -> bool {
        self.condition.is_empty() && !self.conclusion.contains_var()
    }
}

/// ## 逻辑置换记号
/// `Theta { origin, result }` 表示以`result`替换`origin`的一个逻辑置换。
/// 其中`origin`必须为变量(`Symbol::Var`)，否则返回ThetaError
#[derive(Debug, Clone)]
struct Theta {
    origin: Symbol,
    result: Symbol,
}

impl Theta {
    pub fn new(origin: Symbol, result: Symbol) -> Result<Self, ReasoningError> {
        match origin {
            Symbol::Var(_) => Ok(Theta { origin, result }),
            _ => Err(ReasoningError::ThetaError),
        }
    }
}

/// ## 知识库
/// 由规则rules组成
#[derive(Serialize, Deserialize)]
struct KB {
    rules: Vec<Rule>,
}

impl KB {
    // 在变量后追加编号
    fn index_var(x: &Symbol, i: usize) -> Symbol {
        match x {
            Symbol::Var(name) => var(format!("{name}{i}")),
            Symbol::Func(name, args) => {
                let mut new_args = Vec::<Symbol>::new();
                for arg in args.iter() {
                    new_args.push(KB::index_var(&arg.clone(), i))
                }
                func(name.clone(), new_args)
            }
            _ => x.clone(),
        }
    }
    // 为原子公式中每个变量追加统一编号
    fn index_atom(x: &Atom, i: usize) -> Atom {
        Atom {
            predicate: x.predicate.clone(),
            args: x.args.iter().map(|arg| KB::index_var(arg, i)).collect(),
        }
    }
    /// ## 规则标准化
    /// 为一条规则中的变量追加指定序号
    pub fn rule_standardize(r: &Rule, i: usize) -> Rule {
        let mut new_condition = Vec::<Atom>::new();
        for condition in r.condition.iter() {
            new_condition.push(KB::index_atom(condition, i));
        }
        Rule {
            condition: new_condition,
            conclusion: KB::index_atom(&r.conclusion, i),
        }
    }
}
