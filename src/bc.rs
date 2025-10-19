use super::{KB, ReasoningError, Symbol, Theta};
use crate::unify::{exhaust_subst, unify};

pub fn bc(
    kb: &KB,
    theorem: &Symbol,
    thetas: &mut Vec<Theta>,
    verbose: bool,
    call_stack: &mut Vec<Symbol>,
    depth: usize,
    max_depth: usize,
) -> Result<(), ReasoningError> {
    // for theta in thetas.iter() {
    //     println!("\r\n{}:{}\r\n", theta.origin, theta.result);
    // }
    if call_stack.contains(theorem) {
        if verbose {
            let subst_theorem = exhaust_subst(&theorem, thetas);
            println!("检测到循环目标 {subst_theorem}，回退");
        }
        return Err(ReasoningError::UnifyError);
    }
    if depth > max_depth {
        return Err(ReasoningError::UnifyError);
    }
    call_stack.push(theorem.clone());
    if verbose {
        let subst_theorem = exhaust_subst(&theorem, thetas);
        println!("对{subst_theorem}的证明：");
    }
    for fact in kb.facts.iter() {
        let base_theta = thetas.clone();
        let all_paths = unify(theorem, fact, &base_theta);
        for path_theta in all_paths {
            *thetas = path_theta;
            if verbose {
                println!("{fact}是已知条件，证毕。");
            }
            return Ok(());
        }
    }
    for rule in kb.rules.iter() {
        let base_theta = thetas.clone();
        let all_paths = unify(theorem, &rule.conclusion, &base_theta);
        // eprintln!("\r\nwith {:?} => {}\r\n", &rule.condition, &rule.conclusion);
        // eprintln!("\r\nunify_all( {},  {} )\r\n", theorem, &rule.conclusion,);
        for mut path_theta in all_paths {
            let mut proved = true;
            // println!("{:?}", rule.condition);
            for condition in rule.condition.iter() {
                if verbose {
                    let subst_theorem = exhaust_subst(&theorem, thetas);
                    let subst_condition = exhaust_subst(condition, &path_theta);
                    println!("要证{subst_theorem}，需证{subst_condition}");
                }
                if bc(
                    kb,
                    condition,
                    &mut path_theta,
                    verbose,
                    call_stack,
                    depth + 1,
                    max_depth,
                )
                .is_err()
                {
                    proved = false;
                    break;
                }
            }
            if proved {
                *thetas = path_theta;
                // let subst_theorem = exhaust_subst(&theorem, thetas);
                // println!("{subst_theorem}证明完毕");
                return Ok(());
            }
        }
    }
    Err(ReasoningError::UnifyError)
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
            ],
            facts: vec![
                pred("owns", vec![val("nono"), val("m1")]),
                pred("missile", vec![val("m1")]),
                pred("american", vec![val("west")]),
                pred("enemy", vec![val("nono"), val("america")]),
            ],
        };
        kb.standardize_var();
        let theorem_true = pred("criminal", vec![val("west")]);
        let mut thetas = Vec::<Theta>::new();
        let mut stack = Vec::<Symbol>::new();
        bc(&kb, &theorem_true, &mut thetas, true, &mut stack, 0, 100).unwrap();
    }
}
