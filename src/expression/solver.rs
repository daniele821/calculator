use crate::{BinaryOp, EndBlock, StartBlock, Token, UnaryOp};

fn parse_tokens(str: &str) -> Vec<Token> {
    let mut stack = Vec::<StartBlock>::new();
    let mut res = Vec::new();

    for c in str.chars() {
        match c {
            '+' => res.push(Token::from(BinaryOp::Add)),
            '-' => match res.last() {
                Some(Token::Number(_)) | Some(Token::EndBlock(_)) => {
                    res.push(Token::from(BinaryOp::Sub))
                }
                _ => res.push(Token::from(UnaryOp::Neg)),
            },
            '*' => res.push(Token::from(BinaryOp::Mul)),
            '/' => res.push(Token::from(BinaryOp::Div)),
            '%' => res.push(Token::from(BinaryOp::Mod)),
            '(' => {
                stack.push(StartBlock::Bracket);
                res.push(Token::from(StartBlock::Bracket));
            }
            ')' => match stack.last() {
                Some(StartBlock::Bracket) => {
                    stack.pop();
                    res.push(Token::from(EndBlock::Bracket));
                }
                _ => panic!("unbalanced blocks!"),
            },
            '|' => match stack.last() {
                Some(StartBlock::Abs) => {
                    stack.pop();
                    res.push(Token::from(EndBlock::Abs));
                }
                _ => {
                    stack.push(StartBlock::Abs);
                    res.push(Token::from(StartBlock::Abs))
                }
            },
            '0'..='9' | '.' => {}
            _ => panic!("invalid token!"),
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::{EndBlock, StartBlock, Token};

    #[test]
    fn test_parsing() {
        let actual_res1 = super::parse_tokens("(||||)()");
        let expected_res1 = vec![
            Token::from(StartBlock::Bracket),
            Token::from(StartBlock::Abs),
            Token::from(EndBlock::Abs),
            Token::from(StartBlock::Abs),
            Token::from(EndBlock::Abs),
            Token::from(EndBlock::Bracket),
            Token::from(StartBlock::Bracket),
            Token::from(EndBlock::Bracket),
        ];
        assert_eq!(expected_res1, actual_res1);
    }
}
