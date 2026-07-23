/// Parse Lewis Elementary Latin Dictionary XML → CSV wordfiles.
///
/// Uses quick-xml Reader (event‑based) to extract fields from <entry>
/// elements, then classifies PoS with simple XML‑field heuristics. Output
/// files match the existing data/latin/ format.
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use quick_xml::{Reader, events::Event};

// ── output rows (match data/latin/*.csv column headers) ──────────────────

#[derive(Clone)]
struct NounRow {
    nominative: String,
    genitive: String,
    gender: String,
    declension: String,
}

#[derive(Clone)]
struct VerbRow {
    present: String,
    infinitive: String,
    perfect: String,
    supine: String,
    conjugation: String,
}

#[derive(Clone)]
struct AdjectiveRow {
    f: String,
    m: String,
    n: String,
    declension: String,
}

#[derive(Clone)]
struct PrepositionRow {
    word: String,
    cases: String,
}

#[derive(Clone)]
enum Row {
    Noun(NounRow),
    Verb(VerbRow),
    Adjective(AdjectiveRow),
    Preposition(PrepositionRow),
    Skip,
}

// ── PoS detection ────────────────────────────────────────────────────────

fn classify(orth_raw: &str, pos: &str, itype: &str, gender: &str, tr: &str) -> Row {
    let orth = clean_orth(orth_raw);
    if orth.is_empty() {
        return Row::Skip;
    }

    let pos_lc = pos.to_lowercase();
    let itype = itype.trim();
    let gen_lc = gender.to_lowercase();

    if pos_lc.contains("praep") || pos_lc.contains("prep") {
        let cases = prep_cases(&pos_lc, tr);
        return Row::Preposition(PrepositionRow { word: orth, cases });
    }
    if pos_lc.contains("adj") {
        if let Some(r) = try_adjective(&orth, itype, &pos_lc) {
            return Row::Adjective(r);
        }
        return Row::Skip;
    }
    if is_verb_itype(itype, &pos_lc) {
        if let Some(r) = try_verb(&orth, itype) {
            return Row::Verb(r);
        }
        return Row::Skip;
    }
    if !gen_lc.is_empty() && !itype.is_empty() {
        if let Some(r) = try_noun(&orth, itype, &gen_lc) {
            return Row::Noun(r);
        }
        return Row::Skip;
    }
    Row::Skip
}

// ── helpers ──────────────────────────────────────────────────────────────

fn clean_orth(s: &str) -> String {
    clean_field_keep_hyphens(s)
}

fn clean_field_keep_hyphens(s: &str) -> String {
    let s = s.trim().replace(['\n', '\r', ','], "");
    let mut out = String::new();
    let mut depth = 0u32;
    for c in s.chars() {
        match (c, depth) {
            ('(' | '（', _) => depth += 1,
            (')' | '）', 1..) => {
                depth -= 1;
                continue;
            }
            (_, 1..) => {}
            (_, 0) => {
                out.push(c);
            }
        }
    }
    let s = out.trim().to_string();
    let s = if let Some(p) = s.find(" or ") {
        s[..p].trim().to_string()
    } else {
        s
    };
    s.split_whitespace().collect::<Vec<_>>().join("")
}

