pub enum Token {
    Operator(Operator),
    Scalar(i32),
}

pub enum Operator {
    Division, Multiplication, Addition, Subtraction
}