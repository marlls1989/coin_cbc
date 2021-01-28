use num_cpus;
use std::{env, path::Path, process::Command};
use system_deps;

fn main() {
    if cfg!(feature = "embedded_cbc") {
        env::set_var("SYSTEM_DEPS_CBC_BUILD_INTERNAL", "always");
        env::set_var("CBC_STATIC", "1");
    }

    system_deps::Config::new()
        .add_build_internal("cbc", |lib, version| {
            let cores = num_cpus::get();
            let out_dir = env::var("OUT_DIR").unwrap();
            let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

            Command::new(&format!("{}/coinbrew", src_dir))
                .args(&["fetch", "Cbc@2.10.5", "--no-prompt"])
                .current_dir(&Path::new(&out_dir))
                .status()
                .unwrap();

            Command::new(&format!("{}/coinbrew", src_dir))
                .args(&[
                    "build",
                    "Cbc",
                    "--static",
                    "--no-prompt",
                    "--tests=none",
                    &format!("--parallel-jobs={}", cores),
                    "ADD_FFLAGS=-fallow-argument-mismatch -fPIC",
                    "ADD_CFLAGS=-fPIC",
                    "ADD_CXXFLAGS=-fPIC",
                ])
                .current_dir(&Path::new(&out_dir))
                .status()
                .unwrap();

            println!("cargo:rustc-link-lib=static=stdc++");

            system_deps::Library::from_internal_pkg_config(
                format!("{}/dist/lib/pkgconfig", out_dir),
                lib,
                version,
            )
        })
        .probe()
        .unwrap();
}
