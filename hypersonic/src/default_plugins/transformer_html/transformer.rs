extern crate html5ever;
extern crate markup5ever_rcdom as rcdom;

use std::path::Path;

use crate::core::Asset;
use crate::transformation::{Transformer, TransformerContext, TransformerResult};

use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use rcdom::{Handle, NodeData, RcDom};

pub struct DefaultHTMLTransformer {}

impl DefaultHTMLTransformer {
    pub fn new() -> Self {
        return DefaultHTMLTransformer {};
    }
}

impl Transformer for DefaultHTMLTransformer {
    fn get_name(&self) -> String {
        return String::from("DefaultHTMLTransformer");
    }

    fn transform(
        &self,
        ctx: &TransformerContext,
        asset: &mut Asset,
    ) -> TransformerResult {
        let result = get_script_src_attrs(&asset.content);
        if result.is_err() {
            return TransformerResult::Break;
        }

        for specifier in result.unwrap() {
          let asset_dir_path = asset.file_path.parent().unwrap();
          let parsed_specifier = Path::new(&specifier);
          let full_path = asset_dir_path.join(parsed_specifier);
          ctx.add_dependency(full_path);
        }

        return TransformerResult::Continue;
    }
}

fn get_script_src_attrs(html: &str) -> Result<Vec<String>, ()> {
    let mut script_src_attrs = Vec::<String>::new();

    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();

    walk(&dom.document, &mut script_src_attrs);

    return Ok(script_src_attrs);
}

fn walk(handle: &Handle, attrs_list: &mut Vec<String>) {
    let node = handle;
    match node.data {
        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            if name.local.to_string() == "script" {
                for attr in attrs.borrow().iter() {
                    if attr.name.local.to_string() == "src" {
                        attrs_list.push(attr.value.to_string());
                        break;
                    }
                }
            }
        }
        _ => {}
    }

    for child in node.children.borrow().iter() {
        walk(child, attrs_list);
    }
}
