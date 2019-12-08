#[derive(PartialEq, Debug)]
pub enum Operation {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl From<i32> for Operation {
    fn from(code: i32) -> Self {
        match code {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            x => panic!("Unknown opcode: {}", x),
        }
    }
}

impl Into<i32> for Operation {
    fn into(self) -> i32 {
        match self {
            Self::Add => 1,
            Self::Multiply => 2,
            Self::Input => 3,
            Self::Output => 4,
            Self::JumpIfTrue => 5,
            Self::JumpIfFalse => 6,
            Self::LessThan => 7,
            Self::Equals => 8,
            Self::Halt => 99,
        }
    }
}

impl Operation {
    pub fn parameter_count(&self) -> i32 {
        match *self {
            Self::Add => 3,
            Self::Multiply => 3,
            Self::Input => 1,
            Self::Output => 1,
            Self::JumpIfTrue => 2,
            Self::JumpIfFalse => 2,
            Self::LessThan => 3,
            Self::Equals => 3,
            Self::Halt => 0,
        }
    }
}
