// pub mod adjective;
pub mod adjective;
pub mod noun;
pub mod preposition;
pub mod verb;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Number {
    Singular,
    Plural,
}
