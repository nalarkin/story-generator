# Story Generator

## A application generates a story based of provided grammar rules and is built in Rust.

### Requirements

* Have Rust installed

### How to run example

Navigate the same directory as the `README.md` file and enter the command `cargo run examples/simple.txt 20`

### CLI command is

`cargo run <relative/path/to/file.txt> <number of sentences>`

### Rules for custom grammar:

1. Grammar rules must be stored in a `.txt` file
2. Grammar rules must be in BNF notation. 
   1. For more info see: https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form 
3. First line in file will be the rule that all sentences are derived from
4. You don't need to include angled brackets for non-terminals, I simply did so for clarity. 
5. Each nonterminal must have at least 1 path that leads to a terminal node
   1. For example, the two rules `<sentence> = <noun>` and `<noun> = <sentence>` would not be valid, but the following combination would be valid:  `<sentence> = <noun>`  and `<noun> = <sentence> | cat` where cat is a terminal.

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

to develop more complex grammar, you might find the following links helpful

* Grammar rules source: https://grammar.reverso.net/
* Grammar conjugation source: https://conjugator.reverso.net/conjugation-english-verb-work.html
