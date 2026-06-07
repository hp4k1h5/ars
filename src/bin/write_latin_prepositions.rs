use ars::api::latin::prepositions::create_latin_preposition;
use ars::establish_cnx;
use ars::grammar::latin::noun::Case;
use std::error::Error;

#[derive(Debug)]
struct PrepositionError {
    line_number: usize,
    word: Option<String>,
    error: String,
}

fn parse_cases(s: &str) -> Result<Vec<Case>, String> {
    let mut cases = Vec::new();
    for part in s.split(';') {
        match part.trim() {
            "acc." => cases.push(Case::Accusative),
            "abl." => cases.push(Case::Ablative),
            "gen." => cases.push(Case::Genitive),
            "dat." => cases.push(Case::Dative),
            "nom." => cases.push(Case::Nominative),
            "voc." => cases.push(Case::Vocative),
            other => return Err(format!("unknown case abbreviation: '{}'", other)),
        }
    }
    Ok(cases)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let csv_path = if args.len() > 1 {
        &args[1]
    } else {
        "./data/latin/latin-prepositions.csv"
    };

    let mut cnx = establish_cnx();
    let mut rdr = csv::Reader::from_path(csv_path)?;

    let mut successful_writes: Vec<String> = Vec::new();
    let mut errors: Vec<PrepositionError> = Vec::new();

    for result in rdr.records() {
        let record = match result {
            Ok(r) => r,
            Err(e) => {
                errors.push(PrepositionError {
                    line_number: 0,
                    word: None,
                    error: format!("CSV parsing error: {}", e),
                });
                continue;
            }
        };

        let line_number = record.position().map(|p| p.line() as usize).unwrap_or(0);
        let word = record.get(0).unwrap_or("").trim().to_string();
        if word.is_empty() {
            continue;
        }

        let cases_str = record.get(1).unwrap_or("").trim();
        let cases = match parse_cases(cases_str) {
            Ok(c) => c,
            Err(e) => {
                errors.push(PrepositionError {
                    line_number,
                    word: Some(word),
                    error: format!("Case parsing error: {}", e),
                });
                continue;
            }
        };

        match create_latin_preposition(&mut cnx, &word, &cases) {
            Ok(prep) => {
                successful_writes.push(format!(
                    "{} ({:?})",
                    prep.word,
                    Ok::<Option<uuid::Uuid>, ()>(prep.id)
                ));
            }
            Err(e) => {
                errors.push(PrepositionError {
                    line_number,
                    word: Some(word),
                    error: format!("Database error: {}", e),
                });
            }
        }
    }

    println!("\n{}", "=".repeat(80));
    println!("IMPORT SUMMARY");
    println!("{}", "=".repeat(80));

    println!(
        "\n\u{2713} Successfully imported {} prepositions:",
        successful_writes.len()
    );
    if !successful_writes.is_empty() {
        for (i, prep) in successful_writes.iter().enumerate() {
            println!("  {}. {}", i + 1, prep);
        }
    }

    if !errors.is_empty() {
        println!("\n\u{2717} Failed to import {} prepositions:", errors.len());
        for err in &errors {
            println!("\n  Line {}: {}", err.line_number, err.error);
            if let Some(word) = &err.word {
                println!("    Data: word={}", word);
            }
        }
    }

    println!("\n{}", "=".repeat(80));
    println!(
        "Total: {} succeeded, {} failed, {} total",
        successful_writes.len(),
        errors.len(),
        successful_writes.len() + errors.len()
    );
    println!("{}", "=".repeat(80));

    if !errors.is_empty() {
        eprintln!("\nWarning: Some prepositions failed to import.");
        std::process::exit(1);
    }

    Ok(())
}
