use crate::bc::bc;
use crate::{KB, Rule, func, pred, val, var};

pub fn bench_bc_math() {
    let kb = KB {
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
    let theorem_true = pred(
        "leq",
        vec![val("seven"), func("add", vec![val("three"), val("nine")])],
    );
    bc(&kb, &theorem_true, false, 5).unwrap();
}
