//! 推理算法

/// 逻辑语句
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
    fn subst(&self, theta_list: &Vec<Theta>) -> Self
    where
        Self: Sized;
}

/// 错误
#[derive(Debug)]
pub enum ReasoningError {
    ThetaError,
    UnifyError,
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
            ReasoningError::UnifyError => {
                write!(f, "No available unification found.")
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
/// let is = pred("is", vec!(x.clone(), val("number")));
/// ```
#[derive(Debug, Clone)]
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

impl Statement for Symbol {
    fn subst(&self, theta_list: &Vec<Theta>) -> Self
    where
        Self: Sized,
    {
        match *self {
            Symbol::Val(_) => self.clone(),
            _ => {
                let mut new = self.clone();
                for theta in theta_list {
                    new = subst_single(&new, &theta);
                }
                new
            }
        }
    }
}

/// 规则（一阶确定子句）  
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
#[derive(Debug)]
pub struct Rule {
    pub condition: Vec<Symbol>,
    pub conclusion: Symbol,
}

impl Statement for Rule {
    fn subst(&self, theta_list: &Vec<Theta>) -> Self
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

/// 逻辑置换记号
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

fn subst_single(src: &Symbol, theta: &Theta) -> Symbol {
    match src {
        Symbol::Val(_) => src.clone(),
        Symbol::Var(ref x) => match &theta.origin {
            Symbol::Var(name) if name == x => theta.result.clone(),
            _ => src.clone(),
        },
        Symbol::Predicate(ref name, args) => {
            let mut new_args = vec![];
            for arg in args {
                new_args.push(subst_single(arg, theta));
            }
            Symbol::Predicate(name.clone(), new_args)
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
        match y {
            Symbol::Predicate(ref name, ref args) => {
                assert_eq!(name, "knows");
                if let Symbol::Val(ref val1) = args[0] {
                    assert_eq!(val1, "john");
                } else {
                    panic!("wrong arg1 type, expect Val");
                }
                match args[1] {
                    Symbol::Predicate(ref name, ref aargs) => {
                        assert_eq!(name, "mother");
                        if let Symbol::Val(ref vval) = aargs[0] {
                            assert_eq!(vval, "john");
                        } else {
                            panic!("wrong arg2 inner arg type, expect Val");
                        }
                        assert_eq!(aargs.len(), 1);
                    }
                    _ => panic!("wrong arg2 type, expect Predicate"),
                }
                assert_eq!(args.len(), 2);
            }
            _ => panic!("wrong result type, expect Predicate"),
        }
    }
}

/// 合一
// fn unify(
//     x: &impl Statement,
//     y: &impl Statement,
//     theta_list: &Vec<Theta>,
// ) -> Result<Vec<Theta>, ReasoningError> {
//     match
// }

/// 知识库
/// 这里将其分为规则rules和事实facts两部分
pub struct KB {
    rules: Vec<Rule>,
    facts: Vec<Symbol>,
}
