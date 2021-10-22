use std::io::Error;

use crate::core::{template::engine::functions::{Join, Lower, OneParamFunction, TwoParamFunction, Upper, UpperFirst}, utils::errors::invalid_input_error};

#[derive(Debug)]
pub struct FunctionCall {
    pub function: String,
    pub args: Vec<String>,
}

impl FunctionCall {
    pub fn call(&self) -> Result<String, Error> {
        Ok(match self.function.to_lowercase().as_str() {
            "upper" => {
                Upper::validate_args(&self.args)?;
                Upper::call(&self.args)
            }
            "lower" => {
                Lower::validate_args(&self.args)?;
                Lower::call(&self.args)
            },
            "upper_first" => {
                UpperFirst::validate_args(&self.args)?;
                UpperFirst::call(&self.args)
            },
            "join" => {
                Join::validate_args(&self.args)?;
                Join::call(&self.args)
            }
            _ => return Err(invalid_input_error("Wrong engine function.")),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn function_call_test1() {
        let fc = FunctionCall {
            function: "upper".to_string(),
            args: vec!["Templo".to_string()],
        };
        assert_eq!(fc.call().unwrap(), "TEMPLO".to_string());
    }

    #[test]
    fn function_call_test2() {
        let fc = FunctionCall {
            function: "join".to_string(),
            args: vec!["Templo Moon".to_string(), " ".to_string()],
        };
        assert_eq!(fc.call().unwrap(), "TemploMoon".to_string());
    }
    
    #[test]
    fn function_call_test3() {
        let fc = FunctionCall {
            function: "lower".to_string(),
            args: vec![],
        };
        assert!(fc.call().is_err());
    }
    
    #[test]
    fn function_call_test4() {
        let fc = FunctionCall {
            function: "lower".to_string(),
            args: vec!["Templo Sun".to_string()],
        };
        assert_eq!(fc.call().unwrap(), "templo sun".to_string());
    }

    #[test]
    fn function_call_test5() {
        let lower = FunctionCall {
            function: "lower".to_string(),
            args: vec!["Templo Sun".to_string()],
        };

        let join = FunctionCall {
            function: "join".to_string(),
            args: vec![lower.call().unwrap(), " ".to_string()],
        };

        let upper_first = FunctionCall {
            function: "upper_first".to_string(),
            args: vec![join.call().unwrap()],
        };

        let final_result = upper_first.call().unwrap();

        assert_eq!(final_result, "Templosun".to_string());
    }
}
