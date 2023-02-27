use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let html = fs::read_to_string("index.html").unwrap();
    let js = fs::read_to_string("main.js").unwrap();
    let css = fs::read_to_string("style.css").unwrap();

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("constants.rs");
    fs::write(
        dest_path,
        format!("pub fn html() -> &'static str {{
            \"{html}\"
        }}

        pub fn js() -> &'static str {{
            \"{js}\"
        }}

        pub fn css() -> &'static str {{
            \"{css}\"
        }}
        ")
    ).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=index.html");
    println!("cargo:rerun-if-changed=main.js");
    println!("cargo:rerun-if-changed=style.css");
}