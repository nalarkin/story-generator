# Story Generator

A application that generates random sentences based of provided grammar rules. Built in Rust.

## Documentation:

- https://nalarkin.github.io/story-generator/

## Requirements

- To run this program, you may use the built-in rust package manage `cargo` or you can use the precompiled binary (story_gen.exe)

## Quick Start to Use the Generator

1. Clone the git repository or download repository as a zip file.
2. Navigate the same directory as the `README.md` file
3. If using `cargo run`
   1. `cargo run examples/simple.txt 40 5`
4. If using `story_gen.exe`
   1. `./bin/story_gen.exe examples/simple.txt 40 5`

## CLI command is

1. If using cargo
   1. `cargo run <relative/path/to/file.txt> <number of sentences> <number of sentences per paragraph>`
2. If using story_gen.exe
   1. `./story_gen.exe <relative/path/to/file.txt> <number of sentences> <number of sentences per paragraph>`
3. You can use standard file redirection with the output produced from the stories. For example, the command `./bin/story_gen.exe examples/simple.txt 40 5 > output.txt` would store the output in a file named output.txt (or overwrite the file if it already exists).

note: `<sentences per paragraph>` is optional, defaults to 1.

## Rules for Custom Grammar:

1. Grammar rules must be stored in a `.txt` file
2. First non-ignored line in file will be the rule that all sentences are derived from
3. Follows similar rules to BNF notation.
   1. For more info see: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form
4. Optional tokens can be surrounded with parenthesis.
5. Delimit multiple RHS options with `|`.
6. Delimit the components of each option in the RHS with spaces.
   1. For example `<np> = <noun><adj>` would produce unintended results, instead use `<np> = <noun> <adj>`
7. Delimit LHS and RHS with `=`.
8. Each non-terminal must have at least 1 path that leads to a terminal node
   1. For example, the two rules `<sentence> = <noun>` and `<noun> = <sentence>` would not be valid, but the following combination would be valid: `<sentence> = <noun>` and `<noun> = <sentence> | cat` where cat is a terminal.
9. You don't need to include angled brackets for non-terminals, I simply used them for readability.

### Simple Grammar Example

````<sentence> = <np> <vp>
<np> = <pro> | <T> <noun> | <T> <adj> <noun>
<pro> = Mike | Sarah | Will
<T> = the | a
<adj> = happy | angry | sad | tired
<noun> = dog | cat | wolf | bird
<vp> = <verb> <T> <noun> | <verb> <T> <adj> <noun> | <verb> <adj> <pro>
<verb> = hugged | bit | bird```
````

### Advanced Grammar Rules

This program allows optional values to be surrounded with parenthesis. When a rule contains an optional token, the program will calculate all possible paths (combinations) that are possible. For example, `noun = (adj) n` would create the options `noun = n | adj n`

```
NP = (D) (AdjP+) N (PP+) (CP)  // this rule generates the following combinations (or paths that could be taken from NP)
"NP": [
        "N",
        "D N",
        "AdjP+ N",
        "D AdjP+ N",
        "N PP+",
        "D N PP+",
        "AdjP+ N PP+",
        "D AdjP+ N PP+",
        "N CP",
        "D N CP",
        "AdjP+ N CP",
        "D AdjP+ N CP",
        "N PP+ CP",
        "D N PP+ CP",
        "AdjP+ N PP+ CP",
        "D AdjP+ N PP+ CP",
    ]
```

## Major Concepts used to develop this project.

- BNF grammar notation
  - Used to create the grammar parse trees.
- Graph traversal and coloring
  - Used to test validity of grammar rules before generation starts.
  - Detects if there is at least 1 valid path from each option that is reachable from the starting non-terminal.
    - Program prints error message and exits if there are any non-valid rules.
  - Detects which LHS tokens are unreachable from the initial starting LHS.
    - Gives a warning, listing the unreachable LHS tokens.

## FAQ

### Aren't these grammar rules more like trees than graphs?

Trees are graphs which have the minimum number of edges connecting all nodes. Thus, for n nodes, there will always be n-1 edges. These grammars can contain cycles and still be valid, thus, they are more similar to graphs than trees.

### How did you create the documentation?

Rust has built in documentation features that create static HTML code.

The command I used was `cargo doc --no-deps --target-dir ./docs `.

### How did you create the executable file?

Rust has a built in binary compilation feature.

The command I used was `cargo build; cp target/debug/story_gen.exe ./bin/story_gen.exe`

### Where is the executable file located?

`project-root/bin/story_gen.exe`

### Why did you create this project?

There are a few reasons.

* I wanted to learn Rust, this was my first project in the language.
* I wanted to implement BNF grammar that I learned in my Programming Language Concepts class.
* I wanted to implement graph theory that I was learning in the CodePath Advanced Interview Prep Course.
* I wanted to create a mock data generator for my other project which is a New York Times clone located at https://www.nlarkin.us/news

## Other resources

- To develop more complex grammar, you might find the following links helpful
  - https://www.nltk.org/book/ch08.html
  - http://www.lel.ed.ac.uk/grammar/overview.html
  - Grammar rules source: https://grammar.reverso.net/
  - Grammar conjugation source: https://conjugator.reverso.net/conjugation-english-verb-work.html
