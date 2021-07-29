use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Tokens {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Dot,
    Comma,
    Assignment,
    Semicolon,
    Colon,
    Tag,
    Reference,
    Question,

    Plus,
    Minus,
    Star,
    Slash,
    Carrot,
    Greater,
    Less,

    Space,
    Tab,
    Newline,

    Comment,
    SingleQuote,
    DoubleQuote,
    Identifier,
    NumericLiteral,
}

#[pyclass]
#[derive(PartialEq, Debug)]
struct Token {
    part: String,
    token: Tokens,
}

#[pyfunction]
fn is_char_symbol(ch: char) -> bool {
    match ch {
        '[' | ']' | '{' | '}' | '(' | ')' | '.' | ',' | ':' | ';' | '=' | '\'' | '\"' => true,
        _ => false,
    }
}

#[pyfunction]
fn is_char_operator(ch: char) -> bool {
    match ch {
        '+' | '-' | '*' | '/' | '^' | '>' | '<' => true,
        _ => false,
    }
}

#[pyfunction]
fn is_char_whitespace(ch: char) -> bool {
    match ch {
        '\t' | ' ' | '\n' => true,
        _ => false,
    }
}

#[pyfunction]
fn is_char_numeric(ch: char) -> bool {
    return ch.is_digit(10);
}

#[pyfunction]
fn is_single_quote(ch: char) -> bool {
    return ch == '\'';
}

#[pyfunction]
fn is_double_quote(ch: char) -> bool {
    return ch == '\"';
}

#[pyfunction]
fn ends_token(cur: char, next: char) -> bool {
    if is_char_whitespace(next) {
        return true;
    }
    if is_char_symbol(cur) {
        return true;
    }
    if is_char_symbol(next) {
        return true;
    }
    if is_char_operator(cur) {
        return true;
    }
    if is_char_operator(next) {
        return true;
    }
    if is_char_whitespace(cur) {
        return false;
    }
    return false;
}

#[pyfunction]
fn is_part_numeric(part: &str) -> bool {
    for c in part.chars() {
        if is_char_numeric(c) {
            return true;
        }
    }
    return false;
}

#[pyfunction]
fn tokenize(part: &str) -> Token {
    let mut token = match part {
        "{" => Tokens::LeftBrace,
        "}" => Tokens::RightBrace,
        "[" => Tokens::LeftBracket,
        "]" => Tokens::RightBracket,
        "(" => Tokens::LeftParen,
        ")" => Tokens::RightParen,
        "." => Tokens::Dot,
        "," => Tokens::Comma,
        "=" => Tokens::Assignment,
        ";" => Tokens::Semicolon,
        ":" => Tokens::Colon,
        "#" => Tokens::Tag,
        "&" => Tokens::Reference,
        "?" => Tokens::Question,

        "+" => Tokens::Plus,
        "-" => Tokens::Minus,
        "*" => Tokens::Star,
        "/" => Tokens::Slash,
        "^" => Tokens::Carrot,
        ">" => Tokens::Greater,
        "<" => Tokens::Less,

        " " => Tokens::Space,
        "\t" => Tokens::Tab,
        "\n" => Tokens::Newline,

        "~" => Tokens::Comment,
        "\'" => Tokens::SingleQuote,
        "\"" => Tokens::DoubleQuote,
        _ => Tokens::Identifier,
    };

    if token == Tokens::Identifier {
        if is_part_numeric(part) {
            token = Tokens::Identifier;
        }
    }

    let part = String::from(part);
    return Token { part, token };
}

#[pyclass]
struct Lexer {}

impl Lexer {}

#[pymodule]
fn plrs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_char_symbol, m)?)?;
    m.add_function(wrap_pyfunction!(is_char_operator, m)?)?;
    m.add_function(wrap_pyfunction!(is_char_whitespace, m)?)?;
    m.add_function(wrap_pyfunction!(is_char_numeric, m)?)?;
    m.add_function(wrap_pyfunction!(is_single_quote, m)?)?;
    m.add_function(wrap_pyfunction!(is_double_quote, m)?)?;
    m.add_function(wrap_pyfunction!(ends_token, m)?)?;
    m.add_function(wrap_pyfunction!(is_part_numeric, m)?)?;

    m.add_function(wrap_pyfunction!(lexer, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_char_symbol_test() {
        for i in ['[', ']', ')', '(', '.', ';'].iter() {
            assert!(is_char_symbol(*i));
        }
        for i in ['a', 'b', '7', '8'].iter() {
            assert!(!is_char_symbol(*i));
        }
    }

    #[test]
    fn is_char_operator_test() {
        for i in ['+', '-', '*', '^'].iter() {
            assert!(is_char_operator(*i));
        }

        for i in ['a', '(', '7', ']'].iter() {
            assert!(!is_char_operator(*i));
        }
    }

    #[test]
    fn is_char_whitespace_test() {
        for i in [' ', '\t', '\n'].iter() {
            assert!(is_char_whitespace(*i));
        }

        for i in ['a', '(', '7', ']'].iter() {
            assert!(!is_char_whitespace(*i));
        }
    }

    #[test]
    fn is_char_numeric_test() {
        for i in ['1', '3', '5', '9'].iter() {
            assert!(is_char_numeric(*i));
        }

        for i in ['a', '(', ']', '+', 'n'].iter() {
            assert!(!is_char_numeric(*i));
        }
    }

    #[test]
    fn is_double_quote_test() {
        assert!(is_double_quote('\'') == false);
        assert!(is_double_quote('\"') == true);
    }

    #[test]
    fn is_single_quote_test() {
        assert!(is_single_quote('\'') == true);
        assert!(is_single_quote('\"') == false);
    }
}
