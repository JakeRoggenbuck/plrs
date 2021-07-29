from plrs import Lexer, Token

string = "func this(a: int) { return (0); }"

lexer = Lexer(string)


tok = Token("", 0)
while tok.token != Lexer.EOF:
    tok = lexer.next()
    print(tok)
