//! Builds the browser code: `client/` (TypeScript) is type-checked by `tsc` and bundled by esbuild
//! into one ES module, which `client_js` embeds and the server serves under a content hash.
//! Needs Node and `client/node_modules` (`npm ci --prefix client`).

use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    let client = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../../client");
    for p in ["src", "build.mjs", "tsconfig.json", "package.json"] {
        println!("cargo:rerun-if-changed={}", client.join(p).display());
    }
    let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("app.js");
    let release = env::var("PROFILE").is_ok_and(|p| p == "release");

    let node = |args: &[&str]| {
        let status = Command::new("node").current_dir(&client).args(args).status();
        match status {
            Ok(s) if s.success() => {}
            Ok(_) => panic!("client build failed: node {}", args.join(" ")),
            Err(e) => panic!("client build needs Node.js on PATH ({e}); then `npm ci --prefix client`"),
        }
    };
    if !client.join("node_modules/esbuild").exists() {
        panic!("client/node_modules missing: run `npm ci --prefix client`");
    }
    node(&["node_modules/typescript/bin/tsc", "--noEmit", "-p", "."]);
    let out_s = out.to_string_lossy().into_owned();
    let mut args = vec!["build.mjs", "--out", out_s.as_str()];
    if release {
        args.push("--release");
    }
    node(&args);

    // FNV-1a: a stable content hash for the file name, so the file can be cached forever.
    let hash = fs::read(&out)
        .expect("bundle written")
        .iter()
        .fold(0xcbf2_9ce4_8422_2325_u64, |h, b| (h ^ u64::from(*b)).wrapping_mul(0x100_0000_01b3));
    println!("cargo:rustc-env=APP_JS_HASH={hash:016x}");
}
