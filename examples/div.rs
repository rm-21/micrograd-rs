use micrograd_rs::engine::Value;

fn main() {
    let a = Value::from(6.0);
    let b = Value::from(2.0);

    // Div is built on Mul and pow: a / b == a * b^-1
    println!("Value / Value: {a} / {b} = {}", a / b);
    println!("Value / f64:   {a} / 4 = {}", a / 4.0);
    // operand order is preserved: 3 / a, not a / 3
    println!("f64 / Value:   3 / {a} = {}", 3.0 / a);
    println!("by zero:       {a} / 0 = {}", a / 0.0);
}
