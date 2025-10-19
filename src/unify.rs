use crate::{ReasoningError, Symbol, Theta, pred};

/// 合一
pub fn unify(x: &Symbol, y: &Symbol, theta_list: &Vec<Theta>) -> Vec<Vec<Theta>> {
    if x == y {
        return vec![theta_list.clone()];
    } else if let Symbol::Var(_) = x {
        return unify_var(x, y, theta_list);
    } else if let Symbol::Var(_) = y {
        return unify_var(y, x, theta_list);
    } else if let Symbol::Predicate(x_name, x_args) = x
        && let Symbol::Predicate(y_name, y_args) = y
        && x_name == y_name
        && x_args.len() == y_args.len()
    {
        return unify_args(x_args, y_args, theta_list);
    }
    vec![]
}

/// 单变量合一
fn unify_var(var: &Symbol, x: &Symbol, theta_list: &Vec<Theta>) -> Vec<Vec<Theta>> {
    if let Some(val) = subst_known(var, theta_list) {
        unify(&val, x, theta_list)
    } else if let Some(val) = subst_known(x, theta_list) {
        unify(var, &val, theta_list)
    } else {
        let mut new_theta = theta_list.clone();
        new_theta.push(Theta::new(var.clone(), x.clone()).unwrap());
        vec![new_theta]
    }
}

fn unify_args(x_args: &[Symbol], y_args: &[Symbol], theta_list: &Vec<Theta>) -> Vec<Vec<Theta>> {
    if x_args.is_empty() {
        return vec![theta_list.clone()];
    }

    let mut results = Vec::new();

    let x_head = &x_args[0];
    let y_head = &y_args[0];
    let x_tail = &x_args[1..];
    let y_tail = &y_args[1..];

    for thetas in unify(x_head, y_head, theta_list) {
        for theta_res in unify_args(x_tail, y_tail, &thetas) {
            results.push(theta_res);
        }
    }

    results
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

/// 使用已知的置换列表反复作用于x直至无法再被置换
pub fn exhaust_subst(x: &Symbol, theta_list: &[Theta]) -> Symbol {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{pred, val, var};
    #[test]
    fn test_unify() {
        let a = pred("know", vec![val("john"), var("x")]);
        let b = pred("know", vec![var("y"), pred("mother", vec![var("y")])]);
        let thetas = Vec::<Theta>::new();
        let res_theta = unify(&a, &b, &thetas)[0].clone();
        assert_eq!(exhaust_subst(&a, &res_theta), exhaust_subst(&b, &res_theta));
    }
}
