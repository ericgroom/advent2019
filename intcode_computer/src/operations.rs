use super::parameter::ParameterMode;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(PartialEq, Eq, Debug)]
pub struct OpCode {
    pub operation: Operation,
    pub parameter_modes: Vec<ParameterMode>,
}

impl From<i32> for OpCode {
    fn from(opcode: i32) -> Self {
        let operation_int = opcode % 100;
        let operation = Operation::from(operation_int);
        let mut parameter_modes = Vec::new();
        let mut parameter_section = opcode / 100;
        for _ in 0..operation.parameter_count() {
            let mode = parameter_section % 10;
            parameter_modes.push(ParameterMode::from(mode));
            parameter_section /= 10;
        }
        OpCode {
            operation: operation,
            parameter_modes: parameter_modes,
        }
    }
}

impl Into<i32> for OpCode {
    fn into(self) -> i32 {
        let operation_part: i32 = self.operation.into();
        let parameter_part = {
            let mut result = 0;
            for parameter_mode in self.parameter_modes.into_iter().rev() {
                result *= 10;
                let parameter_code: i32 = parameter_mode.into();
                result += parameter_code;
            }
            result
        };
        operation_part + (parameter_part * 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_from_int() {
        assert_eq!(
            OpCode::from(1002),
            OpCode {
                operation: Operation::Multiply,
                parameter_modes: vec![
                    ParameterMode::Pointer,
                    ParameterMode::Value,
                    ParameterMode::Pointer
                ]
            }
        );
        assert_eq!(
            OpCode::from(11001),
            OpCode {
                operation: Operation::Add,
                parameter_modes: vec![
                    ParameterMode::Pointer,
                    ParameterMode::Value,
                    ParameterMode::Value
                ]
            }
        );
        assert_eq!(
            OpCode::from(99),
            OpCode {
                operation: Operation::Halt,
                parameter_modes: vec![]
            }
        );
    }

    #[test]
    fn test_opcode_into_int() {
        {
            let code = OpCode {
                operation: Operation::Add,
                parameter_modes: vec![
                    ParameterMode::Pointer,
                    ParameterMode::Pointer,
                    ParameterMode::Pointer,
                ],
            };
            assert_eq!(Into::<i32>::into(code), 1);
        }
        {
            let code = OpCode {
                operation: Operation::Multiply,
                parameter_modes: vec![
                    ParameterMode::Pointer,
                    ParameterMode::Pointer,
                    ParameterMode::Value,
                ],
            };
            assert_eq!(Into::<i32>::into(code), 10002);
        }
        {
            let code = OpCode {
                operation: Operation::JumpIfFalse,
                parameter_modes: vec![ParameterMode::Value, ParameterMode::Pointer],
            };
            assert_eq!(Into::<i32>::into(code), 106);
        }
    }
}
