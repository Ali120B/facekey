use cxx_qt_build::{CxxQtBuilder, QmlModule};
use std::fs;
use std::path::Path;

fn main() {
    let pkgbuild_path = Path::new("Packaging/PKGBUILD");
    if pkgbuild_path.exists() {
        if let Ok(content) = fs::read_to_string(pkgbuild_path) {
            for line in content.lines() {
                if line.starts_with("pkgver=") {
                    if let Some(ver) = line.strip_prefix("pkgver=") {
                        println!("cargo:rerun-if-changed=Packaging/PKGBUILD");
                        let pkgrel = content
                            .lines()
                            .find(|l| l.starts_with("pkgrel="))
                            .and_then(|l| l.strip_prefix("pkgrel="))
                            .unwrap_or("1");
                        println!("cargo:rustc-env=APP_VERSION={}-{}", ver.trim(), pkgrel);
                        break;
                    }
                }
            }
        }
    }
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=qml/");
    println!("cargo:rerun-if-changed=build.rs");
    CxxQtBuilder::new_qml_module(QmlModule::new("com.howdy.gui").qml_files([
        "qml/main.qml",
        "qml/UiButton.qml",
        "qml/UiSwitch.qml",
        "qml/UiCard.qml",
        "qml/SetupWizard.qml",
    ]))
        .file("src/bridge.rs")
        .build();
}
