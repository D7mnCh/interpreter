# Statements and state
- statements don't evaluate to a value, they need to do something underneath, that thing called `side effect`. It could mean Io, or changing some state in the interpreter
- `expression statment` rule let's you place expression where statment are expected e.g "function_return_i32();"
- `Environment` needed to store/mutate data/state of the program (e.g bindings, re-assignment,...)
- `Lexical scope` or `static scope` is a block of code where you do know what the variable referes to (e.g nested block)
- `dynamic scope` is a block of code where you don't actaully know what the variable refers to until you exec the code(e.g functions body, where the variable knows it's type at runtime. dynamic langauges are known for that) (i don't really know exactely what is the meaning lixcal and dynamic scoping)
