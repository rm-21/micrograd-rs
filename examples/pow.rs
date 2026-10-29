use micrograd_rs::engine::Value;

fn main() {
    let a = Value::from(4.0);

    println!("{a}^2    = {}", a.pow(2.0));
    println!("{a}^0.5  = {}", a.pow(0.5));
    println!("{a}^-1   = {}", a.pow(-1.0));
    println!("{a}^0    = {}", a.pow(0.0));
}