/// Strip parenthetical glosses, newlines, commas, and hyphens from CSV-bound fields.
fn clean_field(s: &str) -> String {
    let s = s.trim().replace(['\n', '\r', ','], "");

    // Strip parenthetical content like "(adsp-, -argō）".
    let mut out = String::new();
    let mut depth = 0u32;
    for c in s.chars() {
        match (c, depth) {
            ('(' | '（', _) => depth += 1,
            (')' | '）', 1..) => {
                depth -= 1;
                continue;
            }
            (_, 1..) => {}
            (_, 0) => {
                out.push(c);
            }
        }
    }
    let s = out.trim().to_string();

    // "amōmum or -on" → "amōmum"
    let s = if let Some(p) = s.find(" or ") {
        s[..p].trim().to_string()
    } else {
        s
    };

    // Collapse whitespace and remove hyphens.
    s.replace('-', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
}

fn prep_cases(pos: &str, tr: &str) -> String {
    let blob = format!("{} {}", pos, tr);
    let mut cs: Vec<&str> = vec![];
    if blob.contains("acc") {
        cs.push("acc.");
    }
    if blob.contains("abl") {
        cs.push("abl.");
    }
    if blob.contains("gen") {
        cs.push("gen.");
    }
    if cs.is_empty() {
        "acc.".into()
    } else {
        cs.join("; ")
    }
}

fn is_verb_itype(itype: &str, pos: &str) -> bool {
    if pos.contains("dep") && !itype.is_empty() {
        return true;
    }
    let i = itype.trim();
    if !i.contains(',') {
        return false;
    }
    // Inf suffix must appear as its own comma‑delimited part, not as a
    // substring of a noun genitive like "trī,".
    (i.contains(", āre")
        || i.contains(",āre")
        || i.contains(", ārī")
        || i.contains(",ārī")
        || i.contains(", ēre")
        || i.contains(",ēre")
        || i.contains(", ere")
        || i.contains(",ere")
        || i.contains(", īre")
        || i.contains(",īre")
        || i.contains(", īrī")
        || i.contains(",īrī")
        || i.contains(", ī")
        || i.contains(",ī"))
        && (i.contains('ī') || i.contains("sī") || i.contains("sum"))
}

fn try_adjective(orth: &str, itype: &str, pos: &str) -> Option<AdjectiveRow> {
    // 3rd‑declension detection:
    //  itype "e" / "is" → 3rd.  orth patterns like "-is", "-ns", "-eps", "-ps",
    //  "-bs", "-rs", "-x" → 3rd.  BUT "-er" is ambiguous — default to I_II
    //  unless itype explicitly signals III.
    let has_3rd_itype = itype.contains("is") || itype.trim().ends_with('e');
    let has_3rd_orth = orth.ends_with("is")
        || orth.ends_with("ns")
        || orth.ends_with("eps")
        || orth.ends_with("ps")
        || orth.ends_with("bs")
        || orth.ends_with("rs")
        || orth.ends_with("x")
        || orth.ends_with("ēs");
    let is_3rd = has_3rd_itype
        || pos.contains("3")
        || (has_3rd_orth && !orth.ends_with("er") && !orth.ends_with("us"));
    let (f, m, n) = if is_3rd {
        if orth.ends_with("is") {
            let base = orth.trim_end_matches("is");
            (orth.to_string(), orth.to_string(), format!("{base}e"))
        } else {
            // Neuter = same form for most 3rd (anceps, felix, etc.)
            (orth.to_string(), orth.to_string(), orth.to_string())
        }
    } else {
        let stem = orth.trim_end_matches("us");
        (format!("{stem}a"), orth.to_string(), format!("{stem}um"))
    };
    Some(AdjectiveRow {
        f,
        m,
        n,
        declension: if is_3rd { "III" } else { "I_II" }.into(),
    })
}

fn try_verb(orth: &str, itype: &str) -> Option<VerbRow> {
    let parts: Vec<&str> = itype.split(',').map(|s| s.trim()).collect();
    let mut inf_raw = String::new();
    let mut perf_raw = String::new();
    let mut sup_raw = String::new();
    for p in &parts {
        let p = p
            .trim()
            .trim_matches(|c: char| c == '.' || c == '—' || c == '-' || c.is_whitespace());
        // Take first of " or " alternatives.
        let p = match p.find(" or ") {
            Some(pos) => &p[..pos],
            None => p,
        };
        let p = p.strip_suffix(" sum").unwrap_or(p);
        let p = p.trim_start_matches('-');
        if p.is_empty() || p == "—" {
            continue;
        }
        if (p.ends_with("re") || p.ends_with("rī") || p == "ī") && inf_raw.is_empty() {
            inf_raw = p.to_string();
        } else if (p.ends_with('ī') || p.ends_with("sī")) && perf_raw.is_empty() {
            perf_raw = p.to_string();
        } else if (p.ends_with("tus")
            || p.ends_with("sus")
            || p.ends_with("xus")
            || p.ends_with("tum")
            || p.ends_with("sum"))
            && sup_raw.is_empty()
        {
            sup_raw = p.to_string();
        }
    }
    if inf_raw.is_empty() {
        return None;
    }
    let inf = build_full_infinitive(orth, &inf_raw);
    let conj = conj_from_inf(&inf);
    let perf = build_full_form(orth, &perf_raw);
    let supine = build_full_form(orth, &sup_raw);
    let is_deponent =
        inf.ends_with("ārī") || inf.ends_with("ērī") || inf.ends_with("īrī") || inf.ends_with('ī');
    if is_deponent {
        let dep_perf = if perf_raw.is_empty() && !sup_raw.is_empty() {
            build_full_form(orth, &sup_raw)
        } else if sup_raw.is_empty() && !perf_raw.is_empty() {
            build_full_form(orth, &perf_raw)
        } else if !perf_raw.is_empty() {
            build_full_form(orth, &perf_raw)
        } else {
            String::new()
        };
        Some(VerbRow {
            present: orth.to_string(),
            infinitive: inf,
            perfect: dep_perf,
            supine: String::new(),
            conjugation: conj,
        })
    } else {
        Some(VerbRow {
            present: orth.to_string(),
            infinitive: inf,
            perfect: perf,
            supine,
            conjugation: conj,
        })
    }
}

/// Reconstruct the full infinitive: present stem + raw suffix.
/// The NFD‑normalized suffix tells us the conjugation, which determines
/// how many characters to strip from the present stem.
fn build_full_infinitive(present: &str, inf_raw: &str) -> String {
    let p = nfc(present);
    let ir = nfc(inf_raw);
    let (pref, base) = p.rsplit_once('-').unwrap_or(("", &p));
    let stem = strip_personal(base);
    // If stripping consumed the entire base (e.g. "eō" → ""), or the
    // suffix is already a complete infinitive (starts with consonant,
    // has spaces), return the suffix as-is.
    if stem.is_empty() {
        return ir;
    }
    if pref.is_empty() {
        format!("{stem}{ir}")
    } else {
        format!("{pref}{stem}{ir}")
    }
}

/// Reconstruct a full perfect or supine form.
/// Returns empty string if raw is empty.
fn build_full_form(present: &str, raw: &str) -> String {
    if raw.is_empty() {
        return String::new();
    }
    let p = nfc(present);
    let r = nfc(raw);
    let (pref, base) = p.rsplit_once('-').unwrap_or(("", &p));
    let stem = strip_personal(base);

    // If the raw form starts with the stem's last character and the
    // remainder would be a known suffix, strip the overlapping consonant.
    // E.g. "suscēns" + "suī" → suffix "uī" → "suscēnsuī".
    let mut suffix = r.clone();
    if !is_known_suffix(&r) && !stem.is_empty() {
        let sc: Vec<char> = stem.chars().collect();
        let rc: Vec<char> = r.chars().collect();
        if rc.len() > 1 && sc.last() == rc.first() {
            let candidate: String = rc[1..].iter().collect();
            if is_known_suffix(&candidate) {
                suffix = candidate;
            }
        }
    }

    if is_known_suffix(&suffix) {
        if stem.is_empty() {
            return suffix;
        }
        let attached = attach_suffix(stem, &suffix);
        if pref.is_empty() {
            attached
        } else {
            format!("{pref}{attached}")
        }
    } else if pref.is_empty() {
        try_prefix_simplex(present, &r).unwrap_or(r.clone())
    } else {
        format!("{pref}{r}")
    }
}

/// Known Latin perfect/supine suffix forms that attach to the present stem.
fn is_known_suffix(raw: &str) -> bool {
    matches!(
        raw,
        "āvī"
            | "ēvī"
            | "īvī"
            | "uī"
            | "iī"
            | "sī"
            | "dī"
            | "tī"
            | "bī"
            | "xī"
            | "ātus"
            | "ātum"
            | "itus"
            | "itum"
            | "ūtus"
            | "ūtum"
            | "ētus"
            | "ētum"
            | "sus"
            | "sum"
            | "tus"
            | "tum"
    )
}

/// Attach a suffix to a stem, handling consonant overlap and assimilation.
/// e.g. stem "claud" + suffix "sī" → "clausī" (d+s → s)
///      stem "rād"  + suffix "sī" → "rāsī"
///      stem "vīs"  + suffix "sī" → "vīsī" (same char, overlap)
fn attach_suffix(stem: &str, suffix: &str) -> String {
    let mut stem_chars: Vec<char> = stem.chars().collect();
    let mut suffix_chars: Vec<char> = suffix.chars().collect();
    match (stem_chars.last(), suffix_chars.first()) {
        (Some(&'d' | &'t'), Some(&'s' | &'t')) => {
            stem_chars.pop();
        }
        (Some(&'g'), Some(&'s' | &'x')) => {
            stem_chars.pop();
        }
        (Some(a), Some(b)) if a == b => {
            suffix_chars.remove(0);
        }
        _ => {}
    }
    let s: String = stem_chars.iter().collect();
    let uf: String = suffix_chars.iter().collect();
    format!("{s}{uf}")
}

/// For non-hyphenated compound verbs, the itype gives the simplex perfect/
/// supine form. Detect overlap between present stem and raw form to recover
/// the prefix. E.g. "accolō" + "coluī" → overlap "col" → "accoluī".
fn try_prefix_simplex(present: &str, raw: &str) -> Option<String> {
    let pnf = nfc(present);
    let stem = strip_personal(&pnf);
    if stem.len() < 3 || raw.len() < 3 {
        return None;
    }
    let stem_chars: Vec<char> = stem.chars().collect();
    let raw_chars: Vec<char> = raw.chars().collect();
    for i in (2..=stem_chars.len().min(raw_chars.len())).rev() {
        let stem_tail: Vec<char> = stem_chars[stem_chars.len() - i..]
            .iter()
            .map(|c| demacron(*c))
            .collect();
        let raw_head: Vec<char> = raw_chars[..i].iter().map(|c| demacron(*c)).collect();
        if stem_tail == raw_head {
            let prefix: String = stem_chars[..stem_chars.len() - i].iter().collect();
            if !prefix.is_empty() {
                return Some(format!("{prefix}{raw}"));
            }
            return None;
        }
    }
    None
}

fn demacron(c: char) -> char {
    match c {
        'ā' => 'a',
        'Ā' => 'A',
        'ē' => 'e',
        'Ē' => 'E',
        'ī' => 'i',
        'Ī' => 'I',
        'ō' => 'o',
        'Ō' => 'O',
        'ū' => 'u',
        'Ū' => 'U',
        _ => c,
    }
}

fn strip_personal(s: &str) -> &str {
    // "eō" (to go): strip both chars.
    if s.ends_with("e\u{14d}") || s.ends_with("e\u{4d}") {
        return &s[..s.len() - "e\u{14d}".len()];
    }
    s.strip_suffix("ior")
        .or_else(|| s.strip_suffix("eor"))
        .or_else(|| s.strip_suffix("or"))
        .or_else(|| s.strip_suffix("i\u{14d}"))
        .or_else(|| s.strip_suffix("e\u{14d}"))
        .or_else(|| s.strip_suffix('\u{14d}'))
        .or_else(|| s.strip_suffix('o'))
        .unwrap_or(s)
}

/// Normalise NFD combining sequences to NFC for safe suffix matching.
fn nfc(s: &str) -> String {
    unicode_normalization::UnicodeNormalization::nfc(s).collect::<String>()
}

fn conj_from_inf(inf: &str) -> String {
    (if inf.ends_with("āre") || inf.ends_with("ārī") {
        "I"
    } else if inf.ends_with("ēre") {
        "II"
    } else if inf.ends_with("ere") {
        "III"
    } else if inf.ends_with("īre") || inf.ends_with("īrī") || inf == "ī" {
        "IV"
    } else if inf == "esse" {
        "Esse"
    } else {
        "III"
    })
    .to_string()
}

fn try_noun(orth: &str, itype: &str, r#gen: &str) -> Option<NounRow> {
    let gdr = match r#gen.chars().next() {
        Some('m') => "m",
        Some('f') => "f",
        Some('n') => "n",
        _ => return None,
    };
    let ending = itype
        .split(',')
        .next()
        .unwrap_or("")
        .trim()
        .trim_end_matches('.');
    let decl = decl_from_ending(ending)?;
    let genitive = build_genitive_form(orth, ending, &decl);
    Some(NounRow {
        nominative: orth.to_string(),
        genitive,
        gender: gdr.to_string(),
        declension: decl,
    })
}

