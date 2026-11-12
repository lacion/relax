// A path's segment is what the importer can see (rustc resolves each segment of a path with
// the visibility of the place that writes it). `use pathdep::pathdep;` imports the macro alone:
// the module of that name is private to its crate, so `pathdep` here is still the crate.
// icu_collections writes `use zerovec::{.., zerovec};` and `#[zerovec::make_varule(..)]` --
// before: the import bound the private module, the attribute was looked for in it and never
// run, and the type it makes was an unresolved import.
use pathdep::pathdep;

pathdep::make!(answer);

fn main() {
    println!("private module path ok {} {} {}", pathdep!(), answer(), pathdep::one());
}
