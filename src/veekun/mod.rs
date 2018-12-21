//! Tools for dealing with the Veekun CSV files included with the library.

pub mod csv;
pub mod repr;

pub fn to_pascal_case(s: &str) -> String {
    let mut builder = String::new();
    for word in s.split('-') {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            builder.extend(first.to_uppercase());
            builder.extend(chars);
        }
    }
    builder
}
