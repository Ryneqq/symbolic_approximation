pub struct ValueIterator {
    original: Value,
    left: Value,
    right: Value, //
}

impl Iterator for ValueIterator {
    type Item = Value;

    fn next(&mut self) -> Option<Value> {
        match self.value {
            Value::Fun(fun) =>
        }
    }
}