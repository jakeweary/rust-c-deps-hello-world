use std::env;
use std::path::PathBuf;

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
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
