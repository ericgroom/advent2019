use super::operations::Operation;

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub parameters: Vec<Parameter>,
}

#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Value(i32),
    Pointer(usize),
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ParameterMode {
    Value,
    Pointer,
}

impl From<i32> for ParameterMode {
    fn from(code: i32) -> Self {
        match code {
            0 => Self::Pointer,
            1 => Self::Value,
            x => panic!("Unknown parameter mode: {}", x),
        }
    }
}
