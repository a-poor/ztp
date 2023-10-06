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
};
use swc_common::sync::Lrc;
use swc_ecma_ast::{Pat, Param, TsTypeAnn, TsKeywordType, TsKeywordTypeKind, TsType, FnDecl, Function, BlockStmt, Expr, ExprStmt, Stmt, Lit};
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
    as_folder(MyVisitor)
}

struct MyVisitor;
impl VisitMut for MyVisitor {
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
        // println!("STMT => {:?}\n", stmt);
        match stmt {
            Stmt::Expr(expr_stmt) => {
                match *expr_stmt.expr {
                    Expr::Lit(lit) => {
                        match lit {
                            Lit::Str(str_lit) => {
                                if str_lit.raw == Some(Atom::from("'use ztp'")) || str_lit.raw == Some(Atom::from("\"use ztp\"")) {
                                    println!("Use ZTP extraction for function: {}", node_copy.ident.sym.to_string());
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



        // let mut new_node = node.clone();
        // new_node.params = new_node.params
        //     .into_iter()
        //     .map(|param| {
        //         let mut new_param = param.clone();
        //         match new_param.pat {
        //             Pat::Ident(ref mut ident) => {
        //                 ident.type_ann = Some(create_any_type());
        //             }
        //             _ => return param
        //         };

        //         new_param
        //     })
        //     .collect();

        // *node = new_node
    }

    // fn visit_mut_param(&mut self, param: &mut Param) {
    //     param.visit_mut_children_with(self);

    //     let mut new_param = param.clone();
    //     match new_param.pat {
    //         Pat::Ident(ref mut ident) => {
    //             ident.type_ann = Some(create_any_type());
    //         }
    //         _ => return
    //     };

    //     *param = new_param
    // }
}

fn create_any_type() -> Box<TsTypeAnn> {
    let any_keyword = TsKeywordType {
        span: Span::default(),
        kind: TsKeywordTypeKind::TsAnyKeyword,
    };

    Box::new(TsTypeAnn {
        span: Span::default(),
        type_ann: Box::new(TsType::TsKeywordType(any_keyword)),
    })
}

// fn main() {
//     let cm = Arc::<SourceMap>::default();
//     let handler = Arc::new(Handler::with_tty_emitter(
//         ColorConfig::Auto,
//         true,
//         false,
//         Some(cm.clone()),
//     ));
//     let compiler = swc::Compiler::new(cm.clone());

//     let fm = cm
//         .load_file(Path::new(SAMPLE_CODE_SRC))
//         .expect("failed to load file");

//     let res = GLOBALS.set(&Default::default(), || {
//         compiler.process_js_with_custom_pass(
//             fm.clone(),
//             None,
//             &handler,
//             &Options {
//                 ..Default::default()
//             },
//             |_,| noop(),
//             |_,_| my_visitor(),
//         )
//     })
//     .expect("failed to process file");

//     std::fs::write(SAMPLE_CODE_DST, res.code).expect("failed to write file");
// }
