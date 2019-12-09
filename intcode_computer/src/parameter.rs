use super::IntcodeMemoryCellType;

#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Value(IntcodeMemoryCellType),
    Pointer(usize),
    Relative(IntcodeMemoryCellType),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParameterMode {
    Value,
    Pointer,
    Relative,
}

impl Into<ParameterMode> for Parameter {
    fn into(self) -> ParameterMode {
        match self {
            Self::Value(_) => ParameterMode::Value,
            Self::Pointer(_) => ParameterMode::Pointer,
            Self::Relative(_) => ParameterMode::Relative,
        }
    }
}

impl Parameter {
    pub fn raw_value(self) -> IntcodeMemoryCellType {
        match self {
            Self::Value(value) => value,
            Self::Pointer(addr) => addr as IntcodeMemoryCellType,
            Self::Relative(offset) => offset,
        }
    }
}

impl From<IntcodeMemoryCellType> for ParameterMode {
    fn from(code: IntcodeMemoryCellType) -> Self {
        match code {
            0 => Self::Pointer,
            1 => Self::Value,
            2 => Self::Relative,
            x => panic!("Unknown parameter mode: {}", x),
        }
    }
}

impl Into<IntcodeMemoryCellType> for ParameterMode {
    fn into(self) -> IntcodeMemoryCellType {
        match self {
            Self::Pointer => 0,
            Self::Value => 1,
            Self::Relative => 2,
        }
    }
}
