#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCharacter(char),
    Evaluation,
    Overflow,
    UnbalancedParentheses,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Number(i32),
    Operation(Operation),
    OpenBracket,
    CloseBracket,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    Mul,
    Add,
    Sub,
    Div,
}

struct TokenBuilder {
    current_token: Option<Token>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    Number(i32),
    Operation(Operation),
}

#[derive(Debug)]
struct Evaluator {
    instructions: Vec<Instruction>,
}

pub fn evaluate(input: &str) -> Result<i32, Error> {
    let tokens = tokens(input)?;
    let stack = stack(tokens)?;
    Evaluator::new(stack).evaluate()
}

fn stack(tokens: Vec<Token>) -> Result<Vec<Instruction>, Error> {
    let mut instructions = Vec::new();
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            Token::Number(value) => instructions.push(Instruction::Number(value)),
            Token::OpenBracket => stack.push(Token::OpenBracket),
            Token::CloseBracket => 'close_bracket: while let Some(token) = stack.pop() {
                match token {
                    Token::OpenBracket => break 'close_bracket,
                    Token::Number(value) => instructions.push(Instruction::Number(value)),
                    Token::Operation(value) => instructions.push(Instruction::Operation(value)),
                    _ => unreachable!(),
                }
            },
            Token::Operation(value) => {
                if Some(Token::OpenBracket) == stack.last().cloned() {
                    stack.push(Token::Operation(value));
                    continue;
                }
                'operation: while let Some(token) = stack.pop() {
                    match token {
                        Token::OpenBracket => {
                            stack.push(Token::OpenBracket);
                            break 'operation;
                        }
                        Token::Number(number) => instructions.push(Instruction::Number(number)),
                        Token::Operation(another_value) => {
                            if value.precedence() <= another_value.precedence() {
                                instructions.push(Instruction::Operation(another_value))
                            } else {
                                stack.push(Token::Operation(another_value));
                                break 'operation;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                stack.push(Token::Operation(value));
            }
        }
    }
    while let Some(token) = stack.pop() {
        match token {
            Token::OpenBracket => return Err(Error::UnbalancedParentheses),
            Token::Number(value) => instructions.push(Instruction::Number(value)),
            Token::Operation(value) => instructions.push(Instruction::Operation(value)),
            _ => unreachable!(),
        }
    }
    Ok(instructions)
}

fn tokens(input: &str) -> Result<Vec<Token>, Error> {
    let mut tokens = Vec::new();
    let mut token_builder = TokenBuilder::new();
    for ch in input.chars() {
        if let Some(next_token) = match ch {
            number @ '0'...'9' => token_builder.push_number(number.to_digit(10).unwrap() as i32)?,
            '*' => token_builder.push_operation(Operation::Mul),
            '-' => token_builder.push_operation(Operation::Sub),
            '+' => token_builder.push_operation(Operation::Add),
            '/' => token_builder.push_operation(Operation::Div),
            '(' => token_builder.push_token(Token::OpenBracket),
            ')' => token_builder.push_token(Token::CloseBracket),
            ' ' => None,
            invalid_char @ _ => return Err(Error::InvalidCharacter(invalid_char)),
        } {
            tokens.push(next_token);
        }
    }

    if let Some(last_token) = token_builder.next() {
        tokens.push(last_token);
    }

    Ok(tokens)
}

impl Evaluator {
    fn new(instructions: Vec<Instruction>) -> Self {
        Evaluator { instructions }
    }

    fn evaluate(&mut self) -> Result<i32, Error> {
        let mut stack = Vec::new();
        for instruction in &self.instructions {
            match *instruction {
                Instruction::Number(value) => stack.push(value),
                Instruction::Operation(operation) => {
                    if let (Some(right_operand), Some(left_operand)) = (stack.pop(), stack.pop()) {
                        let operation_result = operation.apply(left_operand, right_operand)?;
                        stack.push(operation_result);
                    } else {
                        return Err(Error::Evaluation);
                    }
                }
            }
        }
        let last_value = stack.pop();
        match (last_value, stack.is_empty()) {
            (Some(result), true) => Ok(result),
            _ => Err(Error::Evaluation),
        }
    }
}

