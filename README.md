# Youth Programming Language.


or yuth, for short. is dynamic/esoteric language inspired by totalitarianism gov.
- no falsey statement ( use double truthy as falsey statement instead) eg. false = yesyes.
- no `if` statement ( inspired by branchless programming).
- use `.yuth` as file extension.
- no `promise` return.

```
syntax design.

function = rule
true = yes
false = yesyes
print = yell

-------------

eg.

rule hello_people!(){
  var mini_heart = 💕;
  yell "hello, people!, { $mini_heart } ";
}

rule add!(a, b){
  return a + b;
}

rule run_function_inside_function!(){
  do hello_people!();
}

rule get_human_brain_cell!(){
  return 86000;
}

```

## Quick start

```
# run interpreter (REPL)
cargo run  

# run file
cargo run -- <filename.yuth>

# UNUSED!
# to generate abstract syntax tree (AST)
# use this pattern
# cargo run --bin <filename> -- <destination>
cargo run --bin GenerateAst -- src/yuth
```



rules

```

// initial rules.

expression → literal
           | unary
           | binary
           | grouping ;

literal    → NUMBER | STRING | "true" | "false" | "nil" ;
grouping   → "(" expression ")" ;
unary      → ( "-" | "!" ) expression ;
binary     → expression operator expression ;
operator   → "==" | "!=" | "<" | "<=" | ">" | ">="
           | "+"  | "-"  | "*" | "/" ;


---------------------------------

program     → declaration* EOF ;

declaration → var_declaration
            | statement ;

statement → expression_statement
          | for_statement
          | if_statement
          | print_statement
          | while_statement
          | block ;

for_statement   → "for" "(" ( var_declaration | expression_statement | ";" )
                      expression? ";"
                      expression? ")" statement ;


if_statement    → "if" "(" expression ")" statement ( "else" statement )? ;

expression      → assignment ;
print_statement → "print" expression ";" ;
block           → "{" declaration* "}" ;

assignment → identifier "=" assignment
           | logic_or ;
logic_or   → logic_and ( "or" logic_and )* ;
logic_and  → equality ( "and" equality )* ;

equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → addition ( ( ">" | ">=" | "<" | "<=" ) addition )* ;
addition       → multiplication ( ( "-" | "+" ) multiplication )* ;
multiplication → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary
                | primary ;
primary        → "true" | "false" | "nil"
                | NUMBER | STRING
                | "(" expression ")"
                | IDENTIFIER ;


```

# Thanks
it is inspired by Lox language from this [book](http://craftinginterpreters.com/) by Bob Nystrom ( highly recommend to check this out, it's comprehensive/enjoying to read).