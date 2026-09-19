mod bridge;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn print_help() {
    println!("FaceKey - Face ID for Linux");
    println!();
    println!("Usage: facekey [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --test-run   Walk through all screens with simulated responses.");
    println!("               No pkexec, no howdy calls, nothing is changed.");
    println!("  --version    Print version and exit");
    println!("  --help       Print this help and exit");
}

fn main() {
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--test-run" => std::env::set_var("FACEKEY_TEST_RUN", "1"),
            "--version" | "-V" => {
                println!("facekey {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "--help" | "-h" => {
                print_help();
                return;
            }
            // Ignore unknown args (session managers etc. may pass their own)
            _ => {}
        }
    }

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/howdy/gui/qml/main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}
