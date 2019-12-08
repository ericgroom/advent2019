use intcode_computer::instruction::Instruction;
use intcode_computer::operations::Operation;
use intcode_computer::parameter::Parameter;

fn str_to_operation(instr: &str) -> Operation {
    match instr.to_ascii_uppercase().as_str() {
        "ADD" => Operation::Add,
        "MUL" => Operation::Multiply,
        "READ" => Operation::Input,
        "WRITE" => Operation::Output,
        "JIT" => Operation::JumpIfTrue,
        "JIF" => Operation::JumpIfFalse,
        "LT" => Operation::LessThan,
        "GT" => Operation::Equals,
        "HALT" => Operation::Halt,
        x => panic!("Unknown operation: {}", x),
    }
}

fn parse_parameter(parameter: &str) -> Parameter {
    if parameter.to_ascii_lowercase().starts_with('i') {
        let int: i32 = parameter[1..].parse().unwrap();
        Parameter::Value(int)
    } else {
        let addr: i32 = parameter.parse().unwrap();
        Parameter::Pointer(addr as usize)
    }
}

pub fn assemble(code: &str) -> Vec<i32> {
    let instructions = code.split('\n');
    let mut result = Vec::new();
    for instruction in instructions {
        let tokens: Vec<_> = instruction.split_ascii_whitespace().collect();
        if tokens.is_empty() {
            continue;
        }
        let operation = str_to_operation(tokens[0]);
        assert_eq!(tokens[1..].len(), operation.parameter_count() as usize);
        let parameters: Vec<Parameter> = tokens[1..]
            .into_iter()
            .map(|param| parse_parameter(param))
            .collect();
        let instruction = Instruction {
            operation: operation,
            parameters: parameters,
        };
        result.append(&mut instruction.into());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic_program() {
        let program = "ADD 1 2 3";
        assert_eq!(assemble(program), vec![1, 1, 2, 3]);
    }

    #[test]
    fn test_multiline_program() {
        let program = "ADD 1 2 3\nMUL 2 3 4\nJIT 2 1";
        assert_eq!(assemble(program), vec![1, 1, 2, 3, 2, 2, 3, 4, 5, 2, 1]);
    }

    #[test]
    fn test_immediate_values() {
        let program = "ADD 0 i2 i5";
        assert_eq!(assemble(program), vec![11001, 0, 2, 5]);
    }
}
