use ars::api::latin::create_latin_noun;
use ars::{establish_cnx, grammar::latin::noun::NewNoun};
use std::error::Error;

#[derive(Debug)]
struct NounError {
    line_number: usize,
    noun_data: Option<NewNoun>,
    error: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let csv_path = if args.len() > 1 {
        &args[1]
    } else {
        "./data/latin-nouns.csv"
    };

    let mut cnx = establish_cnx();
    let mut rdr = csv::Reader::from_path(csv_path)?;

    let mut successful_writes: Vec<String> = Vec::new();
    let mut errors: Vec<NounError> = Vec::new();

    for (line_number, result) in rdr.deserialize::<NewNoun>().enumerate() {
        match result {
            Ok(new_noun) => {
                match create_latin_noun(
                    &mut cnx,
                    &new_noun.declension,
                    &new_noun.nominative,
                    &new_noun.genitive,
                    &new_noun.gender,
                ) {
                    Ok(noun) => {
                        successful_writes.push(format!(
                            "{} ({:?})",
                            noun.nominative,
                            Ok::<Option<uuid::Uuid>, ()>(noun.id)
                        ));
                    }
                    Err(e) => {
                        errors.push(NounError {
                            line_number,
                            noun_data: Some(new_noun),
                            error: format!("Database error: {}", e),
                        });
                    }
                }
            }
            Err(e) => {
                errors.push(NounError {
                    line_number,
                    noun_data: None,
                    error: format!("CSV parsing error: {}", e),
                });
            }
        }
    }

    // Print summary
    println!("\n{}", "=".repeat(80));
    println!("IMPORT SUMMARY");
    println!("{}", "=".repeat(80));

    println!(
        "\n✓ Successfully imported {} nouns:",
        successful_writes.len()
    );
    if !successful_writes.is_empty() {
        for (i, noun) in successful_writes.iter().enumerate() {
            println!("  {}. {}", i + 1, noun);
        }
    }

    if !errors.is_empty() {
        println!("\n✗ Failed to import {} nouns:", errors.len());
        for err in &errors {
            println!("\n  Line {}: {}", err.line_number, err.error);
            if let Some(noun_data) = &err.noun_data {
                println!(
                    "    Data: declension={:?}, nominative={}, genitive={}, gender={:?}",
                    noun_data.declension,
                    noun_data.nominative,
                    noun_data.genitive,
                    noun_data.gender,
                );
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

    // Return error if any imports failed
    if !errors.is_empty() {
        eprintln!("\nWarning: Some verbs failed to import.");
        std::process::exit(1);
    }

    Ok(())
}
