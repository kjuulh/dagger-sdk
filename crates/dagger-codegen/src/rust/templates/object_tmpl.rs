use dagger_core::introspection::{FullType, FullTypeFields};
use genco::prelude::rust;
use genco::quote;

use crate::rust::functions::format_name;
use crate::utility::OptionExt;

pub fn render_object(t: &FullType) -> eyre::Result<rust::Tokens> {
    Ok(quote! {
        pub struct $(t.name.pipe(|s| format_name(s))) {

        }

        $(t.fields.pipe(render_optional_args))

        impl $(t.name.pipe(|s| format_name(s))) {

        }
    })
}

fn render_optional_args(fields: &Vec<FullTypeFields>) -> Option<rust::Tokens> {
    let rendered_fields = fields
        .iter()
        .map(render_optional_arg)
        .flatten()
        .collect::<Vec<_>>();

    if rendered_fields.len() == 0 {
        None
    } else {
        Some(quote! {
            $(for field in rendered_fields join ($['\r']) => $field)
        })
    }
}

fn render_optional_arg(field: &FullTypeFields) -> Option<rust::Tokens> {
    todo!()
}
