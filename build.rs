use glob::glob;
use grass::Options;
use lightningcss::printer::PrinterOptions;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use std::default::Default;
use std::fs;
use std::path::PathBuf;

const STYLES_PATH_PREFIX: &str = "styles/";
const ARTIFACTS_PATH: &str = "assets/artifacts";
const OUTPUT_PATH: &str = "assets/artifacts/bundle.css";

fn main() {
    transpile_scss();
}

fn transpile_scss() {
    let pattern = format!("./{STYLES_PATH_PREFIX}**/*.scss");
    let files = glob(&pattern).expect("failed to read styles");
    let paths = files
        .into_iter()
        .map(|result| match result {
            Ok(f) => f,
            Err(e) => {
                eprintln!("failed to read style: {e}");
                panic!("exiting due to error(s) above");
            }
        })
        .collect::<Vec<_>>();
    let import_scss = generate_import_scss(&paths);
    println!("{import_scss}");
    let options = Options::default();
    let css = match grass::from_string(import_scss, &options) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("SCSS error: {e}");
            panic!("css transpilation failed");
        }
    };
    fs::create_dir_all(ARTIFACTS_PATH).unwrap();
    let stylesheet =
        StyleSheet::parse(&css, ParserOptions::default()).expect("failed to parse output css file");
    let printer_options = PrinterOptions {
        minify: true,
        ..Default::default()
    };
    let output = stylesheet
        .to_css(printer_options)
        .expect("failed to serialize minified css")
        .code;
    println!("output: {output}");
    fs::write(OUTPUT_PATH, output).expect("failed to write output css file");
}

/// Generates the SCSS entry point which imports all other stylesheets.
fn generate_import_scss(paths: &[PathBuf]) -> String {
    let mut contents = String::new();
    for path in paths {
        // let string = path.display().to_string();
        // let (_, relative_path) = string.split_at(STYLES_PATH_PREFIX.len());
        let line = format!("@import \"{}\";\r\n", path.display());
        contents.push_str(&line);
    }
    contents
}
