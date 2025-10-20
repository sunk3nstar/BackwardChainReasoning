use super::{KB, ReasoningError, Symbol, Theta};
use crate::{
    Rule,
    unify::{exhaust_subst, unify},
};
use rand::seq::SliceRandom;

pub fn bc(
    kb: &KB,
    theorem: &Symbol,
    verbose: bool,
    max_depth: usize,
) -> Result<(), ReasoningError> {
    let mut thetas = Vec::<Theta>::new();
    let mut stack = Vec::<Symbol>::new();
    let mut times: usize = 0;
    let proof = bc_core(
        kb,
        theorem,
        &mut thetas,
        verbose,
        &mut stack,
        0,
        max_depth,
        &mut times,
    );
    println!("总步数：{times}");
    proof
}
fn bc_core(
    kb: &KB,
    theorem: &Symbol,
    thetas: &mut Vec<Theta>,
    verbose: bool,
    call_stack: &mut Vec<Symbol>,
    depth: usize,
    max_depth: usize,
    call_time: &mut usize,
) -> Result<(), ReasoningError> {
    // for theta in thetas.iter() {
    //     println!("\r\n{}:{}\r\n", theta.origin, theta.result);
    // }
    *call_time += 1;
    let subst_theorem = exhaust_subst(&theorem, thetas);
    if call_stack.contains(&subst_theorem) {
        if verbose {
            println!("检测到循环目标 {subst_theorem}，回退");
        }
        return Err(ReasoningError::UnifyError);
    }
    if depth > max_depth {
        println!("深度{depth}超限");
        return Err(ReasoningError::DepthLimitExceed);
    }
    call_stack.push(subst_theorem.clone());
    if verbose {
        println!("对{subst_theorem}的证明：");
        println!("\r\n当前替换：{:?}\r\n", thetas);
    }
    let mut rules: Vec<Rule> = kb
        .rules
        .iter()
        .map(|rule| KB::rule_standardize(rule, *call_time))
        .collect();
    // let mut rng = rand::rng();
    // rules.shuffle(&mut rng);
    for rule in rules.iter() {
        let base_theta = thetas.clone();
        let all_paths = unify(theorem, &rule.conclusion, &base_theta);
        // eprintln!("\r\nwith {:?} => {}\r\n", &rule.condition, &rule.conclusion);
        // eprintln!("\r\nunify_all( {},  {} )\r\n", theorem, &rule.conclusion,);
        if verbose {
            println!("\r\n要证{subst_theorem}，尝试规则：{:?}\r\n", rule);
        }
        for mut path_theta in all_paths {
            let condition_num = rule.condition.len();
            let mut proved_num: usize = 0;
            // println!("{:?}", rule.condition);
            for condition in rule.condition.iter() {
                let subst_theorem = exhaust_subst(&theorem, thetas);
                let subst_condition = exhaust_subst(condition, &path_theta);
                if verbose {
                    println!(
                        "\r\n要证{subst_theorem}\r\n根据规则{rule:?}\r\n需证{subst_condition}\r\n"
                    );
                    // println!("\r\n当前调用栈：{:?}\r\n", call_stack);
                    println!("\r\n当前深度：{:?}\r\n", depth);
                }
                if bc_core(
                    kb,
                    &subst_condition,
                    &mut path_theta,
                    verbose,
                    call_stack,
                    depth + 1,
                    max_depth,
                    call_time,
                )
                .is_ok()
                {
                    proved_num += 1;
                } else {
                    break;
                }
            }
            if proved_num == condition_num {
                *thetas = path_theta;
                if rule.condition.is_empty() {
                    println!("在替换{thetas:?}下这是一条已知事实。")
                }
                println!("{subst_theorem}证明完毕");
                call_stack.pop();
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
            facts: vec![],
        };
        kb.standardize_var();
        let theorem_true = pred("criminal", vec![val("west")]);

        bc(&kb, &theorem_true, true, 100).unwrap();
    }
}
