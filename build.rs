fn main() {
    println!(
        "cargo:rustc-link-search={}/external/lib/x64/",
        std::env!("CARGO_MANIFEST_DIR")
    );
    println!("cargo:rustc-link-lib=OptickCore")
}