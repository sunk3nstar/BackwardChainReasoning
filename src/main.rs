use reasoning::{pred, val, var};

fn main() {
    let a = pred("is", vec![var("X"), val("animal")]);
    println!("{a:#?}");
}
