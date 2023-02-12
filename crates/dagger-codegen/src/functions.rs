use std::sync::Arc;

use dagger_core::introspection::{TypeRef, __TypeKind};

pub trait FormatTypeFuncs {
    fn format_kind_list(&self, representation: &str) -> String;
    fn format_kind_scalar_string(&self, representation: &str) -> String;
    fn format_kind_scalar_int(&self, representation: &str) -> String;
    fn format_kind_scalar_float(&self, representation: &str) -> String;
    fn format_kind_scalar_boolean(&self, representation: &str) -> String;
    fn format_kind_scalar_default(
        &self,
        representation: &str,
        ref_name: &str,
        input: bool,
    ) -> String;
    fn format_kind_object(&self, representation: &str, ref_name: &str) -> String;
    fn format_kind_input_object(&self, representation: &str, ref_name: &str) -> String;
    fn format_kind_enum(&self, representation: &str, ref_name: &str) -> String;
}

pub type DynFormatTypeFuncs = Arc<dyn FormatTypeFuncs + Send + Sync>;

pub struct CommonFunctions {
    format_type_funcs: DynFormatTypeFuncs,
}

impl CommonFunctions {
    pub fn new(funcs: DynFormatTypeFuncs) -> Self {
        Self {
            format_type_funcs: funcs,
        }
    }

    pub fn format_input_type(&self, t: &TypeRef) -> String {
        self.format_type(t, true)
    }

    pub fn format_output_type(&self, t: &TypeRef) -> String {
        self.format_type(t, false)
    }

    fn format_type(&self, t: &TypeRef, input: bool) -> String {
        let mut representation = String::new();
        let mut r = Some(t.clone());
        while r.is_some() {
            match r.as_ref() {
                Some(rf) => match rf.kind.as_ref() {
                    Some(k) => match k {
                        __TypeKind::SCALAR => match Scalar::from(rf) {
                            Scalar::Int => {
                                self.format_type_funcs
                                    .format_kind_scalar_int(&mut representation);
                            }
                            Scalar::Float => {
                                self.format_type_funcs
                                    .format_kind_scalar_float(&mut representation);
                            }
                            Scalar::String => {
                                self.format_type_funcs
                                    .format_kind_scalar_string(&mut representation);
                            }
                            Scalar::Boolean => {
                                self.format_type_funcs
                                    .format_kind_scalar_boolean(&mut representation);
                            }
                            Scalar::Default => {
                                self.format_type_funcs.format_kind_scalar_default(
                                    &mut representation,
                                    rf.name.as_ref().unwrap(),
                                    input,
                                );
                            }
                        },
                        __TypeKind::OBJECT => {
                            self.format_type_funcs
                                .format_kind_object(&mut representation, rf.name.as_ref().unwrap());
                        }
                        __TypeKind::ENUM => {
                            self.format_type_funcs
                                .format_kind_enum(&mut representation, rf.name.as_ref().unwrap());
                        }
                        __TypeKind::INPUT_OBJECT => {
                            self.format_type_funcs.format_kind_input_object(
                                &mut representation,
                                &rf.name.as_ref().unwrap(),
                            );
                        }
                        __TypeKind::LIST => {
                            self.format_type_funcs.format_kind_list(&mut representation);
                        }
                        __TypeKind::Other(_) => {
                            r = rf.of_type.as_ref().map(|t| t.clone()).map(|t| *t)
                        }
                        _ => {}
                    },
                    None => break,
                },
                None => break,
            }
        }

        representation
    }
}

pub enum Scalar {
    Int,
    Float,
    String,
    Boolean,
    Default,
}

impl From<&TypeRef> for Scalar {
    fn from(value: &TypeRef) -> Self {
        match value.name.as_ref().map(|n| n.as_str()) {
            Some("Int") => Scalar::Int,
            Some("Float") => Scalar::Float,
            Some("String") => Scalar::String,
            Some("Boolean") => Scalar::Boolean,
            Some(_) => Scalar::Default,
            None => Scalar::Default,
        }
    }
}
