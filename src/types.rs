use std::result;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub enum ByteCode {
    LOAD_VAL(i64),
    WRITE_VAR(char),
    READ_VAR(char),
    ADD,
    MULTIPLY,
    RETURN_VALUE,
}

#[derive(Copy, Clone, Debug)]
pub struct Variable {
    pub variable: Option<char>,
    pub value: i64,
}

#[derive(Clone)]
pub struct Program {
    pub bytecodes: Vec<ByteCode>,
    pub stack: Vec<Variable>,
}

#[derive(Debug)]
pub enum ProgramError {
    StackUnderflow,
}

pub type Result<T> = result::Result<T, ProgramError>;
