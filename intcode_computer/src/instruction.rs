use super::operations::{OpCode, Operation};
use super::parameter::{Parameter, ParameterMode};
use super::{IntcodeMemoryCellType, IntcodeMemoryType};

#[derive(Debug)]
pub struct Instruction {
    pub operation: Operation,
    pub parameters: Vec<Parameter>,
}

impl Into<IntcodeMemoryType> for Instruction {
    fn into(self) -> IntcodeMemoryType {
        let parameter_modes: Vec<ParameterMode> = self
            .parameters
            .iter()
            .map(|param| Into::<ParameterMode>::into(*param))
            .collect();
        let parameter_values: Vec<_> = self
            .parameters
            .into_iter()
            .map(Parameter::raw_value)
            .collect();
        let opcode: IntcodeMemoryCellType = OpCode {
            operation: self.operation,
            parameter_modes: parameter_modes,
        }
        .into();
        let mut result = Vec::new();
        result.push(opcode);
        result.extend(parameter_values.iter());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_into_intcode() {
        let instruction = Instruction {
            operation: Operation::Add,
            parameters: vec![
                Parameter::Value(3),
                Parameter::Value(2),
                Parameter::Pointer(1),
            ],
        };
        let intcode: IntcodeMemoryType = instruction.into();
        assert_eq!(intcode, vec![1101, 3, 2, 1])
    }
}
