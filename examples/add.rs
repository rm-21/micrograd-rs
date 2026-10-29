use micrograd_rs::engine::Value;

fn main() {
    let a = Value::from(3.0);
    let b = Value::from(2.0);

    println!("Value + Value: {a} + {b} = {}", a + b);
    println!("Value + f64:   {a} + 1.5 = {}", a + 1.5);
    println!("f64 + Value:   1.5 + {a} = {}", 1.5 + a);
    println!("chained:       {a} + {b} + 10 = {}", a + b + 10.0);
}
