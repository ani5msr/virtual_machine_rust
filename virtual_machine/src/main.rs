#[derive(Clone)]
enum Operand {
    Value(i32),
    Var(String),
}

#[derive(Clone)]
enum Instruction {
    Push(i32),
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Print,
    Set(String, i32),
    Get(String),
    Input(String),
    If(Vec<Instruction>, Vec<Instruction>),
    Else(Vec<Instruction>),
}