//! Bitmap path encoding for Latin grammatical categories.
//!
//! Each word form is encoded as a single `i32` with the following layout:
//!
//! ```text
//! ┌─────────────┬───────┬──────────────┬───────────────┐
//! │  bits 11-10 │ bit 9 │  bits 8-6    │  bits 5-0     │
//! │  POS marker │ extra │  shared      │  core fields  │
//! └─────────────┴───────┴──────────────┴───────────────┘
//! ```
//!
//! ## Verb (POS = 00)
//! ```text
//! ┌──────┬──────┬───────┬──────────┬───────────┬──────┬──────────┐
//! │11 10 │  9   │   8   │   7 6    │   5 4 3   │  2   │  1 0     │
//! │ 00   │ inf  │ voice │  mood    │  tense    │ num  │ person   │
//! └──────┴──────┴───────┴──────────┴───────────┴──────┴──────────┘
//! ```
//!
//! ## Noun (POS = 01)
//! ```text
//! ┌──────┬───────┬──────────────┬───────────────┐
//! │11 10 │ 9-4   │      3       │    2 0        │
//! │ 01   │(unused)│  number     │    case       │
//! └──────┴───────┴──────────────┴───────────────┘
//! ```
//!
//! ## Adjective (POS = 10)
//! ```text
//! ┌──────┬─────────┬──────┬──────┬───────────┐
//! │11 10 │  9-5    │ 4 3  │  3   │   2 0     │
//! │ 10   │ (unused)│gender│ num  │   case    │
//! └──────┴─────────┴──────┴──────┴───────────┘
//! ```
//!
//! ## Preposition (POS = 11)
//! ```text
//! ┌──────┬─────────────────────────┐
//! │11 10 │         9-0             │
//! │ 11   │       (unused)          │
//! └──────┴─────────────────────────┘
//! ```

use crate::grammar::latin::noun::Case;
use crate::grammar::latin::{
    Number,
    noun::Gender,
    verb::{Mood, Person, Tense, Voice},
};

// Part of Speech markers in bits 10-11
const POS_VERB: i32 = 0;
const POS_NOUN: i32 = 1;
const POS_ADJ: i32 = 2;
const POS_PREP: i32 = 3;

const POS_SHIFT: u32 = 10;

// ── Verb encoding ──

/// Encode a verb form into its bitmap path.
///
/// Fields are packed right-to-left:
/// - bits 0-1 (2 bits): person (First=0, Second=1, Third=2)
/// - bit 2: number (Singular=0, Plural=1)
/// - bits 3-5 (3 bits): tense (Present=0 .. FuturePerfect=5)
/// - bits 6-7 (2 bits): mood (Indicative=0, Subjunctive=1, Imperative=2)
/// - bit 8: voice (Active=0, Passive=1)
/// - bit 9: infinitive flag (false=0, true=1)
/// - bits 10-11: POS marker `00` for verb
pub fn encode_verb(
    person: Person,
    number: Number,
    tense: Tense,
    mood: Mood,
    voice: Voice,
    infinitive: bool,
) -> i32 {
    let p = match person {
        Person::First => 0,
        Person::Second => 1,
        Person::Third => 2,
    };
    let n = match number {
        Number::Singular => 0,
        Number::Plural => 1,
    };
    let t = match tense {
        Tense::Present => 0,
        Tense::Imperfect => 1,
        Tense::Future => 2,
        Tense::Perfect => 3,
        Tense::Pluperfect => 4,
        Tense::FuturePerfect => 5,
    };
    let m = match mood {
        Mood::Indicative => 0,
        Mood::Subjunctive => 1,
        Mood::Imperative => 2,
    };
    let v = match voice {
        Voice::Active => 0,
        Voice::Passive => 1,
    };
    let inf = if infinitive { 1 } else { 0 };
    p | (n << 2) | (t << 3) | (m << 6) | (v << 8) | (inf << 9) | (POS_VERB << POS_SHIFT)
}

