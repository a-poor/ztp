use std::{path::Path, sync::Arc};
use swc::config::Options;
use swc_common::{
    errors::{ColorConfig, Handler},
    SourceMap,
    GLOBALS,
};

const SAMPLE_CODE_SRC: &str = "sample.in.js";
const SAMPLE_CODE_DST: &str = "sample.out.js";

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
        .load_file(Path::new(SAMPLE_CODE_SRC))
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

    std::fs::write(SAMPLE_CODE_DST, res.code).expect("failed to write file");
}
