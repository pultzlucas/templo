use regex::Regex;

// enum TokenId {
//     Variable(),
//     Function(),
// }
#[derive(Debug)]
struct Token {
    value: String,
    index: usize,
}

impl Token {
    pub fn new(value: String, end_index: usize) -> Self {
        Self {
            index: end_index - value.len(),
            value
        }
    }
}

fn lex(input: String) {
    
    // REGEXES
    let alphanum_reg = Regex::new(r"\w+").unwrap();
    let input_reg = Regex::new(r"\$\w+").unwrap();
    
    let input = format!("{} ", input);
    let chars = input.chars().into_iter().enumerate();
    
    let mut buffer = String::new();
    let mut tokens: Vec<Token> = vec![];
    let symbols = vec!["(", ")", ","];

    for (i, ch) in chars {
        if symbols.iter().any(|symbol| buffer == *symbol) {
            tokens.push(Token::new(buffer.trim().to_string(), i));
            buffer = "".to_string()
        }
        
        if alphanum_reg.is_match(&buffer) && ch == '(' {
            tokens.push(Token::new(buffer.trim().to_string(), i));
            buffer = "".to_string()
        }
        
        if input_reg.is_match(&buffer) && ch == ')' {
            tokens.push(Token::new(buffer.trim().to_string(), i));
            buffer = "".to_string()
        }
        
        buffer.push_str(&ch.to_string());
    }
    println!("{:?}", tokens);
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "Function2(Function1($variable1), $variable2)";
    #[test]
    fn lexer_test1() {
        lex(INPUT.to_string())
    }
}
