use super::{KB, ReasoningError, Symbol, Theta};
use crate::unify::unify;

pub fn bc(kb: &KB, theorem: &Symbol, thetas: &mut Vec<Theta>) -> Result<(), ReasoningError> {
    for fact in kb.facts.iter() {
        let mut tmp_theta = thetas.clone();
        if unify(theorem, fact, &mut tmp_theta).is_ok() {
            *thetas = tmp_theta;
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
                println!("要证{theorem:#?}，需证{condition:#?}");
                if bc(kb, condition, &mut curr_theta).is_ok() {
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
        let kb = KB {
            rules: vec![
                Rule {
                    condition: vec![
                        pred("american", vec![var("x1")]),
                        pred("weapon", vec![var("y1")]),
                        pred("sells", vec![var("x1"), var("y1"), var("z1")]),
                        pred("hostile", vec![var("z1")]),
                    ],
                    conclusion: pred("criminal", vec![var("x1")]),
                },
                Rule {
                    condition: vec![
                        pred("missile", vec![var("x2")]),
                        pred("owns", vec![val("nono"), var("x2")]),
                    ],
                    conclusion: pred("sells", vec![val("west"), var("x2"), val("nono")]),
                },
                Rule {
                    condition: vec![pred("missile", vec![var("x3")])],
                    conclusion: pred("weapon", vec![var("x3")]),
                },
                Rule {
                    condition: vec![pred("enemy", vec![var("x4"), val("america")])],
                    conclusion: pred("hostile", vec![var("x4")]),
                },
            ],
            facts: vec![
                pred("owns", vec![val("nono"), val("m1")]),
                pred("missile", vec![val("m1")]),
                pred("american", vec![val("west")]),
                pred("enemy", vec![val("nono"), val("america")]),
            ],
        };
        let theorem_true = pred("criminal", vec![val("west")]);
        let mut thetas = Vec::<Theta>::new();
        println!("start");
        bc(&kb, &theorem_true, &mut thetas).unwrap();
    }
}
