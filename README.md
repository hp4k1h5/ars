# A.rs

A language library

> Nel mezzo del cammin di nostra vita \
> mi ritrovai per una selva oscura \
> ché la diritta via era smarrita.
- Dante Alighieri

## Statement of Purpose

Initially, this is an experiment in Rust and Latin.

🦀 I'm not a native rustacean, so advice is welcome on rust implementation details and architecture, and large-scale rewrites may be considered, especially in the early stages of this project.

The initial emphasis is on efficient and abstract language comprehension and production.

### Computational Linguistics

I will be compiling [notes and references](./ref/) as I research topics relevant to the tasks at hand. To wit, this library will have facilities for basic inflection, declension, and conjugation and an ability to manipulate phrases, e.g. from present tense to perfect tense, or from active to passive voice, or singular to plural. The emphasis in early iteration will be on speech production, as opposed to parsing or understanding.

## Lexicon

Eventually, there will be a database of lexical items for other languages, but there will probably be several experimental trials over the type of database (vector vs graph) and vendor. Some considerations include search retrieval of terms across languages and semantic proximity of terms across any language.

## API

An experimental api is served from a minimal instance in the cloud. There is a separate terraform repository for deployment.

Try visiting [http://api.ars.wiki/latin/verbs/8ac13640-5e53-428f-a4e6-6cb18a64a85b/conjugate](http://api.ars.wiki/latin/verbs/8ac13640-5e53-428f-a4e6-6cb18a64a85b/conjugate) or [http://api.ars.wiki/latin/query/ambulo](http://api.ars.wiki/latin/query/ambulo).

### API Docs

Available from [https://hp4k1h5.github.io/ars/](https://hp4k1h5.github.io/ars/)

## Ars server

### environment variables
The following environment variables should be set to interact with the
database. A `.env.{ARS_ENV}` file may be used.

```bash
ARS_ENV=dev
DATABASE_URL='postgres://user:pw@host:port/ars'
```

#### local development

```bash
cargo run --bin server
```

#### docker

```bash
docker run \
  -p 7357:7357 \
  -p 5432:5432 \
  --env-file .env.dev \
  ars:latest
```


## AI policy

I'm working with a variety of models and tools as I learn more about rust's ecosystem. I don't include an AGENTS.md here because I'm not convinced this is the best interface for generic AI instruction, or whether sharing mine is worthwhile. Most of the base models and interfaces in /grammar were written without AI, but I've refactored a few times to accommodate new ideas. AI has been helpful in implementing the diesel + axum API mostly as a macro on top of existing models with minimal changes.
