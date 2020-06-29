struct CallFrame<'a> {
    parent: Option<&'a CallFrame<'a>>,
    bytecode: &'a [u8],
    cursor: usize,
    stack: List,
    output: Value,
}
