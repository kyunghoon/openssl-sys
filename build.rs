use anyhow::Result;
use std::{process::Command, path::Path};

fn go() -> Result<()> {
    match std::env::var("TARGET") {
        Ok(target) if target == "aarch64-linux-android" => {
            match std::env::var("HOST") {
                Ok(host) if host == "x86_64-apple-darwin" => {
                    let out_dir = std::env::var("OUT_DIR").unwrap();
                    //let api_level = std::env::var("API_LEVEL").unwrap_or("23".to_owned());
                    let api_level = std::env::var("API_LEVEL").unwrap_or("24".to_owned());
                    let ndk_root = std::env::var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME is undefined");
                    
                    let toolchain = format!("{}/toolchains/aarch64-linux-android-4.9/prebuilt/darwin-x86_64/bin", ndk_root);
                    assert!(Path::new(toolchain.as_str()).exists(), "toolchain not found");

                    Command::new("./Configure")
                        .env("PATH", format!("{}:{}", toolchain.as_str(), env!("PATH")))
                        .arg("android-arm64")
                        .arg("-fPIC")
                        .arg(format!("-D__ANDROID_API__={}", api_level))
                        .arg("zlib")
                        .arg("no-asm")
                        .arg("no-shared")
                        .arg("no-unit-test")
                        .current_dir(format!("{}/openssl", env!("CARGO_MANIFEST_DIR"))).output()?;

                    Command::new("make")
                        .env("PATH", format!("{}:{}", toolchain.as_str(), env!("PATH")))
                        .current_dir(format!("{}/openssl", env!("CARGO_MANIFEST_DIR"))).output()?;

                    Command::new("make")
                        .env("PATH", format!("{}:{}", toolchain.as_str(), env!("PATH")))
                        .arg("install_sw")
                        .arg(format!("DESTDIR={}", out_dir))
                        .current_dir(format!("{}/openssl", env!("CARGO_MANIFEST_DIR"))).output()?;

                    println!("cargo:INCLUDE={}/usr/local/include", out_dir);
                    println!("cargo:LIB={}/usr/local/lib", out_dir);
                }
                x => panic!("{:?} not yet supported", x),
            }
        }
        x => panic!("{:?} not yet supported", x)
    };

    Ok(())
}

fn main() {
    if let Err(e) = go() {
        println!("cargo:warning={}", e);
    }
}
