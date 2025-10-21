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
    let proof = bc_args(
        kb,
        &wrapped_theorem,
        &mut thetas,
        verbose,
        &mut call_time,
        0,
        max_depth,
    );
    println!("证明步数：{call_time}");
    proof
}

struct Ckpt {
    theorems: Vec<Symbol>,
    thetas: Vec<Theta>,
    rule: Rule,
}

fn get_prove_path(
    rules: &[Rule],
    theorem: &Symbol,
    verbose: bool,
    thetas: &[Theta],
) -> Result<Vec<Ckpt>, ReasoningError> {
    let mut to_prove_list = Vec::<Ckpt>::new();
    for rule in rules.iter() {
        let mut tmp_thetas: Vec<Theta> = thetas.to_owned();
        if unify(theorem, &rule.conclusion, &mut tmp_thetas).is_ok() {
            if verbose {
                println!("规则{rule:?}可用于证明{theorem}");
            }
            to_prove_list.push(Ckpt {
                theorems: rule.condition.clone(),
                thetas: tmp_thetas,
                rule: rule.clone(),
            });
        }
    }
    if to_prove_list.is_empty() {
        Err(ReasoningError::ProofNotFound)
    } else {
        Ok(to_prove_list)
    }
}

fn bc_args(
    kb: &KB,
    theorems: &[Symbol],
    thetas: &mut Vec<Theta>,
    verbose: bool,
    call_time: &mut usize,
    depth: usize,
    max_depth: usize,
) -> Result<(), ReasoningError> {
    if theorems.is_empty() {
        return Ok(());
    }

    println!("------当前深度：{depth}-------");
    *call_time += 1;
    let head = &theorems[0];
    let rest = &theorems[1..];
    let subst_theorem = exhaust_subst(head, thetas);
    if depth > max_depth {
        if verbose {
            println!("尝试证明{subst_theorem}时深度超限，回退");
        }
        return Err(ReasoningError::DepthLimitExceed);
    }
    if verbose {
        println!("对{subst_theorem}的证明：");
    }
    let rules: Vec<Rule> = kb
        .rules
        .iter()
        .map(|r| KB::rule_standardize(r, *call_time))
        .collect();
    if let Ok(prove_paths) = get_prove_path(&rules, &subst_theorem, verbose, thetas) {
        for path in prove_paths {
            println!("为证明{}使用规则{:?}", subst_theorem, path.rule);
            let mut tmp_thetas = path.thetas.clone();
            if bc_args(
                kb,
                &path.theorems,
                &mut tmp_thetas,
                verbose,
                call_time,
                depth + 1,
                max_depth,
            )
            .is_ok()
            {
                println!(
                    "{}的前提条件可能得到满足，下面证明其相关命题：{:?}",
                    subst_theorem, rest
                );
                println!("------");
                if bc_args(
                    kb,
                    rest,
                    &mut tmp_thetas,
                    verbose,
                    call_time,
                    depth,
                    max_depth,
                )
                .is_ok()
                {
                    println!("{}得到了证明", subst_theorem);
                    println!("------");
                    *thetas = tmp_thetas;
                    return Ok(());
                } else {
                    println!("{}的路径{:?}失败", subst_theorem, path.rule);
                    println!("------");
                }
            } else {
                println!("{}的前提条件不满足", subst_theorem);
                println!("------");
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
