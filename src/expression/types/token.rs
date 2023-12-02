use fraction::Fraction;
use std::fmt::Display;

// ------------------------------ TOKEN ------------------------------

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    StartBlock(StartBlock),
    EndBlock(EndBlock),
    UnaryOperator(UnaryOp),
    BinaryOperator(BinaryOp),
    Number(Fraction),
}

#[derive(Debug, PartialEq, Eq)]
pub enum StartBlock {
    Bracket,
    Abs,
}

#[derive(Debug, PartialEq, Eq)]
pub enum EndBlock {
    Bracket,
    Abs,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Mod,
    Div,
}

impl From<StartBlock> for Token {
    fn from(value: StartBlock) -> Self {
        Token::StartBlock(value)
    }
}

impl From<EndBlock> for Token {
    fn from(value: EndBlock) -> Self {
        Token::EndBlock(value)
    }
}

impl From<UnaryOp> for Token {
    fn from(value: UnaryOp) -> Self {
        Token::UnaryOperator(value)
    }
}

impl From<BinaryOp> for Token {
    fn from(value: BinaryOp) -> Self {
        Token::BinaryOperator(value)
    }
}

impl From<Fraction> for Token {
    fn from(value: Fraction) -> Self {
        Token::Number(value)
    }
}

impl Display for StartBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StartBlock::Abs => "|",
            StartBlock::Bracket => "(",
        };
        write!(f, "{str}")
    }
}

impl Display for EndBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            EndBlock::Abs => "|",
            EndBlock::Bracket => ")",
        };
        write!(f, "{str}")
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UnaryOp::Neg => "-",
        };
        write!(f, "{str}")
    }
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Mod => "%",
            BinaryOp::Div => "/",
        };
        write!(f, "{str}")
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Token::StartBlock(str) => str.to_string(),
            Token::EndBlock(str) => str.to_string(),
            Token::UnaryOperator(str) => str.to_string(),
            Token::BinaryOperator(str) => str.to_string(),
            Token::Number(str) => str.to_string(),
        };
        write!(f, "{str}")
    }
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Token {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority().cmp(&other.priority())
    }
}

impl Token {
    pub fn eq_type(&self, other: &Self) -> bool {
        match self {
            Token::StartBlock(_) => matches!(other, Token::StartBlock(_)),
            Token::EndBlock(_) => matches!(other, Token::EndBlock(_)),
            Token::UnaryOperator(_) => matches!(other, Token::UnaryOperator(_)),
            Token::BinaryOperator(_) => matches!(other, Token::BinaryOperator(_)),
            Token::Number(_) => matches!(other, Token::Number(_)),
        }
    }

    pub fn priority(&self) -> usize {
        match self {
            Token::StartBlock(_) => 0,
            Token::EndBlock(_) => 1,
            Token::UnaryOperator(_) => 2,
            Token::BinaryOperator(op) => match op {
                BinaryOp::Mul | BinaryOp::Mod | BinaryOp::Div => 3,
                BinaryOp::Add | BinaryOp::Sub => 4,
            },
            Token::Number(_) => 5,
        }
    }
}

impl StartBlock {
    pub fn corrisp(&self) -> EndBlock {
        match self {
            StartBlock::Bracket => EndBlock::Bracket,
            StartBlock::Abs => EndBlock::Abs,
        }
    }
}

impl EndBlock {
    pub fn corrisp(&self) -> StartBlock {
        match self {
            EndBlock::Bracket => StartBlock::Bracket,
            EndBlock::Abs => StartBlock::Abs,
        }
    }
}
