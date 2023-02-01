use dagger_core::introspection::TypeRef;
use genco::prelude::rust;
use genco::prelude::*;

use crate::predicates::{
    is_custom_scalar_type_ref, is_list_type, is_required_type_ref, is_scalar_type_ref,
};

pub fn render_type_ref(inner: &TypeRef) -> eyre::Result<rust::Tokens> {
    let extract_of_type = |t: &TypeRef| -> Option<TypeRef> {
        return t.clone().of_type.map(|t| *t);
    };

    if is_list_type(&inner) {
        if let Some(inner_of_type) = extract_of_type(inner) {
            let inner_field = render_type_ref(&inner_of_type)?;
            return Ok(quote! {
                Vec<$inner_field>
            });
        }
    }

    if is_custom_scalar_type_ref(&inner) {
        if let Some(inner_of_type) = extract_of_type(inner) {
            let inner_field = render_type_ref(&inner_of_type)?;
            return Ok(quote! {
                $inner_field
            });
        }
    }

    if is_scalar_type_ref(&inner) {
        let name = match inner.name.as_ref().map(|s| s.as_str()) {
            Some("ID") => "ID",
            Some("Int") => "Int",
            Some("String") => "String",
            Some("Float") => "Float",
            Some("Boolean") => "Boolean",
            Some("Date") => "Date",
            Some("DateTime") => "DateTime",
            Some("Time") => "Time",
            Some("Decimal") => "Decimal",
            Some(n) => n,
            _ => eyre::bail!("missing type in the end of chain"),
        };

        return Ok(quote! {
            $name
        });
    }

    if !is_required_type_ref(inner) {
        if let Some(inner_of_type) = extract_of_type(inner) {
            let inner_field = render_type_ref(&inner_of_type)?;
            return Ok(quote! {
                Option<$inner_field>
            });
        }
    }

    if let Some(inner_type) = inner.of_type.as_ref() {
        return render_type_ref(&inner_type);
    }

    if let Some(name) = inner.name.as_ref() {
        return Ok(quote! {
            $name
        });
    }

    eyre::bail!("could not determine type")
}