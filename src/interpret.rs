use crate::types::*;

macro_rules! do_op {
    ($code:expr, $op:tt) => {{
		// pop two last variables in the stack
        if let Some(a) = $code.stack.pop() {
            if let Some(b) = $code.stack.pop() {
				// push the result of the operation to stack
                $code.stack.push(Variable {
                    variable: None,
                    value: (b.value $op a.value),
                });
                None
            } else { Some(ProgramError::StackUnderflow) }
        } else { Some(ProgramError::StackUnderflow) }
    }
}}

pub fn interpret(bytecodes: Vec<ByteCode>) -> Result<Variable> {
    let mut code = Program {
        bytecodes,
        stack: Vec::new(),
    };

    // iterate and match the bytecode vector
    for op in code.bytecodes {
        if let Some(err) = match op {
            ByteCode::LOAD_VAL(i) => {
                code.stack.push(Variable {
                    variable: None,
                    value: i,
                });
                None
            }
            ByteCode::WRITE_VAR(c) => {
                let loaded_value = code.stack.pop();
                if let Some(v) = loaded_value {
                    code.stack.push(Variable {
                        variable: Some(c),
                        value: v.value,
                    })
                }
                None
            }
            ByteCode::READ_VAR(c) => {
                let read_value = code.stack.iter().find(|&&x| x.variable == Some(c));
                if let Some(v) = read_value {
                    let var = v.clone();
                    code.stack.push(Variable {
                        variable: var.variable,
                        value: var.value,
                    })
                }
                None
            }
            ByteCode::MULTIPLY => do_op!(code, *),
            ByteCode::ADD => do_op!(code, +),
            ByteCode::RETURN_VALUE => break,
        } {
            return Err(err);
        }
    }

    if let Some(v) = code.stack.pop() {
        Ok(v)
    } else {
        Err(ProgramError::StackUnderflow)
    }
}
