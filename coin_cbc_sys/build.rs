use num_cpus;
use std::{env, path::Path, process::Command};

fn main() {
    let cores = num_cpus::get();
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let deps_dir = format!("{}/deps", src_dir);
    let dist_dir = format!("{}/dist", out_dir);
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
            "--reconfigure",
            &format!("--parallel-jobs={}", cores),
            &format!("--prefix={}", dist_dir),
            "ADD_FFLAGS=-fallow-argument-mismatch",
        ])
        .current_dir(&Path::new(&deps_dir))
        .status()
        .unwrap();
    println!("cargo:rustc-link-search=native={}/lib", dist_dir);
}
