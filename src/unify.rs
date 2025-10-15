use crate::{ReasoningError, Symbol, Theta};

/// 合一
pub fn unify(x: &Symbol, y: &Symbol, theta_list: &mut Vec<Theta>) -> Result<(), ReasoningError> {
    if x == y {
        return Ok(());
    } else if let Symbol::Var(_) = x {
        return unify_var(x, y, theta_list);
    } else if let Symbol::Var(_) = y {
        return unify_var(y, x, theta_list);
    } else if let Symbol::Predicate(x_name, x_args) = x
        && let Symbol::Predicate(y_name, y_args) = y
    {
        if x_name != y_name {
            return Err(ReasoningError::UnifyError);
        } else {
            for (x_arg, y_arg) in x_args.iter().zip(y_args.iter()) {
                unify(x_arg, y_arg, theta_list)?;
            }
            return Ok(());
        }
    }
    Err(ReasoningError::UnifyError)
}

/// 单变量合一
fn unify_var(var: &Symbol, x: &Symbol, theta_list: &mut Vec<Theta>) -> Result<(), ReasoningError> {
    if let Some(val) = subst_known(var, theta_list) {
        unify(&val, x, theta_list)?;
    } else if let Some(val) = subst_known(x, theta_list) {
        unify(var, &val, theta_list)?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pred, val, var};
    /// 使用已知的置换列表反复作用于x直至无法再被置换
    fn exhaust_subst(x: &Symbol, theta_list: &[Theta]) -> Symbol {
        match x {
            Symbol::Var(_) => {
                if let Some(new_x) = subst_known(x, theta_list) {
                    exhaust_subst(&new_x, theta_list)
                } else {
                    x.clone()
                }
            }
            Symbol::Val(_) => x.clone(),
            Symbol::Predicate(name, args) => {
                let mut new_args = Vec::<Symbol>::new();
                for arg in args.iter() {
                    new_args.push(exhaust_subst(arg, theta_list));
                }
                pred(name.clone(), new_args)
            }
        }
    }
    #[test]
    fn test_unify() {
        let a = pred("know", vec![val("john"), var("x")]);
        let b = pred("know", vec![var("y"), pred("mother", vec![var("y")])]);
        let mut thetas = Vec::<Theta>::new();
        unify(&a, &b, &mut thetas).unwrap();
        assert_eq!(exhaust_subst(&a, &thetas), exhaust_subst(&b, &thetas));
    }
}