fn decl_from_ending(ending: &str) -> Option<String> {
    let e: String = ending
        .chars()
        .map(|c| match c {
            'ā' => 'a',
            'ē' => 'e',
            'ī' => 'i',
            'ō' => 'o',
            'ū' => 'u',
            'ȳ' => 'y',
            _ => c,
        })
        .collect();
    let d = match e.as_str() {
        "ae" => "I",
        "ī" | "i" => "II",
        "is" | "inis" | "ium" => "III",
        "ūs" | "us" => "IV",
        "ēī" | "ei" | "eī" => "V",
        _ => {
            if e.starts_with("ae") {
                "I"
            } else if e.starts_with("is") || e.ends_with("is") {
                "III"
            } else if e.starts_with("ūs") || e.starts_with("us") {
                "IV"
            } else if e.contains("ei") || e.contains("ēī") {
                "V"
            } else if e.starts_with('i') {
                "II"
            } else {
                return None;
            }
        }
    };
    Some(d.to_string())
}

fn build_genitive_form(nom: &str, end: &str, decl: &str) -> String {
    let end = end.trim();
    if end.is_empty() {
        return nom.to_string();
    }
    let stem: String = match decl {
        "I" => {
            // Detect full genitive: e.g. "aedēs" + "aedium" → "aedium".
            let nom_nfc = nfc(nom);
            let itype_nfc = nfc(end);
            let nom_first2: String = nom_nfc.chars().take(2).map(demacron).collect();
            let itype_first2: String = itype_nfc.chars().take(2).map(demacron).collect();
            if nom_nfc.len() >= 2 && itype_nfc.len() >= 2 && nom_first2 == itype_first2 {
                return end.to_string();
            }
            nom.strip_suffix('a')
                .or_else(|| nom.strip_suffix("ēs"))
                .or_else(|| nom.strip_suffix("ās"))
                .unwrap_or(nom)
                .to_string()
        }
        "II" => {
            let nom_nfc = nfc(nom);
            let itype_nfc = nfc(end);
            let nom_first2: String = nom_nfc.chars().take(2).map(demacron).collect();
            let itype_first2: String = itype_nfc.chars().take(2).map(demacron).collect();
            if nom_nfc.len() >= 2 && itype_nfc.len() >= 2 && nom_first2 == itype_first2 {
                return end.to_string();
            }
            nom.strip_suffix("us")
                .or_else(|| nom.strip_suffix("um"))
                .or_else(|| nom.strip_suffix("os"))
                .or_else(|| nom.strip_suffix("on"))
                .or_else(|| nom.strip_suffix("is"))
                .unwrap_or(nom)
                .to_string()
        }
        "III" => {
            // Detect whether itype is the full genitive form (e.g. "vōcis"
            // for "vōx") or just a suffix (e.g. "ōris" for "āctor").
            // Full genitive: itype shares the first 2 chars of the nominative
            // (after demacron).  Suffix: itype starts with different chars.
            let nom_nfc = nfc(nom);
            let itype_nfc = nfc(end);
            let nom_first2: String = nom_nfc.chars().take(2).map(demacron).collect();
            let itype_first2: String = itype_nfc.chars().take(2).map(demacron).collect();
            let is_full_genitive =
                nom_nfc.len() >= 2 && itype_nfc.len() >= 2 && nom_first2 == itype_first2;
            if is_full_genitive {
                return end.to_string();
            }

            let itype_chars: Vec<char> = itype_nfc.chars().collect();
            let itype_starts_with_vowel = itype_chars.first().map_or(false, |c| {
                matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'ā' | 'ē' | 'ī' | 'ō' | 'ū')
            });

            // Strip the nominative ending to get the stem.
            let chs: Vec<char> = nom_nfc.chars().collect();
            let n = chs.len();
            if n == 0 {
                return nom_nfc;
            }
            let last2 = if n >= 2 {
                format!("{}{}", chs[n - 2], chs[n - 1])
            } else {
                String::new()
            };
            let last = chs[n - 1];

            let (stem_chars, stripped_len) = if last2.ends_with("or")
                || last2.ends_with("ōr")
                || last2.ends_with("er")
                || last2.ends_with("en")
                || last2.ends_with("ōn")
                || last2.ends_with("ēs")
                || last2.ends_with("es")
                || last2.ends_with("ās")
                || last2.ends_with("as")
                || last2.ends_with("ūs")
                || last2.ends_with("us")
                || last2.ends_with("is")
                || last2.ends_with("ns")
                || last2.ends_with("ut")
                || last2.ends_with("ur")
            {
                (&chs[..n - 2], 2)
            } else if last == 'ō'
                || last == 'ē'
                || last == 'ā'
                || last == 'o'
                || last == 'e'
                || last == 'a'
                || last == 's'
                || last == 'x'
                || last == 'n'
            {
                (&chs[..n - 1], 1)
            } else {
                (&chs[..], 0)
            };

            let stem: String = stem_chars.iter().collect();

            if itype_starts_with_vowel && itype_chars.len() >= 2 && stripped_len > 0 {
                // The itype encodes stem_vowel + stem_consonant + suffix.
                // E.g. "īnis" → stem vowel "ī" + stem consonant "n" + suffix "is".
                let stem = if stripped_len == 1 {
                    let nom_vowel = chs[n - 1];
                    let stem_cons = itype_chars[1];
                    let stripped_str: String = stem_chars.iter().collect();
                    format!("{stripped_str}{nom_vowel}{stem_cons}")
                } else {
                    let itype_stem: String = itype_chars[..2].iter().collect();
                    let stripped_str: String = stem_chars.iter().collect();
                    format!("{stripped_str}{itype_stem}")
                };
                let suffix: String = itype_chars[2..].iter().collect();
                return format!("{stem}{suffix}");
            }

            // Consonant-starting suffix: use the stripped stem.
            // If no overlap with the stripped stem, try the full nominative
            // (for Greek forms like "Acherōn" where the ending is part of stem).
            let stem_chars_vec: Vec<char> = stem.chars().collect();
            let mut overlap = 0usize;
            for i in (1..=stem_chars_vec.len().min(itype_chars.len())).rev() {
                let stem_suffix: Vec<char> = stem_chars_vec[stem_chars_vec.len() - i..]
                    .iter()
                    .map(|c| demacron(*c))
                    .collect();
                let itype_prefix: Vec<char> =
                    itype_chars[..i].iter().map(|c| demacron(*c)).collect();
                if stem_suffix == itype_prefix {
                    overlap = i;
                    break;
                }
            }
            if overlap > 0 {
                let suffix: String = itype_chars[overlap..].iter().collect();
                return format!("{stem}{suffix}");
            }
            // Fall back to full nominative for Greek loans.
            let nom_chars: Vec<char> = nom_nfc.chars().collect();
            let mut nom_overlap = 0usize;
            for i in (1..=nom_chars.len().min(itype_chars.len())).rev() {
                let nom_suffix: Vec<char> = nom_chars[nom_chars.len() - i..]
                    .iter()
                    .map(|c| demacron(*c))
                    .collect();
                let itype_prefix: Vec<char> =
                    itype_chars[..i].iter().map(|c| demacron(*c)).collect();
                if nom_suffix == itype_prefix {
                    nom_overlap = i;
                    break;
                }
            }
            let suffix: String = itype_chars[nom_overlap..].iter().collect();
            return format!("{nom_nfc}{suffix}");
        }
        "IV" => nom.strip_suffix("us").unwrap_or(nom).to_string(),
        "V" => nom.strip_suffix("es").unwrap_or(nom).to_string(),
        _ => nom.to_string(),
    };
    format!("{stem}{end}")
}

