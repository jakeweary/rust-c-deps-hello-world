use std::env;
use std::path::PathBuf;

use bindgen::callbacks::{IntKind, ParseCallbacks};

#[derive(Debug)]
struct Callbacks;

impl ParseCallbacks for Callbacks {
  fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
    if name.starts_with("GLFW_") {
      return Some(IntKind::Int);
    }
    None
  }

  // from bindgen::CargoCallbacks
  fn include_file(&self, filename: &str) {
    println!("cargo:rerun-if-changed={}", filename);
  }
}

fn main() {
  let target = env::var("TARGET").unwrap();
  let out_dir = env::var("OUT_DIR").unwrap();
  let out_path = PathBuf::from(&out_dir);

  cc::Build::new()
    .target(&target)
    .include("deps/include")
    .file("deps/glad.c")
    .compile("glad");

  bindgen::Builder::default()
    .parse_callbacks(Box::new(Callbacks))
    .clang_arg("-std=c99")
    .clang_arg("-Ideps/include")
    .header("deps/deps.h")
    .generate().unwrap()
    .write_to_file(out_path.join("bindings.rs")).unwrap();

  println!("cargo:rustc-link-search=native={}", out_dir);
  println!("cargo:rustc-link-search=native=deps/lib");
  println!("cargo:rustc-link-lib=glfw3");
  println!("cargo:rustc-link-lib=glad");
  println!("cargo:rustc-link-lib=winmm");
  println!("cargo:rustc-link-lib=gdi32");
  println!("cargo:rustc-link-lib=opengl32");
  println!("cargo:rustc-link-lib=shell32");
  println!("cargo:rerun-if-changed=deps/deps.h");
}
