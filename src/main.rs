#![allow(dead_code)]
#![allow(unused_imports)]

use std::{path::Path, sync::Arc};
use swc::config::Options;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
    FileName, 
    Span,
    GLOBALS,
    DUMMY_SP,
};
use swc_common::sync::Lrc;
use swc_ecma_ast::{
    Pat, 
    Param, 
    TsTypeAnn, 
    TsKeywordType, 
    TsKeywordTypeKind, 
    TsType, 
    FnDecl, 
    Function, 
    BlockStmt, 
    Expr,
    ExprStmt, 
    Stmt, 
    Lit, 
    CallExpr, 
    Ident,
    ExprOrSpread,
    ObjectLit,
    PropOrSpread,
    Prop,
    KeyValueProp,
    PropName,
    Str,
    ArrayLit,
};
use swc_atoms::{js_word, Atom, JsWord};
use swc_ecma_visit::{as_folder, VisitMut, Fold, VisitMutWith};
use swc_ecma_transforms::pass::noop;

const SAMPLE_CODE_SRC: &str = "sample.in.js";
const SAMPLE_CODE_DST: &str = "sample.out.js";


fn main() {
    let cm: Lrc<SourceMap> = Default::default();
    let file = cm
        .load_file(Path::new(SAMPLE_CODE_SRC))
        .expect("failed to load file");
    let compiler = swc::Compiler::new(cm.clone());
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    let result = GLOBALS.set(&Default::default(), || {
        compiler.process_js_with_custom_pass(
            file,
            None,
            &handler,
            &Options::default(),
            Default::default(),
            |_| noop(),
            |_| my_visitor(),
        )
    });

    let code = result.unwrap().code;
    println!("\nRESULT => \n{code}");
    std::fs::write(SAMPLE_CODE_DST, code).expect("failed to write file");
}

fn my_visitor() -> impl Fold {
    as_folder(ZtpVisitor)
}

struct ZtpVisitor;
impl VisitMut for ZtpVisitor {
    fn visit_mut_fn_decl(&mut self, node: &mut FnDecl) {
        node.visit_mut_children_with(self);

        let node_copy = node.clone();
        
        let body = match node_copy.function.body {
            Some(body) => body,
            None => return
        };
        let stmt = match body.stmts.get(0) {
            Some(stmt) => stmt.clone(),
            None => return
        };
        let params = node_copy.function.params.clone();
        match stmt {
            Stmt::Expr(expr_stmt) => {
                match *expr_stmt.expr {
                    Expr::Lit(lit) => {
                        match lit {
                            Lit::Str(str_lit) => {

                                // Check if the first line of the function is the string "use ztp"...
                                if str_lit.raw == Some(Atom::from("'use ztp'")) || str_lit.raw == Some(Atom::from("\"use ztp\"")) {
                                    // TODO - Move the body out of the function so it can be executed in a separate context...
                                    //
                                    // NOTES: 
                                    // - Will need to track relevant exports and variables that need to be passed in
                                    // - The exported function can't modify the variables (at least for now)

                                    // Replace the body with a remote call...
                                    node.function.body = Some(make_ztp_func_body(&node_copy.ident.sym.to_string(), params));
                                }
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        };
    }
}

/// Creates a block statement that redirects the function call.
/// 
/// This allows the defined function to be called as normal, but for
/// the result to be executed in a sandboxed environment.
/// 
/// For example, if the function is defined as:
/// 
/// ```js
/// function foo(a, b) {
///   // ...
/// }
/// ```
/// 
/// Then the resulting function, with the generated block statement will be:
/// 
/// ```js
/// function foo(a, b) {
///   __TO_ZTP_SANDBOX({
///     name: 'foo',
///     params: [a, b],
///   });
/// }
/// ```
fn make_ztp_func_body(name: &str, params: Vec<Param>) -> BlockStmt {
    BlockStmt { 
        span: DUMMY_SP, 
        stmts: vec![
            Stmt::Expr(ExprStmt { 
                span: DUMMY_SP,
                expr: Box::new(Expr::Call(CallExpr{
                    span: DUMMY_SP,
                    callee: swc_ecma_ast::Callee::Expr(Box::new(Expr::Ident(Ident {
                        span: DUMMY_SP,
                        sym: "__TO_ZTP_SANDBOX".into(),
                        optional: false,
                    }))),
                    args: vec![
                        ExprOrSpread {
                            spread: None,
                            expr: Box::new(Expr::Object(ObjectLit {
                                span: DUMMY_SP,
                                props: vec![
                                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                        key: PropName::Ident(Ident {
                                            span: DUMMY_SP,
                                            sym: "name".into(),
                                            optional: false,
                                        }),
                                        value: Box::new(Expr::Lit(Lit::Str(Str {
                                            span: DUMMY_SP,
                                            value: name.into(),
                                            raw: Some(name.into()),
                                        }))),
                                    }))),
                                    PropOrSpread::Prop(Box::new(Prop::KeyValue(KeyValueProp {
                                        key: PropName::Ident(Ident {
                                            span: DUMMY_SP,
                                            sym: "params".into(),
                                            optional: false,
                                        }),
                                        value: Box::new(Expr::Array(ArrayLit {
                                            span: DUMMY_SP,
                                            elems: params.into_iter().map(|param| {
                                                let ident = match param.pat {
                                                    Pat::Ident(ident) => ident.id,
                                                    _ => panic!("Expected ident pattern"),
                                                };
                                                Some(ExprOrSpread {
                                                    spread: None,
                                                    expr: Box::new(Expr::Ident(ident)),
                                                })
                                            }).collect(),
                                        })),
                                    }))),
                                ],
                            })),
                        }
                    ],
                    type_args: None,
                })),
            })
        ],
    }
}
