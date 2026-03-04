use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=APP_ICON_ICO");
    println!("cargo:rerun-if-changed=build/app.png");

    if env::var("CARGO_CFG_TARGET_OS").ok().as_deref() != Some("windows") {
        return;
    }

    let icon_path = env::var("APP_ICON_ICO")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("build/windows/icon.ico"));

    if !icon_path.exists() {
        println!(
            "cargo:warning=Windows icon file not found, skip embedding: {}",
            icon_path.display()
        );
        return;
    }

    if let Err(err) = winresource::WindowsResource::new()
        .set_icon(icon_path.to_string_lossy().as_ref())
        .compile()
    {
        panic!("Failed to embed Windows icon: {err}");
    }
}
