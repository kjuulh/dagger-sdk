pub mod format;
mod functions;
pub mod templates;

use std::sync::{Arc, Mutex};

use dagger_core::introspection::Schema;
use eyre::Context;
use genco::prelude::rust;

use crate::generator::Generator;
use crate::visitor::{VisitHandlers, Visitor};

use self::templates::enum_tmpl::render_enum;
use self::templates::input_tmpl::render_input;
use self::templates::object_tmpl::render_object;
use self::templates::scalar_tmpl::render_scalar;

pub struct RustGenerator {}

impl Generator for RustGenerator {
    fn generate(&self, schema: Schema) -> eyre::Result<String> {
        let render = Arc::new(Mutex::new(rust::Tokens::new()));

        let visitor = Visitor {
            schema,
            handlers: VisitHandlers {
                visit_scalar: Arc::new({
                    let render = render.clone();
                    move |t| {
                        let rendered_scalar = render_scalar()?;

                        let mut render = render.lock().unwrap();

                        render.append(rendered_scalar);
                        render.push();

                        Ok(())
                    }
                }),
                visit_object: Arc::new({
                    let render = render.clone();

                    move |t| {
                        let rendered_scalar = render_object()?;

                        let mut render = render.lock().unwrap();

                        render.append(rendered_scalar);
                        render.push();

                        Ok(())
                    }
                }),
                visit_input: Arc::new({
                    let render = render.clone();

                    move |t| {
                        let rendered_scalar = render_input()?;

                        let mut render = render.lock().unwrap();

                        render.append(rendered_scalar);
                        render.push();

                        Ok(())
                    }
                }),
                visit_enum: Arc::new({
                    let render = render.clone();

                    move |t| {
                        let rendered_scalar = render_enum()?;

                        let mut render = render.lock().unwrap();

                        render.append(rendered_scalar);
                        render.push();

                        Ok(())
                    }
                }),
            },
        };

        visitor.run()?;

        let rendered = render.lock().unwrap();

        rendered
            .to_file_string()
            .context("could not render to file string")
    }
}
