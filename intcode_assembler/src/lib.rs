use intcode_computer::operations::OpCode;
use intcode_computer::parameter::ParameterMode;
use intcode_computer::{IntcodeMemoryCellType, IntcodeMemoryType};
use std::collections::HashMap;

mod lexer;

use lexer::{tokenize, Token};

enum Temp {
    Resolved(IntcodeMemoryCellType),
    LabelReference(String),
}

pub fn assemble(code: &str) -> IntcodeMemoryType {
    let mut result: Vec<Temp> = Vec::new();
    let mut address_map: HashMap<String, usize> = HashMap::new();
    let tokens = tokenize(code);
    println!("{:?}", tokens);
    let mut tokens_iter = tokens.iter();
    while let Some(token) = tokens_iter.next() {
        match token {
            Token::LabelDefinition(label) => {
                address_map.insert(label.clone(), result.len());
            }
            Token::Operation(operation) => {
                let expected_parameters = operation.parameter_count() as usize;
                let mut params = Vec::new();
                for _ in 0..expected_parameters {
                    params.push(tokens_iter.next().unwrap());
                }
                let parameter_modes: Vec<ParameterMode> = params
                    .iter()
                    .map(|token| match token {
                        Token::Immediate(_) | Token::LabelReference(_) => ParameterMode::Value,
                        Token::Int(_) => ParameterMode::Pointer,
                        _ => panic!("not a parmeter"),
                    })
                    .collect();
                let opcode = OpCode {
                    operation: *operation,
                    parameter_modes: parameter_modes,
                };
                result.push(Temp::Resolved(opcode.into()));
                for token in params {
                    match token {
                        Token::Immediate(i) => result.push(Temp::Resolved(*i)),
                        Token::Int(i) => result.push(Temp::Resolved(*i)),
                        Token::LabelReference(label) => {
                            result.push(Temp::LabelReference(label.clone()))
                        }
                        _ => panic!("unexpected token as parameter"),
                    }
                }
            }
            _ => panic!("unexpected token"),
        }
    }
    println!("{:?}", address_map);
    result
        .iter()
        .map(|token| match token {
            Temp::Resolved(i) => *i,
            Temp::LabelReference(label) => *address_map
                .get(label)
                .expect(&format!("label: '{}' not found", label))
                as IntcodeMemoryCellType,
        })
        .collect()
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
        let program = "ADD 0 ^2 ^5";
        assert_eq!(assemble(program), vec![11001, 0, 2, 5]);
    }

    #[test]
    fn test_includes_blank_lines() {
        let program = "\n\nADD 0 ^2 ^5\nMUL 1 2 3";
        assert_eq!(assemble(program), vec![11001, 0, 2, 5, 2, 1, 2, 3]);
    }

    #[test]
    fn test_label_reference() {
        let program = "main:\nADD 2 1 2\nJIT 1 main";
        assert_eq!(assemble(program), vec![1, 2, 1, 2, 5, 1, 0]);
    }
}
