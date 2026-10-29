use micrograd_rs::engine::Value;

fn main() {
    let x = Value { data: 3.0 };
    let y = Value { data: 2.0 };
    let z: f64 = 10.1234;

    println!(
        "x: {} + y: {} + z: {} = {}",
        x.clone(),
        y.clone(),
        z,
        x + y + z
    );
    println!(
        "x: {} * y: {} * z: {} = {}",
        x.clone(),
        y.clone(),
        z,
        x * y * z
    );
}
