# A.rs

A language library

> Nel mezzo del cammin di nostra vita \
> mi ritrovai per una selva oscura \
> ché la diritta via era smarrita.
- Dante Alighieri

## Statement of Purpose

Initially, this is an experiment in Rust and Latin.

🦀 I'm not a native rustacean, so advice is welcome on rust implementation details and architecture, and large-scale rewrites may be considered, especially in the early stages of this project.

An important aspect of this effort will be an emphasis on efficient and abstract language comprehension.

### Computational Linguistics

I don't have a formal background in computational linguistics, but I will be compiling [notes and references](./ref/) as I research topics relevant to the tasks at hand. To wit, this library will have facilities for basic inflection, declension, and conjugation and an ability to manipulate phrases, e.g. from present tense to perfect tense, or from active to passive voice, or singular to plural. The emphasis in early iteration will be on speech action, as opposed to parsing or understanding.

The utility is not immediately apparent. For instance, writing ten lines of rust to inflect a simple sentence may not strike the user as useful, but it is the aim of this project to understand whether manipulating higher order constructs built on these primitives can be a useful paradigm.

## Lexicon

Eventually, there will be a database of lexical items for any language, but there will probably be several experimental trials over the type of database (vector vs graph) and vendor. Some considerations include search retrieval of terms across languages and semantic proximity of terms across any language.

### Usage

```rust

use ars::grammar::latin::noun::*;

let = Noun(Declension.I, "puella", "puellae")
```


### RFC

Proposal
```rust
let sentence = Sentence("")
```
