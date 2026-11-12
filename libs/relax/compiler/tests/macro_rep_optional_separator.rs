// A repetition that can go on and end on the same token: `$( .. $(,)? ),*`. A `,` after an
// item is the inner `$(,)?` or the separator of the next item. rustc reads it both ways
// (rustc_expand mbe/macro_parser.rs parse_tt: an NFA, a thread per reading; SequenceSep pushes
// one that ends the repetition and one that goes past the separator) and the next token keeps
// one. hayagriva 0.10.1's `entry!` (src/lib.rs:253, reached through typst) is written so --
// before: "no rule of `entry!` matches these arguments" (the matcher took `$(,)?` and stayed).
macro_rules! entry {
    ($(
        $(#[doc = $doc:literal])*
        $s:literal => $i:ident : $t:ty
        $(| $d:ty)? $(,)?
    ),*) => {
        $( $(#[doc = $doc])* pub fn $i() -> (&'static str, usize) { ($s, core::mem::size_of::<$t>()) } )*
        pub fn count() -> usize { [$($s),*].len() }
    };
}
entry! {
    /// One.
    "one" => one: u8,
    /// Two.
    /// Its type has a deref form.
    "two" => two: [u16; 2] | [u16],
    "three" => three: u32,
}

// the same shape with no trailing separator, and with nothing at all
macro_rules! sum {
    ($( $x:expr $(;)? );*) => { 0 $(+ $x)* };
}

// a `?` whose token also follows it, and one inside a group
macro_rules! tail {
    ($($x:literal)? $(,)? , $y:literal) => { (0 $(+ $x)?, $y) };
    (($( $x:literal $(,)? ),*) $y:literal) => { (0 $(+ $x)*, $y) };
}

// an arm before the one that matches still fails: the search does not accept what no reading matches
macro_rules! pick {
    ($( $x:literal $(,)? ),* ; stop) => { 1 $(+ $x)* };
    ($( $x:literal $(,)? ),*) => { 100 $(+ $x)* };
}

// a long repetition: its length is no depth of the matcher's stack (8000 `tt`)
macro_rules! len { ($($t:tt)*) => { [$(stringify!($t)),*].len() }; }
macro_rules! x10 { ($($t:tt)*) => { len!($($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)*) }; }
macro_rules! x100 { ($($t:tt)*) => { x10!($($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)*) }; }
macro_rules! x1000 { ($($t:tt)*) => { x100!($($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)* $($t)*) }; }

#[test]
fn run() {
    assert_eq!((one(), two(), three()), (("one", 1), ("two", 4), ("three", 4)));
    assert_eq!(count(), 3);
    assert_eq!(sum!(1; 2; 3), 6);
    assert_eq!(sum!(1; 2; 3;), 6);
    assert_eq!(sum!(), 0);
    assert_eq!(tail!(1, , 2), (1, 2));
    assert_eq!(tail!(, 2), (0, 2));
    assert_eq!(tail!((1, 2, 3,) 4), (6, 4));
    assert_eq!(pick!(1, 2, ; stop), 4);
    assert_eq!(pick!(1, 2,), 103);
    assert_eq!(x1000!(a b c d e f g h), 8000);
    println!("D ok {}", count());
}
