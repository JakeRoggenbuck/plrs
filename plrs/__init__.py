from .plrs import (
    Token,
    Lexer,
    is_char_symbol,
    is_char_operator,
    is_char_whitespace,
    is_char_numeric,
    is_single_quote,
    is_double_quote,
    ends_token,
    is_part_numeric,
    token_num_to_name,
)
from enum import Enum


class Tokens(Enum):
    EOF = 0

    Function = 1
    Class = 2
    Struct = 3
    TypeName = 4
    Operator = 5

    LeftBrace = 6
    RightBrace = 7
    LeftBracket = 8
    RightBracket = 9
    LeftParen = 10
    RightParen = 11

    Dot = 12
    Comma = 13

    Assignment = 14
    Semicolon = 15
    Colon = 16
    Tag = 17
    Reference = 18
    Question = 19
    At = 20
    Percent = 21
    Bang = 22
    BackSlash = 23

    Arrow = 24
    Equal = 25

    Space = 26
    Tab = 27
    Newline = 28

    SingleQuote = 29
    DoubleQuote = 30
    Identifier = 31
    NumericLiteral = 32
    StringLiteral = 33

    LoopExit = 34
    Return = 35

    Empty = 0xF09F


class Settings:
    NONE = 0
    PARSE_STRING = 1
    ALL = PARSE_STRING


EOF_TOKEN = Token("", 0)
EMPTY_TOKEN = Token("", 0xF09F)


__version__ = "0.1.3"
__author__ = "jakeroggenbuck"
__license__ = "MIT"
