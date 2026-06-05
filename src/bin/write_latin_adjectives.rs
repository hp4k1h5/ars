use ars::api::latin::create_latin_adjective;
use ars::{establish_cnx, grammar::latin::adjective::NewAdjective};
use std::error::Error;

#[derive(Debug)]
struct AdjectiveError {
    line_number: usize,
    adj_data: Option<NewAdjective>,
    error: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let csv_path = if args.len() > 1 {
        &args[1]
    } else {
        "./data/latin/latin-adjectives.csv"
    };
    let mut rdr = csv::Reader::from_path(csv_path)?;

    let mut cnx = establish_cnx();
    let mut successful_writes: Vec<String> = Vec::new();
    let mut errors: Vec<AdjectiveError> = Vec::new();
    for (line_number, result) in rdr.deserialize::<NewAdjective>().enumerate() {
        match result {
            Ok(new_adj) => {
                match create_latin_adjective(
                    &mut cnx,
                    &new_adj.declension,
                    &new_adj.f,
                    &new_adj.m,
                    &new_adj.n,
                ) {
                    Ok(adj) => {
                        successful_writes.push(format!(
                            "{} ({:?})",
                            adj.n,
                            Ok::<Option<uuid::Uuid>, ()>(adj.id)
                        ));
                    }
                    Err(e) => {
                        errors.push(AdjectiveError {
                            line_number,
                            adj_data: Some(new_adj),
                            error: format!("Database error: {}", e),
                        });
                    }
                }
            }
            Err(e) => {
                errors.push(AdjectiveError {
                    line_number,
                    adj_data: None,
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
        "\n✓ Successfully imported {} adjectives:",
        successful_writes.len()
    );
    if !successful_writes.is_empty() {
        for (i, adj) in successful_writes.iter().enumerate() {
            println!("  {}. {}", i + 1, adj);
        }
    }

    if !errors.is_empty() {
        println!("\n✗ Failed to import {} adjectives:", errors.len());
        for err in &errors {
            println!("\n  Line {}: {}", err.line_number, err.error);
            if let Some(adj_data) = &err.adj_data {
                println!(
                    "    Data: declension={:?}, f={}, m={}, n={:?}",
                    adj_data.declension, adj_data.f, adj_data.m, adj_data.n,
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
        eprintln!("\nWarning: Some ajectives failed to import.");
        std::process::exit(1);
    }

    Ok(())
}
