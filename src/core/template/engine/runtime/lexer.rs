use regex::Regex;
use super::token::{Token, TokenType};
use super::token::blocks::FunctionCall;

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
            tokens.push(Token::new(buffer.trim().to_string(), i, TokenType::Symbol));
            buffer = "".to_string()
        }

        if alphanum_reg.is_match(&buffer) && ch == '(' {
            tokens.push(Token::new(
                buffer.trim().to_string(),
                i,
                TokenType::Function,
            ));
            buffer = "".to_string()
        }

        if input_reg.is_match(&buffer) && !alphanum_reg.is_match(&ch.to_string()) {
            tokens.push(Token::new(buffer.trim().to_string(), i, TokenType::Input));
            buffer = "".to_string()
        }

        buffer.push_str(&ch.to_string());
    }

    build_tokens_block(tokens);
}

fn build_tokens_block(tokens: Vec<Token>) {
    // FUNCTION_CALL -> FUNCTION + SYMBOL + (INPUT|FUNCTION + SYMBOL...)
    let function_call = vec![
        TokenType::Function,
        TokenType::Symbol,
        TokenType::Input,
        TokenType::Symbol,
    ];

    let is_fn_call = function_call
        .iter()
        .zip(tokens.iter())
        .all(|(tk_type, token)| &token.type_ == tk_type);

    if is_fn_call {
        let mut tokens = tokens;
        let function = tokens[0].value.clone();
    
        tokens.pop();
        let args: Vec<String> = tokens
            .drain(2..)
            .into_iter()
            .filter(|token| token.type_ == TokenType::Input)
            .map(|input_tk| input_tk.value)
            .collect();

        let fn_call = FunctionCall { function, args };

        println!("{:?}", fn_call);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &'static str = "Function1($variable1, $variable2)";
    #[test]
    fn lexer_test1() {
        lex(INPUT.to_string());
    }
}
