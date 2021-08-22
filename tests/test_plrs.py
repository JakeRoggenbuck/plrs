import plrs


def test_is_char_symbol():
    cases = ["[", "]", "{", "}", "(", ")", ".", ",", ":", ";", "=", "'", '"', "\\"]
    for case in cases:
        assert plrs.is_char_symbol(case)

    noncases = ["m", "!", "A", "a", "1", "4", "+"]
    for noncase in noncases:
        assert not plrs.is_char_symbol(noncase)


def test_is_char_operator():
    cases = ["+", "-", "*", "/", "^", ">", "<"]
    for case in cases:
        assert plrs.is_char_operator(case)

    noncases = [".", "m", "!", "A", "a", "1", "4", "3"]
    for noncase in noncases:
        assert not plrs.is_char_operator(noncase)


def test_is_char_whitespace():
    cases = [" ", "\n", "\t"]
    for case in cases:
        assert plrs.is_char_whitespace(case)

    noncases = [".", "m", "!", "A", "a", "1", "4", "+"]
    for noncase in noncases:
        assert not plrs.is_char_whitespace(noncase)


def test_is_char_numeric():
    cases = ["2", "3", "4", "9"]
    for case in cases:
        assert plrs.is_char_numeric(case)

    noncases = [".", "m", "!", "A"]
    for noncase in noncases:
        assert not plrs.is_char_numeric(noncase)


def test_is_single_quote():
    assert plrs.is_single_quote("'")

    noncases = [".", "m", "!", "A"]
    for noncase in noncases:
        assert not plrs.is_single_quote(noncase)


def test_is_double_quote():
    assert plrs.is_double_quote('"')

    noncases = [".", "m", "!", "A"]
    for noncase in noncases:
        assert not plrs.is_double_quote(noncase)


def test_ends_token():
    cases = [("a", " "), ("a", "\n"), ("+", "a"), ("]", "a")]
    for case in cases:
        assert plrs.ends_token(*case)

    cases = [("a", "a"), ("a", "b"), ("a", "c"), ("a", "j")]
    for case in cases:
        assert not plrs.ends_token(*case)


def test_is_part_numeric():
    cases = ["344", "4535", "3424.3432"]
    for case in cases:
        assert plrs.is_part_numeric(case)

    noncases = ["a", "fsdf", "+++", "!"]
    for noncase in noncases:
        assert not plrs.is_part_numeric(noncase)


class TestLexer:
    cases_parse_string = [
        {
            "in": "int a = 23;",
            "out": [
                plrs.Token("int", plrs.Tokens.TypeName.value),
                plrs.Token("a", plrs.Tokens.Identifier.value),
                plrs.Token("=", plrs.Tokens.Assignment.value),
                plrs.Token("23", plrs.Tokens.NumericLiteral.value),
                plrs.Token(";", plrs.Tokens.Semicolon.value),
                plrs.Token("", plrs.Tokens.EOF.value),
            ],
        },
        {
            "in": "str name = 'jake';",
            "out": [
                plrs.Token("str", plrs.Tokens.TypeName.value),
                plrs.Token("name", plrs.Tokens.Identifier.value),
                plrs.Token("=", plrs.Tokens.Assignment.value),
                plrs.Token("jake", plrs.Tokens.StringLiteral.value),
                plrs.Token(";", plrs.Tokens.Semicolon.value),
                plrs.Token("", plrs.Tokens.EOF.value),
            ],
        },
        {
            "in": '"hey";',
            "out": [
                plrs.Token("hey", plrs.Tokens.StringLiteral.value),
                plrs.Token(";", plrs.Tokens.Semicolon.value),
                plrs.Token("", plrs.Tokens.EOF.value),
            ],
        },
    ]

    cases_dont_parse_string = [
        {
            "in": 'str name = "jake";',
            "out": [
                plrs.Token("str", plrs.Tokens.TypeName.value),
                plrs.Token("name", plrs.Tokens.Identifier.value),
                plrs.Token("=", plrs.Tokens.Assignment.value),
                plrs.Token('"', plrs.Tokens.DoubleQuote.value),
                plrs.Token("jake", plrs.Tokens.Identifier.value),
                plrs.Token('"', plrs.Tokens.DoubleQuote.value),
                plrs.Token(";", plrs.Tokens.Semicolon.value),
                plrs.Token("", plrs.Tokens.EOF.value),
            ],
        }
    ]

    def test_lexer_parse_string(self):
        for case in TestLexer.cases_parse_string:
            lexer = plrs.Lexer(case["in"], plrs.Settings.PARSE_STRING)

            tokens = []
            current_token = plrs.EMPTY_TOKEN

            while current_token.token != plrs.Lexer.EOF:
                current_token = lexer.next()
                tokens.append(current_token)

            for found_token, out_token in zip(tokens, case["out"]):
                assert found_token.part == out_token.part
                assert found_token.token == out_token.token

    def test_lexer_dont_parse_string(self):
        for case in TestLexer.cases_dont_parse_string:
            lexer = plrs.Lexer(case["in"], plrs.Settings.NONE)

            tokens = []
            current_token = plrs.EMPTY_TOKEN

            while current_token.token != plrs.Lexer.EOF:
                current_token = lexer.next()
                tokens.append(current_token)

            for found_token, out_token in zip(tokens, case["out"]):
                assert found_token.part == out_token.part
                assert found_token.token == out_token.token
