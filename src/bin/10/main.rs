
use std::fmt::{
    self,
    Display,
};
use simple_error::{
    //SimpleError,
    SimpleResult,
    simple_error,
};

const TEST_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

#[derive(Debug, PartialEq, Copy, Clone)]
enum Token {
    RoundOpen,   // (
    RoundClose,  // )
    CurlyOpen,   // {
    CurlyClose,  // }
    SquareOpen,  // [
    SquareClose, // ]
    PointyOpen,  // <
    PointyClose, // >
}

impl Token {
    //fn from_char(c: &char) -> Result<Token, SimpleError> {
    fn from_char(c: &char) -> SimpleResult<Token> {
        Ok(match c {
            '(' => Token::RoundOpen,
            ')' => Token::RoundClose,
            '[' => Token::SquareOpen,
            ']' => Token::SquareClose,
            '{' => Token::CurlyOpen,
            '}' => Token::CurlyClose,
            '<' => Token::PointyOpen,
            '>' => Token::PointyClose,
            e => return Err(simple_error!("{:?} is not a valid token!", e)),
        })
    }
    fn open(&self) -> bool {
        match self {
            Token::CurlyOpen |
            Token::RoundOpen |
            Token::SquareOpen |
            Token::PointyOpen => true,
            _ => false,
        }
    }
    fn closing(&self, other: &Token) -> bool {
        match (self, other) {
            (Token::RoundOpen, Token::RoundClose) |
            (Token::SquareOpen, Token::SquareClose) |
            (Token::CurlyOpen, Token::CurlyClose) |
            (Token::PointyOpen, Token::PointyClose) => true,
            _ => false,
        }
    }
    fn error_score(&self) -> usize {
        match *self {
            Token::RoundClose => 3,
            Token::SquareClose => 57,
            Token::CurlyClose => 1197,
            Token::PointyClose => 25137,
            _ => 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(match self {
            Token::RoundOpen => "(",
            Token::RoundClose => ")",
            Token::SquareOpen => "[",
            Token::SquareClose => "]",
            Token::CurlyOpen => "{",
            Token::CurlyClose => "}",
            Token::PointyOpen => "<",
            Token::PointyClose => ">",
        })
    }
}

fn evaluate_line(s: &str) -> SimpleResult<usize> {
    let mut stack = Vec::new();
    for c in s.chars() {
        let token = Token::from_char(&c)?;
        if token.open() {
            stack.push(token);
        } else {
            match stack.pop() {
                Some(open) if !open.closing(&token) => return Ok(token.error_score()),
                None => return Ok(token.error_score()),
                Some(_) => {},
            }
        }
    }
    Ok(0)
}

fn main() -> SimpleResult<()> {
    //let input = TEST_INPUT;
    let input = include_str!("input.txt");
    let score = input.split('\n').try_fold(0, |error_score, line| -> SimpleResult<usize> {
        Ok(error_score + evaluate_line(line)?)
    })?;
    println!("{:?}", score);

    Ok(())
}
