pub struct InputToken {
    name: String,
    value: Option<String>
}

pub struct FunctionToken {
    name: String,
    args: Vec<String>
}

pub struct SymbolToken {
    digit: String
}