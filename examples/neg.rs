use micrograd_rs::engine::Value;

fn main() {
    let a = Value::from(3.0);
    let b = Value::from(-2.0);

    // Neg is built on Mul: -a == a * -1.0
    println!("-{a} = {}", -a);
    println!("-{b} = {}", -b);
    println!("double negation: --{a} = {}", -(-a));
}
