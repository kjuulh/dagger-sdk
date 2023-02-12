use dagger_core::introspection::FullType;
use genco::prelude::rust;
use genco::quote;

use crate::rust::functions::format_name;

pub fn render_input(t: &FullType) -> eyre::Result<rust::Tokens> {
    Ok(quote! {
        pub struct $(format_name(t.name.as_ref().unwrap())) {

        }
    })
}
