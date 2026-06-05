use ars::api::latin::create_latin_verb;
use ars::{establish_cnx, grammar::latin::verb::NewVerbOwned};
use std::error::Error;

#[derive(Debug)]
struct VerbError {
    line_number: usize,
    verb_data: Option<NewVerbOwned>,
    error: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let csv_path = if args.len() > 1 {
        &args[1]
    } else {
        "./data/latin/latin-verbs.csv"
    };

    let mut cnx = establish_cnx();
    let mut rdr = csv::Reader::from_path(csv_path)?;

    let mut successful_writes: Vec<String> = Vec::new();
    let mut errors: Vec<VerbError> = Vec::new();

    for (line_number, result) in rdr.deserialize::<NewVerbOwned>().enumerate() {
        match result {
            Ok(new_verb) => {
                match create_latin_verb(
                    &mut cnx,
                    &new_verb.conjugation,
                    &new_verb.present,
                    &new_verb.infinitive,
                    &new_verb.perfect,
                    &new_verb.supine,
                ) {
                    Ok(verb) => {
                        successful_writes.push(format!(
                            "{} ({:?})",
                            verb.present,
                            Ok::<Option<uuid::Uuid>, ()>(verb.id)
                        ));
                    }
                    Err(e) => {
                        errors.push(VerbError {
                            line_number,
                            verb_data: Some(new_verb),
                            error: format!("Database error: {}", e),
                        });
                    }
                }
            }
            Err(e) => {
                errors.push(VerbError {
                    line_number,
                    verb_data: None,
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
        "\n✓ Successfully imported {} verbs:",
        successful_writes.len()
    );
    if !successful_writes.is_empty() {
        for (i, verb) in successful_writes.iter().enumerate() {
            println!("  {}. {}", i + 1, verb);
        }
    }

    if !errors.is_empty() {
        println!("\n✗ Failed to import {} verbs:", errors.len());
        for err in &errors {
            println!("\n  Line {}: {}", err.line_number, err.error);
            if let Some(verb_data) = &err.verb_data {
                println!(
                    "    Data: present={}, infinitive={}, perfect={}, supine={}",
                    verb_data.present, verb_data.infinitive, verb_data.perfect, verb_data.supine
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
