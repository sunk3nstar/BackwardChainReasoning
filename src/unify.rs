use crate::{Atom, ReasoningError, Symbol, Theta, func};

/// 合一项
fn unify_symbol(x: &Symbol, y: &Symbol, theta_list: &mut Vec<Theta>) -> Result<(), ReasoningError> {
    if x == y {
        return Ok(());
    } else if let Symbol::Var(_) = x {
        return unify_var(x, y, theta_list);
    } else if let Symbol::Var(_) = y {
        return unify_var(y, x, theta_list);
    } else if let Symbol::Func(x_name, x_args) = x
        && let Symbol::Func(y_name, y_args) = y
    {
        if x_name != y_name {
            return Err(ReasoningError::UnifyError);
        } else {
            for (x_arg, y_arg) in x_args.iter().zip(y_args.iter()) {
                unify_symbol(x_arg, y_arg, theta_list)?;
            }
            return Ok(());
        }
    }
    Err(ReasoningError::UnifyError)
}

/// 合一谓词
pub fn unify(x: &Atom, y: &Atom, theta_list: &mut Vec<Theta>) -> Result<(), ReasoningError> {
    if x.predicate == y.predicate {
        for (x_arg, y_arg) in x.args.iter().zip(y.args.iter()) {
            unify_symbol(x_arg, y_arg, theta_list)?;
        }
        return Ok(());
    }
    Err(ReasoningError::UnifyError)
}

/// 单变量合一
fn unify_var(var: &Symbol, x: &Symbol, theta_list: &mut Vec<Theta>) -> Result<(), ReasoningError> {
    if let Some(val) = subst_known(var, theta_list) {
        unify_symbol(&val, x, theta_list)?;
    } else if let Some(val) = subst_known(x, theta_list) {
        unify_symbol(var, &val, theta_list)?;
    } else {
        theta_list.push(Theta::new(var.clone(), x.clone()).unwrap());
    }
    Ok(())
}

/// 在已知的置换列表中找到一个变量x的置换结果
fn subst_known(x: &Symbol, theta_list: &[Theta]) -> Option<Symbol> {
    if let Symbol::Var(name) = x {
        let theta_iter = theta_list.iter();
        for theta in theta_iter {
            match theta.origin {
                Symbol::Var(ref existed_name) if existed_name == name => {
                    return Some(theta.result.clone());
                }
                _ => {}
            }
        }
    }
    None
}

/// 使用已知的置换列表反复作用于项x直至无法再被置换
fn exhaust_subst_symbol(x: &Symbol, theta_list: &[Theta]) -> Symbol {
    match x {
        Symbol::Var(_) => {
            if let Some(new_x) = subst_known(x, theta_list) {
                exhaust_subst_symbol(&new_x, theta_list)
            } else {
                x.clone()
            }
        }
        Symbol::Val(_) => x.clone(),
        Symbol::Func(name, args) => {
            let mut new_args = Vec::<Symbol>::new();
            for arg in args.iter() {
                new_args.push(exhaust_subst_symbol(arg, theta_list));
            }
            func(name.clone(), new_args)
        }
    }
}

/// 使用已知的置换列表反复作用于项x直至无法再被置换
pub fn exhaust_subst(x: &Atom, theta_list: &[Theta]) -> Atom {
    Atom {
        predicate: x.predicate.clone(),
        args: x
            .args
            .iter()
            .map(|arg| exhaust_subst_symbol(arg, theta_list))
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{func, val, var};
    #[test]
    fn test_unify_sym() {
        let a = func("add", vec![val("zero"), var("x")]);
        let b = func(
            "add",
            vec![var("y"), func("add", vec![var("zero"), var("zero")])],
        );
        let mut thetas = Vec::<Theta>::new();
        unify_symbol(&a, &b, &mut thetas).unwrap();
        assert_eq!(
            exhaust_subst_symbol(&a, &thetas),
            exhaust_subst_symbol(&b, &thetas)
        );
    }
}
