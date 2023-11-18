
use parcel_transformer_js::{Config, transform};
use std::str;

use crate::core::Asset;
use crate::resolver::resolve;
use crate::transformation::{Transformer, TransformerContext, TransformerResult};

pub struct DefaultJSTransformer {
    is_jsx: bool,
    is_type_script: bool,
}

impl DefaultJSTransformer {
    pub fn new(
        is_jsx: bool,
        is_type_script: bool,
    ) -> Self {
        return DefaultJSTransformer {
            is_jsx,
            is_type_script,
        };
    }
}

impl Transformer for DefaultJSTransformer {
    fn get_name(&self) -> String {
        let mut name = String::from("DefaultJSTransformer");
        if self.is_type_script {
            name.push_str(" typescript");
        } else {
            name.push_str(" javascript");
        }
        return name;
    }

    fn transform(
        &self,
        ctx: &TransformerContext,
        asset: &mut Asset,
    ) -> TransformerResult {
        let log_level = ctx.get_log_level();

        let mut config = Config::new();
        config.code = asset.content.as_bytes().to_vec();
        config.filename = asset.file_path.to_str().unwrap().to_string();
        config.is_jsx = self.is_jsx;
        config.is_type_script = self.is_type_script;

        let transformation_res = transform(config);
        if transformation_res.is_err() {
            return TransformerResult::Break;
        }

        let transformation = transformation_res.unwrap();
        let code_res = str::from_utf8(transformation.code.as_slice());
        if code_res.is_err() {
            return TransformerResult::Break;
        }

        asset.content = code_res.unwrap().to_string();

        'outer: for descriptor in transformation.dependencies {
            let result = resolve(
                descriptor.specifier.as_str(),
                &asset.file_path,
            );

            if result.is_ok() {
                ctx.add_dependency(result.unwrap());
                continue;
            }
            
            let mut log = String::new();

            if log_level.is_verbose() {
                log.push_str(&format!("TRYING: {}\n", descriptor.specifier.as_str()))
            }

            for try_this in [
                ".js", ".jsx", ".ts", ".tsx",
                "/index.js", "/index.jsx", "/index.ts", "/index.tsx",
                "/src/index.js", "/src/index.jsx", "/src/index.ts", "/src/index.tsx",
            ] {
                let spec = format!("{}{}", descriptor.specifier.as_str(), try_this);
                if log_level.is_verbose() {
                    log.push_str(&format!("  {}\n", spec));
                }

                let result = resolve(
                    &spec,
                    &asset.file_path,
                );
                if result.is_ok() {
                    if log_level.is_verbose() {
                        log.push_str(&format!("FOUND: {}", spec));
                        println!("{}", log);
                    }
                    ctx.add_dependency(result.unwrap());
                    break 'outer;
                }
            }
            
            println!("Resolve Error, {}", descriptor.specifier);
            return TransformerResult::Err(String::from(""));
        }

        return TransformerResult::Continue;
    }
}
