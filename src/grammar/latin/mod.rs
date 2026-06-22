// pub mod adjective;
pub mod adjective;
pub mod noun;
pub mod path;
pub mod preposition;
pub mod verb;
pub mod word;

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize)]
pub enum Number {
    Singular,
    Plural,
}