impl Operation {
    fn apply(self, left: i32, right: i32) -> Result<i32, Error> {
        match self {
            Operation::Add => left.checked_add(right).ok_or(Error::Overflow),
            Operation::Mul => left.checked_mul(right).ok_or(Error::Overflow),
            Operation::Sub => left.checked_sub(right).ok_or(Error::Overflow),
            Operation::Div => left.checked_div(right).ok_or(Error::Overflow),
        }
    }

    fn precedence(&self) -> i32 {
        match *self {
            Operation::Add => 0,
            Operation::Mul => 1,
            Operation::Sub => 0,
            Operation::Div => 1,
        }
    }
}

impl TokenBuilder {
    fn new() -> TokenBuilder {
        TokenBuilder {
            current_token: None,
        }
    }

    fn push_number(&mut self, number: i32) -> Result<Option<Token>, Error> {
        assert!(number < 10 && number >= 0);
        if let &mut Some(Token::Number(ref mut value)) = &mut self.current_token {
            *value = value
                .checked_mul(10)
                .and_then(|val| val.checked_add(number))
                .ok_or(Error::Overflow)?;
            return Ok(None);
        }

        Ok(::std::mem::replace(
            &mut self.current_token,
            Some(Token::Number(number)),
        ))
    }

    fn push_operation(&mut self, operation: Operation) -> Option<Token> {
        ::std::mem::replace(&mut self.current_token, Some(Token::Operation(operation)))
    }

    fn push_token(&mut self, token: Token) -> Option<Token> {
        ::std::mem::replace(&mut self.current_token, Some(token))
    }

    fn next(&mut self) -> Option<Token> {
        self.current_token.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correctly_emits_tokens() {
        assert_eq!(
            tokens("213+123/()99").unwrap(),
            vec![
                Token::Number(213),
                Token::Operation(Operation::Add),
                Token::Number(123),
                Token::Operation(Operation::Div),
                Token::OpenBracket,
                Token::CloseBracket,
                Token::Number(99),
            ]
        );
    }

    #[test]
    fn test_stack() {
        assert_eq!(
            stack(tokens("10+10").unwrap()).unwrap(),
            vec![
                Instruction::Number(10),
                Instruction::Number(10),
                Instruction::Operation(Operation::Add),
            ]
        );
        assert_eq!(
            stack(tokens("10+5*2").unwrap()).unwrap(),
            vec![
                Instruction::Number(10),
                Instruction::Number(5),
                Instruction::Number(2),
                Instruction::Operation(Operation::Mul),
                Instruction::Operation(Operation::Add),
            ]
        );
    }

    #[test]
    fn test_evaluation() {
        assert_eq!(evaluate("10"), Ok(10));
        assert_eq!(evaluate("10+10"), Ok(20));
        assert_eq!(evaluate("10*10"), Ok(100));
        assert_eq!(evaluate("100/10"), Ok(10));
        assert_eq!(evaluate("20-10"), Ok(10));
    }

    #[test]
    fn test_precedence() {
        assert_eq!(evaluate("10+10 / 10"), Ok(11));
        assert_eq!(evaluate("10+10 *  3"), Ok(40));
        assert_eq!(evaluate("10-10 / 10"), Ok(9));
        assert_eq!(evaluate("10-10 *  3"), Ok(-20));
    }

    #[test]
    fn test_parenthesis() {
        assert_eq!(evaluate("(10+10 )/ 10"), Ok(2));
        assert_eq!(evaluate("(10+10 )* 10"), Ok(200));
        assert_eq!(evaluate("(10-10 )/ 10"), Ok(0));
        assert_eq!(evaluate("(10-10 )* 10"), Ok(0));
    }
}
