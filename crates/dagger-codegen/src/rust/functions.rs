use convert_case::{Case, Casing};
use dagger_core::introspection::FullTypeFields;
use genco::prelude::rust;
use genco::quote;

use crate::functions::{
    input_values_has_optionals, type_field_has_optional, type_ref_is_optional, CommonFunctions,
};
use crate::utility::OptionExt;

pub fn format_name(s: &str) -> String {
    s.to_case(Case::Pascal)
}

pub fn format_struct_name(s: &str) -> String {
    s.to_case(Case::Snake)
}

pub fn field_options_struct_name(field: &FullTypeFields) -> Option<String> {
    field
        .parent_type
        .as_ref()
        .map(|p| p.name.as_ref().map(|n| format_name(n)))
        .flatten()
        .zip(field.name.as_ref().map(|n| format_name(n)))
        .map(|(parent_name, field_name)| format!("{parent_name}{field_name}Opts"))
}

pub fn format_function(funcs: &CommonFunctions, field: &FullTypeFields) -> Option<rust::Tokens> {
    let signature = quote! {
        pub fn $(field.name.pipe(|n | format_struct_name(n)))
    };
    let args = format_function_args(funcs, field);

    let output_type = field
        .type_
        .pipe(|t| &t.type_ref)
        .pipe(|t| funcs.format_output_type(t));

    Some(quote! {
        $(signature)(
            $(args)
        ) -> $(output_type) {
            todo!()
        }
    })
}

fn format_function_args(funcs: &CommonFunctions, field: &FullTypeFields) -> Option<rust::Tokens> {
    if let Some(args) = field.args.as_ref() {
        let args = args
            .into_iter()
            .map(|a| {
                a.as_ref().and_then(|s| {
                    if type_ref_is_optional(&s.input_value.type_) {
                        return None;
                    }

                    let t = funcs.format_input_type(&s.input_value.type_);
                    let n = format_struct_name(&s.input_value.name);

                    Some(quote! {
                        $(n): $(t),
                    })
                })
            })
            .flatten()
            .collect::<Vec<_>>();
        let required_args = quote! {
            &self,
            $(for arg in args join ($['\r']) => $arg)
        };

        if type_field_has_optional(field) {
            Some(quote! {
                $(required_args)
                opts: Option<$(field_options_struct_name(field))>
            })
        } else {
            Some(required_args)
        }
    } else {
        None
    }
}
