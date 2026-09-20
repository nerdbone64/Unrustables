//! A crate containing things that should've been in Rust already.

pub mod general;

#[test]
fn map_test() {
    let m = map![
        "a" => 69.42,
        "b" => 420.0,
        "c" => 2.1,
        "d" => -87.0,
        "e" => -666.6
    ];
    for (i, v) in m {
        println!("{} maps to {}", i, v);
    }
}
