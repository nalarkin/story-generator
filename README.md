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

Concepts used to develop this project.

* BNF grammar notation
  * used to create the grammar 
* Graph traversal and coloring
  * used to test validity of grammar rules before generation starts.
    * Detects if there is at least 1 valid path from each option that is reachable from the starting nonterminal
    * TODO: Detect that each nonterminal is reachable (test by doing single traversal from the starting nonterminal, and traversing over to see which nodes were visited)

FAQ

* Aren't these grammar rules more like trees than graphs?
  * **<u>Trees are graphs which have the minimum number of edges connecting all nodes. Thus, for n nodes, there will always be n-1 edges</u>**. These grammars can contain cycles and still be valid, thus, they are more similar to graphs than trees.
*  



to develop more complex grammar, you might find the following links helpful

* Grammar rules source: https://grammar.reverso.net/
* Grammar conjugation source: https://conjugator.reverso.net/conjugation-english-verb-work.html
