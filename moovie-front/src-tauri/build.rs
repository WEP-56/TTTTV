fn main() {
    println!("cargo:warning=Running build.rs");
    tauri_build::build()
}
