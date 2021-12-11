
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
    fn complete_score(&self) -> Option<usize> {
        Some(match self {
            Token::RoundOpen => 1,
            Token::SquareOpen => 2,
            Token::CurlyOpen => 3,
            Token::PointyOpen => 4,
            _ => return None,
        })
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

enum ErrorComplete {
    Error(usize),
    Complete(usize),
}

fn evaluate_line(s: &str) -> SimpleResult<ErrorComplete> {
    let mut stack = Vec::new();
    //let mut error = 0;
    for c in s.chars() {
        let token = Token::from_char(&c)?;
        if token.open() {
            stack.push(token);
        } else {
            match stack.pop() {
                Some(open) if !open.closing(&token) => {
                    return Ok(ErrorComplete::Error(token.error_score()));
                },
                None => {
                    return Ok(ErrorComplete::Error(token.error_score()));
                },
                _ => {},
            }
        }
    }
    let complete = stack.into_iter().rev().fold(0, |complete, c| {
        match c.complete_score() {
            Some(score) => complete * 5 + score,
            None => complete,
        }
    });
    Ok(ErrorComplete::Complete(complete))
}

fn main() -> SimpleResult<()> {
    //let input = TEST_INPUT;
    let input = include_str!("input.txt");
    let (error, complete) = input.split('\n')
        .try_fold((0, Vec::new()), |(error_score, complete_score), line| -> SimpleResult<(usize, Vec<usize>)> {
        match evaluate_line(line)? {
            ErrorComplete::Error(err) => {
                Ok((error_score + err, complete_score))
            },
            ErrorComplete::Complete(comp) => {
                let mut complete_score = complete_score;
                complete_score.push(comp);
                Ok((error_score, complete_score))
            },
        }
    })?;
    let mut complete: Vec<usize> = complete.into_iter().filter(|x| *x != 0).collect();
    complete.sort();
    println!("{}", error);
    println!("{}", complete[complete.len()/2]);

    Ok(())
}
