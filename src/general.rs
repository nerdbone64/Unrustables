/// Helper for `HashMap`s. Similar to `vec![...]`.
/// Example:
/// ```rust
/// use unrustables::map;
/// let m = map!["a" => 3.14, "b" => 69.42];
/// for (i, v) in m {
///     println!("{} maps to {}", i, v);
/// }
/// ```
#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        use std::collections::HashMap;
        let mut map = HashMap::new();
        $(map.insert(
            $key,
            $value,
        );)*
        map
    }};
}