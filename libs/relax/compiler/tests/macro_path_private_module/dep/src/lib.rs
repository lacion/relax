//! A private module and an exported macro of the crate's own name, as zerovec has
//! (`mod zerovec;` and `macro_rules! zerovec`).
mod pathdep {
    pub fn inside() -> u32 { 1 }
}
pub fn one() -> u32 { pathdep::inside() }

#[macro_export]
macro_rules! pathdep {
    () => { 7u32 };
}

#[macro_export]
macro_rules! make {
    ($n:ident) => { pub fn $n() -> u32 { 42 } };
}
