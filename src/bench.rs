use crate::bc::bc;
use crate::{KB, Rule, func, pred, val, var};

pub fn bench_bc_math() {
    let mut kb = KB {
        rules: vec![
            Rule {
                condition: vec![],
                conclusion: pred("leq", vec![val("zero"), val("three")]),
            },
            Rule {
                condition: vec![],
                conclusion: pred("leq", vec![val("seven"), val("nine")]),
            },
            Rule {
                condition: vec![],
                conclusion: pred(
                    "leq",
                    vec![var("x"), func("add", vec![var("x"), val("zero")])],
                ),
            },
            Rule {
                condition: vec![],
                conclusion: pred(
                    "leq",
                    vec![func("add", vec![var("x"), val("zero")]), var("x")],
                ),
            },
            Rule {
                condition: vec![
                    pred("leq", vec![var("x"), var("y")]),
                    pred("leq", vec![var("y"), var("z")]),
                ],
                conclusion: pred("leq", vec![var("x"), var("z")]),
            },
            Rule {
                condition: vec![
                    pred("leq", vec![var("w"), var("y")]),
                    pred("leq", vec![var("x"), var("z")]),
                ],
                conclusion: pred(
                    "leq",
                    vec![
                        func("add", vec![var("w"), var("x")]),
                        func("add", vec![var("y"), var("z")]),
                    ],
                ),
            },
            Rule {
                condition: vec![],
                conclusion: pred("leq", vec![var("x"), var("x")]),
            },
            Rule {
                condition: vec![],
                conclusion: pred(
                    "leq",
                    vec![
                        func("add", vec![var("x"), var("y")]),
                        func("add", vec![var("y"), var("x")]),
                    ],
                ),
            },
        ],
    };
    kb.standardize_var();
    let theorem_true = pred(
        "leq",
        vec![val("seven"), func("add", vec![val("three"), val("nine")])],
    );
    // let json1 = serde_json::to_string_pretty(&kb).unwrap();
    // std::fs::write("math.json", json1).unwrap();
    // let json2 = serde_json::to_string_pretty(&theorem_true).unwrap();
    // std::fs::write("math_theorem.json", json2).unwrap();
    bc(&kb, &theorem_true, false, 5).unwrap();
}
