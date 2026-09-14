use logos::{Lexer, Logos};

#[derive(Default, Debug, thiserror::Error, Clone, PartialEq)]
pub enum Error {
    #[default]
    #[error("invalid token")]
    InvalidToken,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    program: Box<str>,
    flags: Box<[Box<str>]>,
}

#[derive(Debug, Logos, PartialEq, Clone)]
#[logos(skip r"\s+", error = Error)]
pub enum Token {
    #[regex(r"\S.*", (|lex: &mut Lexer<Token>| -> Option<Command> {
        let mut parts = lex.slice().split(' ');

        let program: Box<str> = parts.next()?.into();
        let flags: Box<[Box<str>]> = parts.map(Into::into).collect();

        Some(Command { program, flags })
    }), allow_greedy = true)]
    Command(Command),
}

pub fn lex(source: &str) -> Result<Vec<Token>, Error> {
    Token::lexer(source).collect::<Result<Vec<Token>, Error>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_lex() {
        assert_eq!(
            lex("testcmd --flag1 --flag2"),
            Ok(vec![Token::Command(Command {
                program: "testcmd".into(),
                flags: Box::new(["--flag1".into(), "--flag2".into()])
            })])
        );
    }
}
