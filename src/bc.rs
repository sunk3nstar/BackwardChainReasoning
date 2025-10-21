use super::{KB, ReasoningError, Symbol, Theta};
use crate::{
    Rule,
    unify::{exhaust_subst, unify},
};

pub fn bc(
    kb: &KB,
    theorem: &Symbol,
    verbose: bool,
    max_depth: usize,
) -> Result<(), ReasoningError> {
    let mut thetas = Vec::<Theta>::new();
    let mut call_time = 0;
    let wrapped_theorem = vec![theorem.clone()];
    let mut call_stack = Vec::<Symbol>::new();
    let proof = bc_core(
        kb,
        &wrapped_theorem,
        &mut thetas,
        verbose,
        &mut call_time,
        &mut call_stack,
        0,
        max_depth,
    );
    if verbose {
        println!("证明步数：{call_time}");
    }
    proof
}

/// 暂存发现的子命题
struct Ckpt {
    theorems: Vec<Symbol>,
    thetas: Vec<Theta>,
}

fn get_prove_path(
    rules: &[Rule],
    theorem: &Symbol,
    thetas: &[Theta],
) -> Result<Vec<Ckpt>, ReasoningError> {
    let mut to_prove_list = Vec::<Ckpt>::new();
    for rule in rules.iter() {
        let mut tmp_thetas: Vec<Theta> = thetas.to_owned();
        if unify(theorem, &rule.conclusion, &mut tmp_thetas).is_ok() {
            to_prove_list.push(Ckpt {
                theorems: rule.condition.clone(),
                thetas: tmp_thetas,
            });
        }
    }
    if to_prove_list.is_empty() {
        Err(ReasoningError::ProofNotFound)
    } else {
        Ok(to_prove_list)
    }
}

fn bc_core(
    kb: &KB,
    theorems: &[Symbol],
    thetas: &mut Vec<Theta>,
    verbose: bool,
    call_time: &mut usize,
    call_stack: &mut Vec<Symbol>,
    depth: usize,
    max_depth: usize,
) -> Result<(), ReasoningError> {
    if theorems.is_empty() {
        return Ok(());
    }
    if verbose {
        println!("------当前深度：{depth}-------");
    }
    let head = &theorems[0];
    let rest = &theorems[1..];
    let subst_theorem = exhaust_subst(head, thetas);
    if call_stack.contains(&subst_theorem) {
        if verbose {
            println!("证明{subst_theorem}是循环论证，回退");
        }
        return Err(ReasoningError::DepthLimitExceed);
    }
    if depth > max_depth {
        if verbose {
            println!("尝试证明{subst_theorem}时深度超限，回退");
        }
        return Err(ReasoningError::DepthLimitExceed);
    }
    if verbose {
        println!("对{subst_theorem}的证明：");
    }
    *call_time += 1;
    let rules: Vec<Rule> = kb
        .rules
        .iter()
        .map(|r| KB::rule_standardize(r, *call_time))
        .collect();
    if let Ok(prove_paths) = get_prove_path(&rules, &subst_theorem, thetas) {
        for path in prove_paths {
            let mut tmp_thetas = path.thetas.clone();
            if bc_core(
                kb,
                &path.theorems,
                &mut tmp_thetas,
                verbose,
                call_time,
                call_stack,
                depth + 1,
                max_depth,
            )
            .is_ok()
            {
                if bc_core(
                    kb,
                    rest,
                    &mut tmp_thetas,
                    verbose,
                    call_time,
                    call_stack,
                    depth,
                    max_depth,
                )
                .is_ok()
                {
                    println!("{}得到了证明", subst_theorem);
                    *thetas = tmp_thetas;
                    call_stack.pop();
                    return Ok(());
                }
            }
        }
    }
    Err(ReasoningError::ProofNotFound)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Rule, pred, val, var};
    #[test]
    fn test_bc() {
        let mut kb = KB {
            rules: vec![
                Rule {
                    condition: vec![
                        pred("american", vec![var("x")]),
                        pred("weapon", vec![var("y")]),
                        pred("sells", vec![var("x"), var("y"), var("z")]),
                        pred("hostile", vec![var("z")]),
                    ],
                    conclusion: pred("criminal", vec![var("x")]),
                },
                Rule {
                    condition: vec![
                        pred("missile", vec![var("x")]),
                        pred("owns", vec![val("nono"), var("x")]),
                    ],
                    conclusion: pred("sells", vec![val("west"), var("x"), val("nono")]),
                },
                Rule {
                    condition: vec![pred("missile", vec![var("x")])],
                    conclusion: pred("weapon", vec![var("x")]),
                },
                Rule {
                    condition: vec![pred("enemy", vec![var("x"), val("america")])],
                    conclusion: pred("hostile", vec![var("x")]),
                },
                Rule {
                    condition: vec![],
                    conclusion: pred("owns", vec![val("nono"), val("m1")]),
                },
                Rule {
                    condition: vec![],
                    conclusion: pred("missile", vec![val("m1")]),
                },
                Rule {
                    condition: vec![],
                    conclusion: pred("american", vec![val("west")]),
                },
                Rule {
                    condition: vec![],
                    conclusion: pred("enemy", vec![val("nono"), val("america")]),
                },
            ],
        };
        kb.standardize_var();
        let theorem_true = pred("criminal", vec![val("west")]);
        println!("start");
        bc(&kb, &theorem_true, true, 10).unwrap();
    }
}