fn ending_stripped(end: &str) -> String {
    end.chars()
        .map(|c| match c {
            'ā' => 'a',
            'ē' => 'e',
            'ī' => 'i',
            'ō' => 'o',
            'ū' => 'u',
            _ => c,
        })
        .collect()
}

// ── CSV writers ──────────────────────────────────────────────────────────

struct Writers {
    nouns: BufWriter<File>,
    verbs: BufWriter<File>,
    adjs: BufWriter<File>,
    preps: BufWriter<File>,
}

impl Writers {
    fn new(dir: &Path) -> std::io::Result<Self> {
        std::fs::create_dir_all(dir)?;
        let mut s = Self {
            nouns: BufWriter::new(File::create(dir.join("latin-nouns.csv"))?),
            verbs: BufWriter::new(File::create(dir.join("latin-verbs.csv"))?),
            adjs: BufWriter::new(File::create(dir.join("latin-adjectives.csv"))?),
            preps: BufWriter::new(File::create(dir.join("latin-prepositions.csv"))?),
        };
        writeln!(s.nouns, "nominative,genitive,gender,declension")?;
        writeln!(s.verbs, "present,infinitive,perfect,supine,conjugation")?;
        writeln!(s.adjs, "f,m,n,declension")?;
        writeln!(s.preps, "word,cases")?;
        Ok(s)
    }

