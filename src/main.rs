use micrograd_rs::engine::Value;

fn main() {
    let x = Value { data: 3.0 };
    let y = Value { data: 2.0 };
    println!("x: {} + y: {} = {}", x.clone(), y.clone(), x + y);
}
