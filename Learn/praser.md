# Parsing
- parsing's job is to produce an AST
- the parser may misenderstand the user's code(it can generate valid AST), the parser should track which rule a token belong into in order to generate a "correct" AST
- an incorrect AST is caused by incorrect evaluation of operations
- we solve this problem by defining precedence and assosiativity for the operators, implies making the expression rule more specific (making a subexpression)
- primary expr/rule covers the highest precedence literals and parenthesized expr
- there's multiple implemantation of a parser, we choose `recursive descent` which is basically translate the grammar's rule graph into an actual code, in other word, walk through the grammar starting from high to low precedence rules