    fn put(&mut self, r: &Row) -> std::io::Result<()> {
        match r {
            Row::Noun(n) => writeln!(
                self.nouns,
                "{},{},{},{}",
                csv_quote(&strip_hyphens(&n.nominative)),
                csv_quote(&strip_hyphens(&n.genitive)),
                n.gender,
                n.declension
            )?,
            Row::Verb(v) => writeln!(
                self.verbs,
                "{},{},{},{},{}",
                csv_quote(&strip_hyphens(&v.present)),
                csv_quote(&strip_hyphens(&v.infinitive)),
                csv_quote(&strip_hyphens(&v.perfect)),
                csv_quote(&strip_hyphens(&v.supine)),
                v.conjugation
            )?,
            Row::Adjective(a) => writeln!(
                self.adjs,
                "{},{},{},{}",
                csv_quote(&strip_hyphens(&a.f)),
                csv_quote(&strip_hyphens(&a.m)),
                csv_quote(&strip_hyphens(&a.n)),
                a.declension
            )?,
            Row::Preposition(p) => {
                writeln!(self.preps, "{},{}", csv_quote(&p.word), csv_quote(&p.cases))?
            }
            Row::Skip => {}
        }
        Ok(())
    }

    fn flush_all(&mut self) -> std::io::Result<()> {
        self.nouns.flush()?;
        self.verbs.flush()?;
        self.adjs.flush()?;
        self.preps.flush()
    }
}