pub fn decode_verb(path: i32) -> (Person, Number, Tense, Mood, Voice, bool) {
    let p = match path & 0b11 {
        0 => Person::First,
        1 => Person::Second,
        _ => Person::Third,
    };
    let n = match (path >> 2) & 0b1 {
        0 => Number::Singular,
        _ => Number::Plural,
    };
    let t = match (path >> 3) & 0b111 {
        0 => Tense::Present,
        1 => Tense::Imperfect,
        2 => Tense::Future,
        3 => Tense::Perfect,
        4 => Tense::Pluperfect,
        _ => Tense::FuturePerfect,
    };
    let m = match (path >> 6) & 0b11 {
        0 => Mood::Indicative,
        1 => Mood::Subjunctive,
        _ => Mood::Imperative,
    };
    let v = match (path >> 8) & 0b1 {
        0 => Voice::Active,
        _ => Voice::Passive,
    };
    let inf = (path >> 9) & 0b1 != 0;
    (p, n, t, m, v, inf)
}

// ── Noun encoding ──

/// Encode a noun form into its bitmap path.
///
/// Fields are packed right-to-left:
/// - bits 0-2 (3 bits): case (Nominative=0 .. Vocative=5)
/// - bit 3: number (Singular=0, Plural=1)
/// - bits 4-9: unused
/// - bits 10-11: POS marker `01` for noun
pub fn encode_noun(case: Case, number: Number) -> i32 {
    let c = case_to_u32(case);
    let n = match number {
        Number::Singular => 0,
        Number::Plural => 1,
    };
    c as i32 | (n << 3) | (POS_NOUN << POS_SHIFT)
}

pub fn decode_noun(path: i32) -> (Case, Number) {
    let c = u32_to_case((path & 0b111) as u32);
    let n = match (path >> 3) & 0b1 {
        0 => Number::Singular,
        _ => Number::Plural,
    };
    (c, n)
}

// ── Adjective encoding ──

/// Encode an adjective form into its bitmap path.
///
/// Fields are packed right-to-left:
/// - bits 0-2 (3 bits): case (Nominative=0 .. Vocative=5)
/// - bit 3: number (Singular=0, Plural=1)
/// - bits 4-5 (2 bits): gender (Feminine=0, Masculine=1, Neuter=2)
/// - bits 6-9: unused
/// - bits 10-11: POS marker `10` for adjective
pub fn encode_adjective(gender: Gender, case: Case, number: Number) -> i32 {
    let c = case_to_u32(case);
    let n = match number {
        Number::Singular => 0,
        Number::Plural => 1,
    };
    let g = match gender {
        Gender::Feminine => 0,
        Gender::Masculine => 1,
        Gender::Neuter => 2,
    };
    c as i32 | (n << 3) | (g << 4) | (POS_ADJ << POS_SHIFT)
}

pub fn decode_adjective(path: i32) -> (Gender, Case, Number) {
    let c = u32_to_case((path & 0b111) as u32);
    let n = match (path >> 3) & 0b1 {
        0 => Number::Singular,
        _ => Number::Plural,
    };
    let g = match (path >> 4) & 0b11 {
        0 => Gender::Feminine,
        1 => Gender::Masculine,
        _ => Gender::Neuter,
    };
    (g, c, n)
}

// ── Preposition encoding ──

/// Encode a preposition into its bitmap path.
///
/// Prepositions carry no inflected fields — only the POS marker is set:
/// - bits 0-9: unused
/// - bits 10-11: POS marker `11` for preposition
pub fn encode_preposition() -> i32 {
    POS_PREP << POS_SHIFT
}

fn case_to_u32(c: Case) -> u32 {
    match c {
        Case::Nominative => 0,
        Case::Genitive => 1,
        Case::Dative => 2,
        Case::Accusative => 3,
        Case::Ablative => 4,
        Case::Vocative => 5,
    }
}

fn u32_to_case(v: u32) -> Case {
    match v {
        0 => Case::Nominative,
        1 => Case::Genitive,
        2 => Case::Dative,
        3 => Case::Accusative,
        4 => Case::Ablative,
        _ => Case::Vocative,
    }
}
