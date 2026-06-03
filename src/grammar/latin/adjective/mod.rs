use crate::grammar::latin::noun::*;

pub mod i_ii;

#[derive(Debug)]
pub struct Adjective<'a> {
    pub f: NounInstance<'a>,
    pub m: NounInstance<'a>,
    pub n: NounInstance<'a>,
}