# interpreter-lang
- there's several paths to build a interpreter or to interprete your program
- steps to build an interpreter from raw text are : 
    - `scanner/lexer/lexical analysis step` gonna turns the chars on the text-file and chunk them together into words (in programming they called `tokens`), tokens can be a char "(", or word "foo", or literals like nums or strings, some chars get ignored like comments and whitespaces, leaving meaningful tokens, example: "var average = (min + max) / 2;" turns into -> "var" "average" "=" "(" "min" "max" ")" "/" "2" ";"
    - `parsing` will take a statment or expression (bunch of tokens) and compose them out of smaller parts, it tuns it to a tree structure (a tree like) knows as `prase tree/ abstract syntax tree (AST) or just trees`.(info is not complete)
    - `static analysis` step will give meaningful purpose to each token "this token referes to what ?", this step will do the `beding` 
- those steps are `the front-end`
- the pipeline of the front-end is tied to the langauge syntax, `the back-end` concerned with final architecture of the running program
- there's a middle step between the to ends, `intermidiate represntation (IR)` it will make it easy to target more platforms with less effort.(search : “control flow graph”, “static single-assignment”, “continuation-passing style”, and “three-address code”.)(didn't quite understand this step)
- there's also `optimization` step, can be at `compile-time` or at `runtime`
- `code Generation (code gen)` step converte the source code into a machine code(and then `the backend`? so what this step does exactly), we need to decide, do we need to target each cpu architucture(difficult) or make like imagenary cpu/machine `virtual machine` program that emulates the cpu that translate the `bytecode`(??) to machine code, the second one is heavy program(emulation) but you'll get simplicity and portability
# Resuources
[crafting-interpreter book](https://craftinginterpreters.com) -> page 16(single pass compilers)
