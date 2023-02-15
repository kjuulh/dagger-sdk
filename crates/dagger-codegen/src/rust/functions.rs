use convert_case::{Case, Casing};
use dagger_core::introspection::FullTypeFields;

pub fn format_name(s: &str) -> String {
    s.to_case(Case::Pascal)
}

pub fn field_options_struct_name(field: &FullTypeFields) -> Option<String> {
    field
        .parent_type
        .as_ref()
        .map(|p| p.name.as_ref())
        .flatten()
        .zip(field.name.as_ref())
        .map(|(parent_name, field_name)| format!("{parent_name}{field_name}Opts"))
}
