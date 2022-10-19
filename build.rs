use anyhow::Result;
use std::{process::Command, path::Path, env::var};

macro_rules! check {
    ($out: expr) => {
        let out = $out;
        if !out.status.success() {
            panic!("{}", std::str::from_utf8(&out.stderr)?);
        }
    };
}

fn go() -> Result<()> {
    match std::env::var("TARGET") {
        Ok(target) if target == "aarch64-linux-android" => {
            match std::env::var("HOST") {
                Ok(host) if host == "x86_64-apple-darwin" => {

                    let out_dir = var("OUT_DIR").unwrap();
                    let ndk_root = var("ANDROID_NDK_HOME").expect("ANDROID_NDK_HOME is undefined");
                    //let api_level = var("API_LEVEL").unwrap_or("24".to_owned());
                    let api_level = var("API_LEVEL").unwrap_or("23".to_owned());

                    let toolchain = format!("{}/toolchains/{}-4.9/prebuilt/darwin-x86_64/bin", ndk_root, target);
                    assert!(Path::new(toolchain.as_str()).exists(), "toolchain not found");

                    check!(Command::new("./Configure")
                        .env("PATH", format!("{}:{}", toolchain, env!("PATH")))
                        .env("ANDROID_NDK", ndk_root.as_str())
                        .arg("android-arm64")
                        .arg("-fPIC")
                        .arg(format!("-D__ANDROID_API__={}", api_level))
                        .arg("zlib")
                        .arg("no-asm")
                        .arg("no-shared")
                        .arg("no-unit-test")
                        //.arg(if var("PROFILE").unwrap() == "debug" { "--debug" } else { "--release" })
                        .current_dir(format!("{}/openssl", env!("CARGO_MANIFEST_DIR"))).output()?);

                    check!(Command::new("make")
                        .env("PATH", format!("{}:{}", toolchain, env!("PATH")))
                        .env("ANDROID_NDK", ndk_root.as_str())
                        .current_dir(format!("{}/openssl", env!("CARGO_MANIFEST_DIR"))).output()?);

                    check!(Command::new("make")
                        .env("PATH", format!("{}:{}", toolchain, env!("PATH")))
                        .env("ANDROID_NDK", ndk_root.as_str())
                        .arg(format!("DESTDIR={}", out_dir))
                        .arg("install_sw")
                        .current_dir(format!("{}/openssl", env!("CARGO_MANIFEST_DIR"))).output()?);

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
