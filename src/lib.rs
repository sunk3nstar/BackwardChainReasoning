//! 推理算法

/// 逻辑项
pub trait Term {}

/// 错误
#[derive(Debug)]
pub enum ReasoningError {
    ThetaError,
}

impl std::fmt::Display for ReasoningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ReasoningError::ThetaError => {
                write!(
                    f,
                    "Substitution can only applied for a variable (Symbol::Var)."
                )
            }
        }
    }
}

impl std::error::Error for ReasoningError {}

/// 代表变量的逻辑符号
/// ```
/// use reasoning::{var, val, pred};
/// let x = var("X");
/// let zero = val("zero");
/// let is = pred("is", vec!(x, val("number")));
/// ```
#[derive(Debug)]
pub enum Symbol {
    Var(String),
    Val(String),
    Predicate(String, Vec<Symbol>),
}

impl Symbol {
    pub fn var(name: impl Into<String>) -> Self {
        Symbol::Var(name.into())
    }
    pub fn val(name: impl Into<String>) -> Self {
        Symbol::Val(name.into())
    }
    pub fn pred(name: impl Into<String>, args: Vec<Symbol>) -> Self {
        Symbol::Predicate(name.into(), args)
    }
}

#[inline]
pub fn var(s: &'static str) -> Symbol {
    Symbol::var(s)
}
#[inline]
pub fn val(s: &'static str) -> Symbol {
    Symbol::val(s)
}
#[inline]
pub fn pred(name: &'static str, args: Vec<Symbol>) -> Symbol {
    Symbol::pred(name, args)
}

impl Term for Symbol {}

/// 逻辑置换
/// `Theta { origin, result }` 表示以`result`替换`origin`的一个逻辑置换。  
/// 其中`origin`必须为变量(`Symbol::Var`)，否则返回ThetaError
pub struct Theta {
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
