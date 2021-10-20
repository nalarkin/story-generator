# Story Generator

## A CLI built with Rust that will generate a story based of provided grammar rules.

Grammar rules source: https://grammar.reverso.net/
Grammar conjugation source: https://conjugator.reverso.net/conjugation-english-verb-work.html

Simple Grammar
<sentence> => <np> <vp>
<np> => <T> <noun>
<vp> => <np> <verb> <noun>
<T> => "the"
<noun> => "dog" | "cat"
<verb> => "jumped"

