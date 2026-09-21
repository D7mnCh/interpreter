# Overview on what i'm going to build
> [!NOTE]
> the overview will not include rust speicifc info(you'll find it in the source code if so), and also the book is used mainly to learn the concepts(so don't dive into java impl)

- book's challenges section for search and sometime try to impl a new feature
- book's Design notes section give info about other programming languages implementation
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
    - e.g of a lexeme is "(" that turns into a token "LeftParen"
- `lexical grammar` is the rules that determine how on a particular language group chars into tokens, if so the lang is clasiffied as a `regular language`
- identifiers can be user-defined(variables) or reserved keywords

## Representing code
- when turning the source code into tokens, you will transform it into a high-level representation(series of tokens)
- we gotta build a abstract-syntax-tree(AST) that describes any valid syntax pattern for our language
- that AST is more complexe than that scanner's output, that will allow me turn theme into a bunch of meaningfull data(that where chapter names came from, we will represent our code as a data) 
- to write a `parser`, need to turn that high-level representation into more richer and complex one
- A `formal grammar` takes a set of atomic pieces it calls its “alphabet”(complete set of letters). Then it defines a (usually infinite) set of “strings” that are “in” the grammar. Each string is a sequence of “letters”(specific symbol) in the alphabet (umm, that's a the scanner thing, but the scanner only have one string and didn't try to make an infinite set of strings)
- formal grammar job is to specifies which strings are valid and which aren't
- if a string created by set of rules, they called a `derivation`
- rules are callled `productions` cuz they produce strings
- each production/rule has a `head`(name), and a `body` which describe what it generates
- in pure form the body of a rule is a list of symbols that come with two flavors: `terminals`(letter/lexeme/token) and `nonterminals`(another/nested rule/production)
- Author's expression's grammar (or syntax of our langauge) is defnined as : "rule(head) ->symbols(body);". if a symbol is a terminal defined as quoted string(if terminal/token may vary (not exactly the same but it's in the same category like numbers or strings) it represented as fully capitalized word), else (nonterminals) lowercase word
- recurions allowance on nonterminals (it looks like rust's macro's syntax):
    - `+` allow recurion at least once or more
    - `*` allow recurion zero or more
    - `?` allow recurion zero or once
# Resuources
[crafting-interpreter book](https://craftinginterpreters.com) -> chapter-5
