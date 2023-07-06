#[derive(Clone, Debug)]
pub enum Value {
    Number(f32),
    String(String),
    Undefined,
}
