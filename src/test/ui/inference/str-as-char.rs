// When a char literal is used where a str should be,
// suggest changing to double quotes.

// run-rustfix

fn main() {
    let _: &str = 'a'; //~ ERROR mismatched types
}
