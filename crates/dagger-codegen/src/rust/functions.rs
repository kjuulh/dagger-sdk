use convert_case::{Case, Casing};

pub fn format_name(s: &str) -> String {
    s.to_case(Case::Pascal)
}
