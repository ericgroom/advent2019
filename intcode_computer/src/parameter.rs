#[derive(Clone, Copy, Debug)]
pub enum Parameter {
    Value(i32),
    Pointer(usize),
}

impl Into<ParameterMode> for Parameter {
    fn into(self) -> ParameterMode {
        match self {
            Self::Value(_) => ParameterMode::Value,
            Self::Pointer(_) => ParameterMode::Pointer,
        }
    }
}

impl Parameter {
    pub fn raw_value(self) -> i32 {
        match self {
            Self::Value(value) => value,
            Self::Pointer(addr) => addr as i32,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl Into<i32> for ParameterMode {
    fn into(self) -> i32 {
        match self {
            Self::Pointer => 0,
            Self::Value => 1,
        }
    }
}
