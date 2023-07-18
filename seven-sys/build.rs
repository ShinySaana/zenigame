use std::{path::PathBuf, env, fs::canonicalize};

fn main() {
  // we skip assembling the runtime for docs.rs builds.
  if !cfg!(docs_rs) {
    let root_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/.."));
    let sdk_seven_dir = root_dir.join(PathBuf::from("vendor/sdk-seven"));
    let build_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_path = build_path.join("build");
    let libseven_build_dir = build_path.to_str().unwrap();


    let setup_output = std::process::Command::new("meson")
      .args([
        "setup",
        "--cross-file=cross/arm-none-eabi.txt",
        "--cross-file=cross/arm7tdmi.txt",
        "-Dminrt_lang=rust",
        "-Dbuildtype=debugoptimized",
        libseven_build_dir
      ])
      .current_dir(sdk_seven_dir.to_str().unwrap())
      .output()
      .expect("libseven: failed to setup");

    if !setup_output.status.success() {
      panic!("{}\n{}", String::from_utf8_lossy(&setup_output.stdout), String::from_utf8_lossy(&setup_output.stderr));
    }

    let build_output = std::process::Command::new("ninja")
      .arg("-C")
      .arg("build")
      .current_dir(PathBuf::from(env::var("OUT_DIR").unwrap()))
      .output()
      .expect("libseven: failed to build");

    if !build_output.status.success() {
      panic!("{}", String::from_utf8_lossy(&build_output.stderr));
    }

    println!("cargo:rustc-link-search={}/{}", libseven_build_dir, "libseven");
    println!("cargo:rustc-link-search={}/{}", libseven_build_dir, "libutil");
    println!("cargo:rustc-link-lib=static=seven");
    println!("cargo:rustc-link-lib=static=util");

    bindgen(&sdk_seven_dir);
  }
}

fn bindgen(sdk_seven_dir: &PathBuf) {
  let libseven_include_dir = sdk_seven_dir.join("libseven").join("include");
  let libutil_include_dir = sdk_seven_dir.join("libutil").join("include");

  let wrapper_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wrapper.h");

  let mut builder = bindgen::Builder::default()
    .header(wrapper_file_path.to_str().unwrap());

  let libseven_headers_dir_canonical = canonicalize(libseven_include_dir).unwrap();
  let libutil_headers_dir_canonical = canonicalize(libutil_include_dir).unwrap();

  let libseven_include_dir = libseven_headers_dir_canonical.to_str().unwrap();
  let libutil_include_dir = libutil_headers_dir_canonical.to_str().unwrap();
  builder = builder.clang_arg(format!("-I{libseven_include_dir}"));
  builder = builder.clang_arg(format!("-I{libutil_include_dir}"));

  println!("cargo:rerun-if-changed={}/seven/base.h", libseven_include_dir);
  println!("cargo:rerun-if-changed=wrapper.h");
  builder = builder.parse_callbacks(Box::new(bindgen::CargoCallbacks));

  let builder = builder
    .clang_args(["-D", "__BINDGEN__"])
    .clang_arg("--target=thumbv4t-none-eabi");

  let bindings = builder
    .use_core()
    .rustified_enum(".*")
    .generate()
    .expect("Unable to generate libseven bindings");

  let src_dir = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src"));

  bindings
  .write_to_file(src_dir.join("bindgen_bindings.rs"))
  .expect("Couldn't write libseven bindings!");
}
