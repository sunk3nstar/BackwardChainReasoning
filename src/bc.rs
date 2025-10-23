use super::{Atom, KB, ReasoningError, Rule, Theta};
use crate::unify::{exhaust_subst, unify};

/// 反向链接推理器
pub fn bc(kb: &KB, theorem: &Atom, verbose: bool, max_depth: usize) -> Result<(), ReasoningError> {
    let mut thetas = Vec::<Theta>::new();
    let mut call_time = 0;
    let wrapped_theorem = vec![theorem.clone()];
    let mut call_stack = Vec::<Atom>::new();
    let mut known_facts = Vec::<Atom>::new();
    for rule in kb.rules.iter() {
        if rule.is_fact() {
            known_facts.push(rule.conclusion.clone());
        }
    }
    let proof = bc_core(
        kb,
        &wrapped_theorem,
        &mut thetas,
        verbose,
        &mut call_time,
        &mut call_stack,
        0,
        max_depth,
        &mut known_facts,
    );
    if verbose {
        println!("证明步数：{call_time}");
    }
    proof
}

/// 暂存发现的子命题
struct Ckpt {
    theorems: Vec<Atom>,
    thetas: Vec<Theta>,
}

/// 对于一条命题，找到所有能与其合一的规则结论，记录结论需要的条件和使用的替换
fn get_prove_path(
    rules: &[Rule],
    theorem: &Atom,
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

/// ## 证明一系列关联命题的反向链接算法
/// 其中每条命题的前提包含排在其之前的所有命题
/// 如：为证明0<9，找到的一条可行路径需要证明存在x使得0<x且x<9
/// 因此需要对命题序列0<x,x<9调用该函数
/// 函数在通过找到x=x_0证明0<x后不立刻认为0<x得证
/// 而是将x=x_0代入x<9
/// 如果x_0不满足x<9则认为证明失败，算法回退采取其他可行路径证明0<9
/// 当然如果x_0满足了x<9证明就成功了。
fn bc_core(
    kb: &KB,
    theorems: &[Atom],
    thetas: &mut Vec<Theta>,
    verbose: bool,
    call_time: &mut usize,
    call_stack: &mut Vec<Atom>,
    depth: usize,
    max_depth: usize,
    facts: &mut Vec<Atom>,
) -> Result<(), ReasoningError> {
    if theorems.is_empty() {
        return Ok(());
    }
    let head = &theorems[0];
    let rest = &theorems[1..];
    let subst_theorem = exhaust_subst(head, thetas);
    if facts.contains(&subst_theorem) {
        return bc_core(
            kb, rest, thetas, verbose, call_time, call_stack, depth, max_depth, facts,
        );
    }
    if call_stack.contains(&subst_theorem) {
        if verbose {
            eprintln!("证明{subst_theorem}是循环论证，回退");
        }
        return Err(ReasoningError::CycleProof);
    }
    if depth > max_depth {
        if verbose {
            eprintln!("尝试证明{subst_theorem}时深度超限，回退");
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
                facts,
            )
            .is_ok()
                && bc_core(
                    kb,
                    rest,
                    &mut tmp_thetas,
                    verbose,
                    call_time,
                    call_stack,
                    depth,
                    max_depth,
                    facts,
                )
                .is_ok()
            {
                if verbose {
                    println!("{}得到了证明", subst_theorem);
                }
                if !subst_theorem.contains_var() {
                    facts.push(subst_theorem.clone());
                }
                *thetas = tmp_thetas;
                call_stack.pop();
                return Ok(());
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
    fn test_bc_example1() {
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
        let json = serde_json::to_string_pretty(&kb).unwrap();
        std::fs::write("knowledge_base.json", json).unwrap();
        println!("start");
        bc(&kb, &theorem_true, true, 5).unwrap();
    }
}
