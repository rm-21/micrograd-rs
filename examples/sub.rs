use micrograd_rs::engine::Value;

fn main() {
    let a = Value::from(6.0);
    let b = Value::from(2.0);

    // Sub is built on Add and Neg: a - b == a + (-b)
    println!("Value - Value: {a} - {b} = {}", a - b);
    println!("Value - f64:   {a} - 1.5 = {}", a - 1.5);
    // operand order is preserved: 10 - a, not a - 10
    println!("f64 - Value:   10 - {a} = {}", 10.0 - a);
    println!("chained:       {a} - {b} - 1 = {}", a - b - 1.0);
}
