## Rust challenges

Q1. You are a TA at a university, and you want to evaluate your student's homework without executing their (untrusted) code. You decide to write a small web-service that takes bytecode as input, and interprets the results.
The bytecode language you need to support includes basic arithmetic and variables. The bytecode language is stack, rather than register based.
ByteCode (right) is given for the following pseudo code (left):

```
function f() {
    x = 1               LOAD_VAL 1
                        WRITE_VAR 'x'
    y = 2               LOAD_VAL 2
                        WRITE_VAR 'y'
    return (x + 1) * y  READ_VAR 'x'
                        LOAD_VAL 1
                        ADD
                        READ_VAR 'y'
                        MULTIPLY
                        RETURN_VALUE
}
```

Add a data type `ByteCode` that can represent bytecode like in the example above, along with an interpreter for said bytecode. Make sure your bytecode is flat, i.e. not nested.

Result:

```bash
cargo test
```

Q4. Write a function that given a directory, recursively finds all files with a given file extension in that directory and all sub-directories, and counts the number of lines in the file and prints it to stdout.

Result:

```bash
cargo run
```
