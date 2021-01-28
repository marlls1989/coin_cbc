use num_cpus;
use std::{env, path::Path, process::Command};
use system_deps;

fn main() {
    system_deps::Config::new()
        .add_build_internal("cbc", |lib, version| {
            let cores = num_cpus::get();
            let out_dir = env::var("OUT_DIR").unwrap();
            let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

            let deps_dir = format!("{}/deps", src_dir);
            let dest_dir = format!("{}/cbc", out_dir);

            Command::new(&format!("{}/coinbrew", deps_dir))
                .args(&["fetch", "Cbc@2.10.5"])
                .current_dir(&Path::new(&deps_dir))
                .status()
                .unwrap();

            Command::new(&format!("{}/coinbrew", deps_dir))
                .args(&[
                    "build",
                    "Cbc",
                    "--static",
                    "--no-prompt",
                    "--tests=none",
                    &format!("--parallel-jobs={}", cores),
                    &format!("--prefix={}", dest_dir),
                    "ADD_FFLAGS=-fallow-argument-mismatch -fPIC",
                    "ADD_CFLAGS=-fPIC",
                    "ADD_CXXFLAGS=-fPIC",
                ])
                .current_dir(&Path::new(&deps_dir))
                .status()
                .unwrap();

            if cfg!(target_os = "macos") {
                println!("cargo:rustc-link-lib=c++");
            } else if cfg!(target_os = "linux") {
                println!("cargo:rustc-link-lib=static=stdc++");
            }

            system_deps::Library::from_internal_pkg_config(
                format!("{}/lib/pkgconfig", dest_dir),
                lib,
                version,
            )
        })
        .probe()
        .unwrap();
}
