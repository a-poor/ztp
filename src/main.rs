use std::{path::Path, sync::Arc};
use swc::{self, config::Options};
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
    GLOBALS,
};

fn main() {
    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));
    let compiler = swc::Compiler::new(cm.clone());

    let fm = cm
        .load_file(Path::new("sample.js"))
        .expect("failed to load file");

    let res = GLOBALS.set(&Default::default(), || {
        compiler.process_js_file(
            fm.clone(),
            &handler,
            &Options {
                ..Default::default()
            },
        )
    })
    .expect("failed to process file");

    print!("{}", res.code);
}
