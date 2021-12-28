fn main() {
    // we skip assembling the runtime for docs.rs builds.
    if !cfg!(docs_rs) {
      let libseven_dir = "vendor/libseven";
      let libseven_build_dir = format!("{}/build", libseven_dir);

      let make_output = std::process::Command::new("make")
        .arg("-C")
        .arg(libseven_dir)
        .output()
        .expect("libseven: failed to run make");

      if !make_output.status.success() {
        panic!("{}", String::from_utf8_lossy(&make_output.stderr));
      }
      //
      println!("cargo:rustc-link-search={}/{}", env!("CARGO_MANIFEST_DIR"), libseven_build_dir);
      println!("cargo:rustc-link-lib=static=seven");
    }
  }
