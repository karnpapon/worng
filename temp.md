statementstatementstatement >> Ok(Return(Token { token_type: Return, lexeme: "return", literal: None, line: 3 }, Literal(String("correcttt"))))


statementstatementstatement >> Ok(Return(Token { token_type: Return, lexeme: "return", literal: None, line: 5 }, Literal(String("not this number"))))


statementstatementstatement >> Ok(If(Binary(Var(Token { token_type: Identifier, lexeme: "n", literal: None, line: 2 }, None), Token { token_type: EqualEqual, lexeme: "==", literal: None, line: 2 }, Literal(Number(5.0))), Block([Return(Token { token_type: Return, lexeme: "return", literal: None, line: 3 }, Literal(String("correcttt")))]), Some(Block([Return(Token { token_type: Return, lexeme: "return", literal: None, line: 5 }, Literal(String("not this number")))]))))



statementstatementstatement >> Ok(Func(Token { token_type: Identifier, lexeme: "fibo", literal: None, line: 1 }, [Token { token_type: Identifier, lexeme: "n", literal: None, line: 1 }], Block([If(Binary(Var(Token { token_type: Identifier, lexeme: "n", literal: None, line: 2 }, None), Token { token_type: EqualEqual, lexeme: "==", literal: None, line: 2 }, Literal(Number(5.0))), Block([Return(Token { token_type: Return, lexeme: "return", literal: None, line: 3 }, Literal(String("correcttt")))]), Some(Block([Return(Token { token_type: Return, lexeme: "return", literal: None, line: 5 }, Literal(String("not this number")))])))])))


statementstatementstatement >> Ok(Print(Call(Var(Token { token_type: Identifier, lexeme: "fibo", literal: None, line: 9 }, None), Token { token_type: RightParen, lexeme: ")", literal: None, line: 9 }, [Literal(Number(5.0))])))




statementstatementstatement >> 
Ok(Return(Token { 
    token_type: Return, 
    literal: None, 
    lexeme: "return", 
    line: 3 
    }, 
    Literal(String("correcttt"))
 )
)


statementstatementstatement >> Ok(Return(Token { token_type: Return, literal: None, lexeme: "return", line: 5 }, Literal(String("not this number"))))


statementstatementstatement >> Ok(If(Binary(Var(Token { token_type: Identifier, literal: None, lexeme: "n", line: 2 }, None), Token { token_type: EqualEqual, literal: None, lexeme: "==", line: 2 }, Literal(Number(5.0))), Block([Return(Token { token_type: Return, literal: None, lexeme: "return", line: 3 }, Literal(String("correcttt")))]), Some(Block([Return(Token { token_type: Return, literal: None, lexeme: "return", line: 5 }, Literal(String("not this number")))]))))


statementstatementstatement >> Ok(Func(Token { token_type: Identifier, literal: None, lexeme: "fibo", line: 1 }, [Token { token_type: Identifier, literal: None, lexeme: "n", line: 1 }], Block([If(Binary(Var(Token { token_type: Identifier, literal: None, lexeme: "n", line: 2 }, None), Token { token_type: EqualEqual, literal: None, lexeme: "==", line: 2 }, Literal(Number(5.0))), Block([Return(Token { token_type: Return, literal: None, lexeme: "return", line: 3 }, Literal(String("correcttt")))]), Some(Block([Return(Token { token_type: Return, literal: None, lexeme: "return", line: 5 }, Literal(String("not this number")))])))])))


statementstatementstatement >> Ok(Print(Call(Var(Token { token_type: Identifier, literal: None, lexeme: "fibo", line: 9 }, None), [Literal(Number(5.0))], Token { token_type: RightParen, literal: None, lexeme: ")", line: 9 })))