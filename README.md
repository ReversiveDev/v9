# v9
A Javascript Engine Interpreter

## Usage
```rust
use javascript::interpreter::{values::Value, Context, Script};

mod javascript;

fn main() {
    // Write the javascript code
    let code = "10 + 10";

    // Create the context
    let ctx = Context {};

    // Execute the code in the context
    let value = Script::new(code).run_in_context(&ctx);

    // Read the return value
    match value {
        Value::Number(n) => println!("{n}"),
        _ => panic!("Expected a number"),
    }
}
```
