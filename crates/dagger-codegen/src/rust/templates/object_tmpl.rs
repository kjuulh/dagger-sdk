use dagger_core::introspection::{FullType, FullTypeFields};
use genco::prelude::rust;
use genco::quote;

use crate::functions::CommonFunctions;
use crate::rust::functions::{field_options_struct_name, format_name};
use crate::utility::OptionExt;

pub fn render_object(funcs: &CommonFunctions, t: &FullType) -> eyre::Result<rust::Tokens> {
    Ok(quote! {
        pub struct $(t.name.pipe(|s| format_name(s))) {

        }

        $(t.fields.pipe(|f| render_optional_args(funcs, f)))

        impl $(t.name.pipe(|s| format_name(s))) {

        }
    })
}

fn render_optional_args(
    funcs: &CommonFunctions,
    fields: &Vec<FullTypeFields>,
) -> Option<rust::Tokens> {
    let rendered_fields = fields
        .iter()
        .map(|f| render_optional_arg(funcs, f))
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

fn render_optional_arg(funcs: &CommonFunctions, field: &FullTypeFields) -> Option<rust::Tokens> {
    field
        .type_
        .pipe(|t| funcs.format_output_type(&t.type_ref))
        .pipe(|f| {
            quote! {
                $f
            }
        })
}
