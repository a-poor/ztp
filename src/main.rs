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
use swc_ecma_ast::{Pat, Param, TsTypeAnn, TsKeywordType, TsKeywordTypeKind, TsType};
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
    println!("end: {code}");
    std::fs::write(SAMPLE_CODE_DST, code).expect("failed to write file");
}

fn my_visitor() -> impl Fold {
    as_folder(MyVisitor)
}

struct MyVisitor;
impl VisitMut for MyVisitor {
    fn visit_mut_param(&mut self, param: &mut Param) {
        param.visit_mut_children_with(self);

        let mut new_param = param.clone();
        match new_param.pat {
            Pat::Ident(ref mut ident) => {
                ident.type_ann = Some(create_any_type());
            }
            _ => return
        };

        *param = new_param
    }
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
