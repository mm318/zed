#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]
#![cfg_attr(any(not(target_os = "macos"), feature = "macos-blade"), allow(unused))]

use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gles)");

    #[cfg(any(
        not(any(target_os = "macos", target_os = "windows")),
        all(target_os = "macos", feature = "macos-blade")
    ))]
    check_wgsl_shaders();

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    if target_os == "windows" {
        #[cfg(feature = "windows-manifest")]
        embed_resource();
    }
}

#[cfg(any(
    not(any(target_os = "macos", target_os = "windows")),
    all(target_os = "macos", feature = "macos-blade")
))]
fn check_wgsl_shaders() {
    use std::path::PathBuf;
    use std::str::FromStr;

    let shader_source_path = "./src/platform/blade/shaders.wgsl";
    let shader_path = PathBuf::from_str(shader_source_path).unwrap();
    println!("cargo:rerun-if-changed={}", &shader_path.display());

    let shader_source = std::fs::read_to_string(&shader_path).unwrap();

    // Strip dual_source_blending features before validation since naga doesn't
    // support them yet, but blade handles them at runtime.
    let stripped_source = shader_source
        .replace("enable dual_source_blending;\n", "")
        .replace("@blend_src(0) ", "")
        .replace("@blend_src(1) ", "");

    match naga::front::wgsl::parse_str(&stripped_source) {
        Ok(_) => {
            // All clear
        }
        Err(e) => {
            println!("cargo::error=WGSL shader compilation failed:\n{}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(feature = "windows-manifest")]
fn embed_resource() {
    let manifest = std::path::Path::new("resources/windows/gpui.manifest.xml");
    let rc_file = std::path::Path::new("resources/windows/gpui.rc");
    println!("cargo:rerun-if-changed={}", manifest.display());
    println!("cargo:rerun-if-changed={}", rc_file.display());
    embed_resource::compile(rc_file, embed_resource::NONE)
        .manifest_required()
        .unwrap();
}
