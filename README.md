# Overview on what i'm going to build
- there's several paths to build a interpreter or to interprete your program
- steps to build an interpreter from raw text are : 
    - `scanner/lexer/lexical analysis step` gonna turns the chars on the text-file and chunk them together into words (in programming they called `lexemes`), lexemes can be a char "(", or word "foo", or literals like nums or strings, some chars get ignored like comments and whitespaces, leaving meaningful tokens, example: "var average = (min + max) / 2;" turns into -> "var" "average" "=" "(" "min" "max" ")" "/" "2" ";"
    - `parsing` will take a statment or expression (bunch of tokens) and compose them out of smaller parts, it tuns it to a tree structure (a tree like) knows as `prase tree/ abstract syntax tree (AST) or just trees`.(info is not complete)
    - `static analysis` step will give meaningful purpose to each token "this token referes to what ?", this step will do the `beding` 
- those steps are `the front-end`
- the pipeline of the front-end is tied to the langauge syntax, `the back-end` concerned with final architecture of the running program
- there's a middle step between the to ends, `intermidiate represntation (IR)` it will make it easy to target more platforms with less effort.(search : “control flow graph”, “static single-assignment”, “continuation-passing style”, and “three-address code”.)(didn't quite understand this step)
- there's also `optimization` step, can be at `compile-time` or at `runtime`
- `code Generation (code gen)` step converte the source code into a machine code(and then `the backend`? so what this step does exactly), we need to decide, do we need to target each cpu architucture(difficult) or make like imagenary cpu/machine `virtual machine` program that emulates the cpu that translate the `bytecode`(??) to machine code, the second one is heavy program(emulation) but you'll get simplicity and portability
- types of impl `compilers`:
    - `single-pass compilers` they skip some steps  
    - `walk-tree interpreter` or `interpreter` they just scan and parse source code to AST with a bit of static analysis in order to run the program, they are meant for little languages, but it is slow
    - `transcompilers` is translate your langauge into other langauge that have existing tooling (e.x C lang, or to target the web Javascript) so you don't have to write the backend
    - `just-in-time JIT`
- `compiler` is an implementation technique that involves transliting a lang source code into other low or high level form. the compiler won't execute the source code, the user need to get the resulting output and run it themselves
- `interpreter` is an implementation technique that takes the source code and run it immediately (read one line and execute it immediately)
- most scripting langauges get an interpreter and a compiler at some point

# Implementing the interpreter
## Scanner/Lexing short for Lexical anaylsis
- take the source code chars and turns it into `Lexemes`(words), and then categorize it to represent a meaningful thing to turn into a `token` that used for parsing(next step) 
### Implementation
# Resuources
[crafting-interpreter book](https://craftinginterpreters.com) -> chapter4 (Reserved words and identifier)

# what i learn building this project beside interpreter/compilers concepts
- when to have "&str" and "String" ?
