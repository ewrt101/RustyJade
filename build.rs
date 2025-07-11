use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rustc-link-search=Z:/Jade2022/library");
    println!("cargo:rustc-link-lib=jom");
    println!("cargo:rustc-link-lib=jomutil");

    // Specify include directories
    println!("cargo:include=C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.22621.0\\ucrt");
    println!("cargo:include=C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.22621.0\\um");
    println!("cargo:include=C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.22621.0\\shared");
    println!("cargo:include=C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.44.35207\\include");

    

    // Generate bindings
    let bindings = bindgen::Builder::default()
        .header("external/wrapper.h")
        //.generate_inline_functions(true) // Keep inline functions
        .wrap_static_fns(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("--target=x86_64-pc-windows-msvc")
        .clang_arg("-std=c++20") // or another appropriate C++ standard
        .clang_arg("-x")
        .clang_arg("c++")
        //.clang_arg("-nostdinc")
        .clang_arg("-D__AVX512VLFP16INTRIN_H")
        .clang_arg("-D__AVX512FP16INTRIN_H")
        .clang_arg("-I")
        .clang_arg("C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.22621.0\\ucrt")
        .clang_arg("-I")
        .clang_arg("C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.22621.0\\um")
        .clang_arg("-I")
        .clang_arg("C:\\Program Files (x86)\\Windows Kits\\10\\Include\\10.0.22621.0\\shared")
        .clang_arg("-I")
        .clang_arg("C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.44.35207\\include")
        .generate()
        .expect("Unable to generate bindings");

    // Write bindings to file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}