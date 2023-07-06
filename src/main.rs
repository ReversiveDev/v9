use javascript::interpreter::{values::Value, Context, Script};
use std::time::Instant;

mod javascript;

fn main() {
    // let code = "class Foo {\n    constructor() {}\n}";
    let code = "10 + 10 + 10";

    let start = Instant::now();
    let ctx = Context {};
    let value = Script::new(code).run_in_context(&ctx);

    let duration = start.elapsed();
    println!("Time: {} microsegundos", duration.as_micros());

    match value {
        Value::Number(n) => println!("{n}"),
        _ => panic!("Expected a number"),
    }
}
