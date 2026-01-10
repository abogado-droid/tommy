pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug, Clone)]
enum Token {
    TableDec,    // table declaration: [table]
    Int(String), // integer: 123
    // Flo(String), // float: 12.3
    Str(String), // string: "hello"
    EqS,         // equals: =
    BoolF,       // bool: false
    BoolT,       // bool: true
    Color,       //
}

struct Tokenizer {
    tokens: Vec<Token>,
}

impl Tokenizer {
    fn new() -> Self {
        Self { tokens: Vec::new() }
    }
}

pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    // Datetime(Datetime),
    // Array(Array),
    // Table(Table),
}

struct Parse {
    tables: Vec<Value>,
    output: Vec<TomlTable>,
}
impl Parse {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
