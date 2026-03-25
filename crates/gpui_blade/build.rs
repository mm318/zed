#![allow(clippy::disallowed_methods, reason = "build scripts are exempt")]

fn main() {
    println!("cargo::rustc-check-cfg=cfg(gles)");
    check_wgsl_shaders();
}

fn check_wgsl_shaders() {
    use std::path::PathBuf;
    use std::str::FromStr;

    let shader_source_path = "./src/shaders.wgsl";
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
