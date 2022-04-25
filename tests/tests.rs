use rust_interpreter::interpret::interpret;
use rust_interpreter::types::ByteCode;

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    #[test]
    fn test_f_func() {
        use ByteCode::*;

        let test_f_func_inputs = vec![
            // LOAD_VAL 1
            LOAD_VAL(1),
            // WRITE_VAR 'x'
            WRITE_VAR('x'),
            // LOAD_VAL 2
            LOAD_VAL(2),
            // WRITE_VAR 'y'
            WRITE_VAR('y'),
            // READ_VAR 'x'
            READ_VAR('x'),
            // LOAD_VAL 1
            LOAD_VAL(1),
            // ADD -> 1 + 1 = 2 (new value in stack)
            ADD,
            // READ_VAR 'y'
            READ_VAR('y'),
            // MULTIPLY -> 2 * 2 = 4 (new value in stack)
            MULTIPLY,
            // RETURN_VALUE the result
            RETURN_VALUE,
        ];

        assert_eq!(interpret(test_f_func_inputs).unwrap().value, 4);
    }
}
