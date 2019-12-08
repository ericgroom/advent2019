use intcode_computer::operations::Operation;

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

pub fn assemble(code: &str) -> Vec<i32> {
    let instructions = code.split('\n');
    let mut result = Vec::new();
    for instruction in instructions {
        let tokens: Vec<_> = instruction.split_ascii_whitespace().collect();
        let operation = str_to_operation(tokens[0]);
        assert_eq!(tokens[1..].len(), operation.parameter_count() as usize);
        let mut addresses: Vec<i32> = tokens[1..]
            .into_iter()
            .map(|addr| addr.parse::<i32>().ok().unwrap())
            .collect();
        let opcode: i32 = operation.into();
        result.push(opcode);
        result.append(&mut addresses);
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
}