fn csv_quote(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn strip_hyphens(s: &str) -> String {
    // Strip " or X" alternatives first, then remove hyphens and collapse spaces.
    let s = match s.find(" or ") {
        Some(p) => s[..p].to_string(),
        None => s.to_string(),
    };
    s.replace(
        [
            '-', '\u{2010}', '\u{2011}', '\u{2012}', '\u{2013}', '\u{2014}',
        ],
        "",
    )
    .split_whitespace()
    .collect::<Vec<_>>()
    .join("")
}

// ── XML field extraction from one <entry> fragment ───────────────────────

struct Fields {
    orth: String,
    pos: String,
    itype: String,
    gender: String,
    tr: String,
}

fn extract_fields(frag: &str) -> Fields {
    let mut reader = Reader::from_str(frag);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut orth = String::new();
    let mut pos = String::new();
    let mut itype = String::new();
    let mut gender = String::new();
    let mut tr = String::new();
    let mut current: Option<&str> = None;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                current = match e.name().as_ref() {
                    b"orth" => Some("orth"),
                    b"pos" => Some("pos"),
                    b"itype" => Some("itype"),
                    b"tr" if tr.is_empty() => Some("tr"),
                    b"gen" => Some("gen"),
                    _ => current,
                };
            }
            Ok(Event::End(ref e)) => {
                let en = e.name();
                if current == Some("orth") && en.as_ref() == b"orth" {
                    current = None;
                } else if current == Some("pos") && en.as_ref() == b"pos" {
                    current = None;
                } else if current == Some("itype") && en.as_ref() == b"itype" {
                    current = None;
                } else if current == Some("tr") && en.as_ref() == b"tr" {
                    current = None;
                } else if current == Some("gen") && en.as_ref() == b"gen" {
                    current = None;
                }
            }
            Ok(Event::Text(ref t)) => {
                if let Ok(s) = t.decode() {
                    match current {
                        Some("orth") => orth.push_str(&s),
                        Some("pos") => pos.push_str(&s),
                        Some("itype") => itype.push_str(&s),
                        Some("gen") => gender.push_str(&s),
                        Some("tr") => tr.push_str(&s),
                        _ => {}
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(_) => break,
            _ => {}
        }
        buf.clear();
    }
    Fields {
        orth,
        pos,
        itype,
        gender,
        tr,
    }
}

// ── main ─────────────────────────────────────────────────────────────────

#[derive(Default)]
struct Stats {
    n: usize,
    v: usize,
    a: usize,
    p: usize,
    skip: usize,
}

impl Stats {
    fn tally(&mut self, r: &Row) {
        match r {
            Row::Noun(_) => self.n += 1,
            Row::Verb(_) => self.v += 1,
            Row::Adjective(_) => self.a += 1,
            Row::Preposition(_) => self.p += 1,
            Row::Skip => self.skip += 1,
        }
    }
    fn report(&self) {
        println!("\n{}", "=".repeat(60));
        println!(
            "  nouns: {}  verbs: {}  adjs: {}  preps: {}  skipped: {}  → {} total",
            self.n,
            self.v,
            self.a,
            self.p,
            self.skip,
            self.n + self.v + self.a + self.p + self.skip
        );
        println!("{}", "=".repeat(60));
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let xml_path = args
        .get(1)
        .map_or("./data/latin/lewis_elementary-latin-dictionary.xml", |s| s);
    let out_dir = Path::new(args.get(2).map_or("./data/latin", |s| s));
    let start_ln: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(0);
    let end_ln: usize = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(0);

    let file = File::open(xml_path)?;
    let reader = BufReader::new(file);
    let mut rows: Vec<Row> = Vec::new();
    let mut buf = String::new();
    let mut ln = 0usize;
    let mut collecting = false;

    for line in reader.lines() {
        ln += 1;
        let l = line?;
        if !collecting {
            if l.contains("<entry ") || l.contains("<entry>") {
                if in_range(ln, start_ln, end_ln) {
                    collecting = true;
                    buf.clear();
                    buf.push_str(&l);
                    if l.contains("</entry>") {
                        collect_row(&buf, &mut rows);
                        collecting = false;
                    }
                }
            }
        } else {
            buf.push('\n');
            buf.push_str(&l);
            if l.contains("</entry>") {
                collect_row(&buf, &mut rows);
                collecting = false;
            }
        }
    }

    fill_compound_verbs(&mut rows);

    let mut w = Writers::new(out_dir)?;
    let mut stats = Stats::default();
    for row in &rows {
        stats.tally(row);
        if let Err(e) = w.put(row) {
            eprintln!("write error: {e}");
        }
    }
    w.flush_all()?;
    stats.report();
    Ok(())
}

fn in_range(line: usize, start: usize, end: usize) -> bool {
    if start == 0 && end == 0 {
        return true;
    }
    if start != 0 && line < start {
        return false;
    }
    if end != 0 && line > end {
        return false;
    }
    true
}

fn collect_row(buf: &str, rows: &mut Vec<Row>) {
    let frag = if let Some(s) = buf.find("<entry") {
        if let Some(e) = buf.rfind("</entry>") {
            &buf[s..e + 8]
        } else {
            buf
        }
    } else {
        return;
    };
    let f = extract_fields(frag);
    let o = clean_field_keep_hyphens(&f.orth);
    let p = clean_field(&f.pos);
    let i = f.itype.trim().replace('\n', "").replace('\r', "");
    let g = clean_field(&f.gender);
    let t = clean_field(&f.tr);
    rows.push(classify(&o, &p, &i, &g, &t));
}

// ── compound verb back‑filling ──────────────────────────────────────────

const PREFIXES: &[&str] = &[
    "circum", "praeter", "super", "subter", "inter", "intro", "ab", "abs", "ad", "ante", "com",
    "con", "co", "contra", "de", "dis", "di", "ex", "e", "extra", "in", "im", "ob", "per", "post",
    "prae", "pro", "re", "red", "se", "sub", "sus", "trans", "tra",
];

fn fill_compound_verbs(rows: &mut Vec<Row>) {
    let mut base_perf: HashMap<String, String> = HashMap::new();
    let mut base_sup: HashMap<String, String> = HashMap::new();

    for row in rows.iter() {
        if let Row::Verb(v) = row {
            let clean = strip_hyphens(&v.present);
            let stem = strip_personal(&nfc(&clean)).to_string();
            if !v.perfect.is_empty() {
                base_perf.insert(stem.clone(), v.perfect.clone());
            }
            if !v.supine.is_empty() {
                base_sup.insert(stem.clone(), v.supine.clone());
            }
        }
    }

    for row in rows.iter_mut() {
        let Row::Verb(v) = row else { continue };
        if !v.perfect.is_empty() && !v.supine.is_empty() {
            continue;
        }

        let clean = strip_hyphens(&v.present);
        let cpd_stem = {
            let n = nfc(&clean);
            strip_personal(&n).to_string()
        };

        'outer: for &pref in PREFIXES {
            let pref_nfc = nfc(pref);
            if let Some(leftover) = cpd_stem.strip_prefix(&pref_nfc) {
                if leftover.len() < 2 {
                    continue;
                }
                if let Some(bp_stem) = resolve_base_stem(leftover, &base_perf, &base_sup) {
                    if v.perfect.is_empty() {
                        if let Some(bperf) = base_perf.get(&bp_stem) {
                            let suffix = extract_suffix(&bp_stem, bperf);
                            v.perfect = attach_suffix(&cpd_stem, &suffix);
                        }
                    }
                    if v.supine.is_empty() {
                        if let Some(bsup) = base_sup.get(&bp_stem) {
                            let suffix = extract_suffix(&bp_stem, bsup);
                            v.supine = attach_suffix(&cpd_stem, &suffix);
                        }
                    }
                    break 'outer;
                }
            }
        }
    }
}

fn resolve_base_stem(
    leftover: &str,
    base_perf: &HashMap<String, String>,
    base_sup: &HashMap<String, String>,
) -> Option<String> {
    let exists = |bp: &str| -> bool {
        let s = strip_personal(bp);
        base_perf.contains_key(s) || base_sup.contains_key(s)
    };
    if exists(leftover) {
        return Some(strip_personal(leftover).to_string());
    }
    for suffix in ["ō", "or", "eō"] {
        let bp = format!("{leftover}{suffix}");
        if exists(&bp) {
            return Some(strip_personal(&bp).to_string());
        }
    }
    let ch: Vec<char> = leftover.chars().collect();
    let vp: Vec<usize> = ch
        .iter()
        .enumerate()
        .filter(|(_, c)| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u' | 'ā' | 'ē' | 'ī' | 'ō' | 'ū'))
        .map(|(i, _)| i)
        .collect();
    for &p in &vp {
        for &r in &['a', 'i', 'e', 'u', 'ā', 'ī', 'ē', 'ū'] {
            if r == ch[p] {
                continue;
            }
            let mut vc = ch.clone();
            vc[p] = r;
            let vs: String = vc.iter().collect();
            if exists(&vs) {
                return Some(vs);
            }
            for sfx in ["ō", "or", "eō"] {
                let vsp = format!("{vs}{sfx}");
                if exists(&vsp) {
                    return Some(strip_personal(&vsp).to_string());
                }
            }
        }
    }
    None
}

fn extract_suffix(base_stem: &str, full: &str) -> String {
    if full.is_empty() {
        return String::new();
    }
    if let Some(s) = full.strip_prefix(base_stem) {
        return s.to_string();
    }
    let ac: Vec<char> = base_stem.chars().collect();
    let bc: Vec<char> = full.chars().collect();
    for i in (1..=ac.len().min(bc.len())).rev() {
        if ac[ac.len() - i..] == bc[..i] {
            return full[i..].to_string();
        }
    }
    full.to_string()
}
