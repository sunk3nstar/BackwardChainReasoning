use super::{KB, ReasoningError, Symbol, Theta};
use crate::unify::{exhaust_subst, unify};

pub fn bc(
    kb: &KB,
    theorem: &Symbol,
    thetas: &mut Vec<Theta>,
    verbose: bool,
) -> Result<(), ReasoningError> {
    if verbose {
        println!("对{theorem}的证明：");
    }
    for fact in kb.facts.iter() {
        let mut tmp_theta = thetas.clone();
        if unify(theorem, fact, &mut tmp_theta).is_ok() {
            *thetas = tmp_theta;
            if verbose {
                println!("{fact}是已知条件，证毕。");
            }
            return Ok(());
        }
    }
    for rule in kb.rules.iter() {
        let mut tmp_theta = thetas.clone();
        if unify(theorem, &rule.conclusion, &mut tmp_theta).is_ok() {
            *thetas = tmp_theta.clone();
            let mut proved = true;
            for condition in rule.condition.iter() {
                let mut curr_theta = tmp_theta.clone();
                if verbose {
                    let subst_condition = exhaust_subst(condition, &curr_theta);
                    println!("要证{theorem}，需证{subst_condition}");
                }
                if bc(kb, condition, &mut curr_theta, verbose).is_ok() {
                    *thetas = curr_theta;
                } else {
                    proved = false;
                    break;
                }
            }
            if proved {
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
        println!("start");
        bc(&kb, &theorem_true, &mut thetas, true).unwrap();
    }
}
