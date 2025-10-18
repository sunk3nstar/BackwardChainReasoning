//! ## 包含反向链接算法的一阶谓词逻辑实现

use serde::{Deserialize, Serialize};
use std::fmt::Display;
mod bc;
pub mod cli;
mod unify;

/// ## 逻辑语句
pub trait Statement {
    /// 置换
    /// ```
    /// use reasoning::{var, val, pred, Rule, Theta};
    /// use reasoning::Statement;
    /// let is = pred("is", vec!(var("X"), var("Y")));
    /// let mut theta_list = vec!(
    ///   Theta::new(var("X"), val("Bird")).unwrap(),
    ///   Theta::new(var("Y"), val("Animal")).unwrap()
    /// );
    /// let bird_is_animal = is.subst(&theta_list);
    /// theta_list.push(
    ///   Theta::new(var("Z"), val("Robin")).unwrap()
    /// );
    /// let r = Rule {
    ///   condition: vec!(
    ///     pred("is", vec!(var("Z"), var("Y"))),
    ///     pred("is", vec!(var("Y"), var("X")))
    ///   ),
    ///   conclusion: pred("is", vec!(var("Z"), var("X")))
    /// };
    /// let robin_is_bird_is_animal = r.subst(&theta_list);
    /// ```
    fn subst(&self, theta_list: &[Theta]) -> Self
    where
        Self: Sized;
}

/// ## 错误类型
#[derive(Debug)]
pub enum ReasoningError {
    ThetaError,
    UnifyError,
    ParseError,
    FileError(String),
}

impl Display for ReasoningError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReasoningError::ThetaError => {
                write!(
                    f,
                    "Substitution can only applied for a variable (Symbol::Var)."
                )
            }
            ReasoningError::UnifyError => {
                write!(f, "No available unification found.")
            }
            ReasoningError::ParseError => {
                write!(f, "JSON格式错误")
            }
            ReasoningError::FileError(name) => {
                write!(f, "无法读取文件{}", name)
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
/// ## 逻辑符号
/// ```
/// use reasoning::{var, val, pred};
/// let x = var("X");
/// let zero = val("zero");
/// let is = pred("is", vec!(x.clone(), val("number")));
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Var(name) => {
                write!(f, "{}", name.to_uppercase())
            }
            Symbol::Val(name) => {
                write!(f, "{}", name)
            }
            Symbol::Predicate(name, args) => {
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

/// ## 常量构造函数
#[inline]
pub fn var(s: impl Into<String>) -> Symbol {
    Symbol::var(s)
}
/// ## 变量构造函数
#[inline]
pub fn val(s: impl Into<String>) -> Symbol {
    Symbol::val(s)
}
/// ## 谓词构造函数
#[inline]
pub fn pred(name: impl Into<String>, args: Vec<Symbol>) -> Symbol {
    Symbol::pred(name, args)
}

impl Statement for Symbol {
    fn subst(&self, theta_list: &[Theta]) -> Self
    where
        Self: Sized,
    {
        match *self {
            Symbol::Val(_) => self.clone(),
            _ => {
                let mut new = self.clone();
                for theta in theta_list {
                    new = subst_single(&new, theta);
                }
                new
            }
        }
    }
}

/// ## 规则（一阶确定子句）  
/// 形如X^Y^Z=>W的语句。=>左侧为condition，右侧为conclusion
/// ```
/// # use reasoning::{var, val, pred, Rule};
/// let r = Rule {
///   condition: vec!(
///     pred("is", vec!(var("X"), val("bird"))),
///     pred("is", vec!(val("bird"), val("animal")))
///   ),
///   conclusion: pred("is", vec!(var("X"), val("animal")))
/// };
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub condition: Vec<Symbol>,
    pub conclusion: Symbol,
}

impl Statement for Rule {
    fn subst(&self, theta_list: &[Theta]) -> Self
    where
        Self: Sized,
    {
        let mut new_conditions = Vec::<Symbol>::new();
        for symbol in &self.condition {
            new_conditions.push(symbol.subst(theta_list));
        }
        Rule {
            condition: new_conditions,
            conclusion: self.conclusion.subst(theta_list),
        }
    }
}

/// ## 逻辑置换记号
/// `Theta { origin, result }` 表示以`result`替换`origin`的一个逻辑置换。  
/// 其中`origin`必须为变量(`Symbol::Var`)，否则返回ThetaError
#[derive(Debug, Clone)]
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

fn subst_single(src: &Symbol, theta: &Theta) -> Symbol {
    match src {
        Symbol::Val(_) => src.clone(),
        Symbol::Var(x) => match &theta.origin {
            Symbol::Var(name) if name == x => theta.result.clone(),
            _ => src.clone(),
        },
        Symbol::Predicate(name, args) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(subst_single(arg, theta));
            }
            Symbol::Predicate(name.clone(), new_args)
        }
    }
}

/// ## 知识库  
/// 这里将其分为规则rules和事实facts两部分
#[derive(Serialize, Deserialize)]
pub struct KB {
    pub rules: Vec<Rule>,
    pub facts: Vec<Symbol>,
}

impl KB {
    // 在变量后追加编号
    fn index_var(x: &Symbol, i: usize) -> Symbol {
        match x {
            Symbol::Var(name) => var(format!("{name}{i}")),
            Symbol::Predicate(name, args) => {
                let mut new_args = Vec::<Symbol>::new();
                for arg in args.iter() {
                    new_args.push(KB::index_var(&arg.clone(), i))
                }
                pred(name.clone(), new_args)
            }
            _ => x.clone(),
        }
    }
    /// ## 变量名标准化  
    /// 为知识库中每条规则中的变量按照规则序号追加编号
    pub fn standardize_var(&mut self) {
        for (index, rule) in self.rules.iter_mut().enumerate() {
            for condition in rule.condition.iter_mut() {
                *condition = KB::index_var(condition, index);
            }
            rule.conclusion = KB::index_var(&rule.conclusion, index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subst_single() {
        let x = var("X");
        let john = val("john");
        let knows = pred("knows", vec![x.clone(), pred("mother", vec![x.clone()])]);
        let theta = Theta::new(x.clone(), john.clone()).unwrap();

        let y = subst_single(&knows, &theta);
        assert_eq!(
            y,
            pred(
                "knows",
                vec![val("john"), pred("mother", vec![val("john")])]
            )
        );
    }
}
