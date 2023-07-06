use javascript::interpreter::{values::Value, Context, Script};
use std::time::Instant;

mod javascript;

fn main() {
    // let code = "class Foo {\n    constructor() {}\n}";
    let code = "var a = 10 + 10";

    let start = Instant::now();
    let mut ctx = Context::new();
    let value = Script::new(code).run_in_context(&mut ctx);

    let duration = start.elapsed();
    println!("Time: {} microsegundos", duration.as_micros());

    println!("{:?}", ctx);
    println!("{:?}", value);
}
