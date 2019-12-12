use intcode_computer::operations::Operation;
use intcode_computer::IntcodeMemoryCellType;

#[derive(Debug)]
pub enum Token {
    LabelDefinition(String),
    Int(IntcodeMemoryCellType),
    Immediate(IntcodeMemoryCellType),
    LabelReference(String),
    Operation(Operation),
}

fn parse_operation(instr: &str) -> Option<Operation> {
    match instr.to_ascii_uppercase().as_str() {
        "ADD" => Some(Operation::Add),
        "MUL" => Some(Operation::Multiply),
        "READ" => Some(Operation::Input),
        "WRITE" => Some(Operation::Output),
        "JIT" => Some(Operation::JumpIfTrue),
        "JIF" => Some(Operation::JumpIfFalse),
        "LT" => Some(Operation::LessThan),
        "GT" => Some(Operation::Equals),
        "HALT" => Some(Operation::Halt),
        _ => None,
    }
}

fn parse_label_definition(label: &str) -> Option<String> {
    let stripped = label.trim();
    if let Some(label) = parse_label(&stripped[..stripped.len() - 1]) {
        if stripped.ends_with(':') {
            println!("here label: {}, stripped: {}", label, &stripped[..]);
            return Some(label.to_string());
        }
    }
    None
}

fn parse_label(label: &str) -> Option<String> {
    let stripped = label.trim();
    println!("inside label: {}, stripped: {}", label, stripped);

    if !stripped.is_empty()
        && stripped.chars().all(|c| c.is_alphanumeric())
        && !stripped.chars().next().unwrap().is_digit(10)
    {
        return Some(stripped.to_string());
    }
    None
}

fn parse_instruction(line: &str) -> Option<Vec<Token>> {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }
    let operation = parse_operation(tokens[0])?;
    let parameters = tokens[1..]
        .into_iter()
        .filter_map(|param| parse_parameter(param));
    let mut result: Vec<Token> = Vec::new();
    result.push(Token::Operation(operation));
    result.extend(parameters);
    Some(result)
}

fn parse_parameter(parameter: &str) -> Option<Token> {
    if parameter.to_ascii_lowercase().starts_with('^') {
        parameter[1..].parse().ok().map(|i| Token::Immediate(i))
    } else if let Some(label) = parse_label(parameter) {
        Some(Token::LabelReference(label))
    } else {
        parameter.parse().ok().map(|i| Token::Int(i))
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut result = Vec::new();
    for token in source.lines() {
        if token.is_empty() {
            continue;
        }
        if let Some(label) = parse_label_definition(token) {
            result.push(Token::LabelDefinition(label));
            continue;
        }
        if let Some(instruction) = parse_instruction(token) {
            result.extend(instruction);
            continue;
        }
    }
    result
}
