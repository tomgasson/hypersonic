use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::path::Path;

use swc_core::common::{Mark, SourceMap, SyntaxContext, DUMMY_SP};
use swc_core::ecma::ast::{self, Id};
use swc_core::ecma::atoms::JsWord;
use swc_core::ecma::visit::{Fold, FoldWith};

use crate::dependency_collector::{DependencyDescriptor, DependencyKind};
use crate::utils::{create_global_decl_stmt, create_require, SourceLocation, SourceType};

pub struct NodeReplacer<'a> {
  pub source_map: &'a SourceMap,
  pub items: &'a mut Vec<DependencyDescriptor>,
  pub global_mark: Mark,
  pub globals: HashMap<JsWord, (SyntaxContext, ast::Stmt)>,
  pub project_root: &'a Path,
  pub filename: &'a Path,
  pub decls: &'a mut HashSet<Id>,
  pub scope_hoist: bool,
  pub has_node_replacements: &'a mut bool,
}

impl<'a> Fold for NodeReplacer<'a> {
  fn fold_expr(&mut self, node: ast::Expr) -> ast::Expr {
    use ast::{Expr::*, MemberExpr, MemberProp};

    // Do not traverse into the `prop` side of member expressions unless computed.
    let mut node = match node {
      Member(expr) => {
        if let MemberProp::Computed(_) = expr.prop {
          Member(MemberExpr {
            obj: expr.obj.fold_with(self),
            prop: expr.prop.fold_with(self),
            ..expr
          })
        } else {
          Member(MemberExpr {
            obj: expr.obj.fold_with(self),
            ..expr
          })
        }
      }
      _ => node.fold_children_with(self),
    };

    if let Ident(id) = &mut node {
      // Only handle global variables
      if self.decls.contains(&(id.sym.clone(), id.span.ctxt())) {
        return node;
      }

      match id.sym.to_string().as_str() {
        "__filename" => {
          let specifier = swc_core::ecma::atoms::JsWord::from("path");
          let replace_me_value = swc_core::ecma::atoms::JsWord::from("$parcel$filenameReplace");

          let expr = |this: &NodeReplacer| {
            let filename = if let Some(name) = this.filename.file_name() {
              name
            } else {
              OsStr::new("unknown.js")
            };
            ast::Expr::Call(ast::CallExpr {
              span: DUMMY_SP,
              type_args: None,
              args: vec![
                ast::ExprOrSpread {
                  spread: None,
                  expr: Box::new(ast::Expr::Ident(ast::Ident {
                    optional: false,
                    span: DUMMY_SP,
                    // This also uses __dirname as later in the path.join call the hierarchy is then correct
                    // Otherwise path.join(__filename, '..') would be one level to shallow (due to the /filename.js at the end)
                    sym: swc_core::ecma::atoms::JsWord::from("__dirname"),
                  })),
                },
                ast::ExprOrSpread {
                  spread: None,
                  expr: Box::new(ast::Expr::Lit(ast::Lit::Str(ast::Str {
                    span: DUMMY_SP,
                    value: replace_me_value,
                    raw: None,
                  }))),
                },
                ast::ExprOrSpread {
                  spread: None,
                  expr: Box::new(ast::Expr::Lit(ast::Lit::Str(ast::Str {
                    span: DUMMY_SP,
                    value: swc_core::ecma::atoms::JsWord::from(filename.to_string_lossy()),
                    raw: None,
                  }))),
                },
              ],
              callee: ast::Callee::Expr(Box::new(ast::Expr::Member(ast::MemberExpr {
                span: DUMMY_SP,
                obj: (Box::new(Call(create_require(specifier.clone())))),
                prop: MemberProp::Ident(ast::Ident::new("resolve".into(), DUMMY_SP)),
              }))),
            })
          };
          if self.update_binding(id, "$parcel$__filename".into(), expr) {
            self.items.push(DependencyDescriptor {
              kind: DependencyKind::Require,
              loc: SourceLocation::from(self.source_map, id.span),
              specifier,
              attributes: None,
              is_optional: false,
              is_helper: false,
              source_type: Some(SourceType::Module),
              placeholder: None,
            });

            *self.has_node_replacements = true;
          }
        }
        "__dirname" => {
          let specifier = swc_core::ecma::atoms::JsWord::from("path");
          let replace_me_value = swc_core::ecma::atoms::JsWord::from("$parcel$dirnameReplace");

          if self.update_binding(id, "$parcel$__dirname".into(), |_| {
            ast::Expr::Call(ast::CallExpr {
              span: DUMMY_SP,
              type_args: None,
              args: vec![
                ast::ExprOrSpread {
                  spread: None,
                  expr: Box::new(ast::Expr::Ident(ast::Ident {
                    optional: false,
                    span: DUMMY_SP,
                    sym: swc_core::ecma::atoms::JsWord::from("__dirname"),
                  })),
                },
                ast::ExprOrSpread {
                  spread: None,
                  expr: Box::new(ast::Expr::Lit(ast::Lit::Str(ast::Str {
                    span: DUMMY_SP,
                    value: replace_me_value,
                    raw: None,
                  }))),
                },
              ],
              callee: ast::Callee::Expr(Box::new(ast::Expr::Member(ast::MemberExpr {
                span: DUMMY_SP,
                obj: (Box::new(Call(create_require(specifier.clone())))),
                prop: MemberProp::Ident(ast::Ident::new("resolve".into(), DUMMY_SP)),
              }))),
            })
          }) {
            self.items.push(DependencyDescriptor {
              kind: DependencyKind::Require,
              loc: SourceLocation::from(self.source_map, id.span),
              specifier,
              attributes: None,
              is_optional: false,
              is_helper: false,
              source_type: Some(SourceType::Module),
              placeholder: None,
            });

            *self.has_node_replacements = true;
          }
        }
        _ => {}
      }
    }

    node
  }

  fn fold_module(&mut self, node: ast::Module) -> ast::Module {
    // Insert globals at the top of the program
    let mut node = swc_core::ecma::visit::fold_module(self, node);
    node.body.splice(
      0..0,
      self
        .globals
        .values()
        .map(|(_, stmt)| ast::ModuleItem::Stmt(stmt.clone())),
    );
    node
  }
}

impl NodeReplacer<'_> {
  fn update_binding<F>(&mut self, id_ref: &mut ast::Ident, new_name: JsWord, expr: F) -> bool
  where
    F: FnOnce(&Self) -> ast::Expr,
  {
    if let Some((ctxt, _)) = self.globals.get(&new_name) {
      id_ref.sym = new_name;
      id_ref.span.ctxt = *ctxt;
      false
    } else {
      id_ref.sym = new_name;

      let (decl, ctxt) = create_global_decl_stmt(id_ref.sym.clone(), expr(self), self.global_mark);
      id_ref.span.ctxt = ctxt;

      self.globals.insert(id_ref.sym.clone(), (ctxt, decl));
      self.decls.insert(id_ref.to_id());

      true
    }
  }
}
