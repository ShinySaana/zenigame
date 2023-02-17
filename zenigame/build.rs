use std::{path::PathBuf, env};

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
          libseven_build_dir
        ])
        .current_dir(sdk_seven_dir.to_str().unwrap())
        .output()
        .expect("libseven: failed to setup");

      if !setup_output.status.success() {
        panic!("{}", String::from_utf8_lossy(&setup_output.stderr));
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

      println!("cargo:rustc-link-search={}/minrt", libseven_build_dir);
      println!("cargo:rustc-link-lib=static=minrt_rom");
    }
  }
