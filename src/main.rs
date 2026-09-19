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

/// Probe the host EGL stack directly. Qt aborts when EGL will not
/// initialize (broken drivers, minimal compositors); detecting it first
/// lets us fall back to XCB instead of crashing.
fn egl_works() -> bool {
    unsafe {
        let lib = match libloading::Library::new("libEGL.so.1") {
            Ok(l) => l,
            Err(_) => return false,
        };
        let get_display: Result<
            libloading::Symbol<unsafe extern "C" fn(*const std::ffi::c_void) -> *mut std::ffi::c_void>,
            _,
        > = lib.get(b"eglGetDisplay");
        let initialize: Result<
            libloading::Symbol<unsafe extern "C" fn(*mut std::ffi::c_void, *mut i32, *mut i32) -> u32>,
            _,
        > = lib.get(b"eglInitialize");
        let terminate: Result<
            libloading::Symbol<unsafe extern "C" fn(*mut std::ffi::c_void) -> u32>,
            _,
        > = lib.get(b"eglTerminate");
        let (get_display, initialize, terminate) = match (get_display, initialize, terminate) {
            (Ok(g), Ok(i), Ok(t)) => (g, i, t),
            _ => return false,
        };
        // EGL_DEFAULT_DISPLAY
        let dpy = get_display(std::ptr::null());
        if dpy.is_null() {
            return false;
        }
        let (mut major, mut minor) = (0i32, 0i32);
        let ok = initialize(dpy, &mut major, &mut minor);
        let _ = terminate(dpy);
        ok == 1 // EGL_TRUE
    }
}

fn main() {
    // Pin the FFmpeg camera backend: on some systems QtMultimedia would
    // otherwise probe a half-present GStreamer stack and spam assertion
    // failures while opening cameras.
    std::env::set_var("QT_MEDIA_BACKEND", "ffmpeg");

    // If the session wants Wayland but host EGL will not initialize, Qt
    // would abort during RHI creation. Fall back to XCB/XWayland instead —
    // strictly better than a crash (healthy setups never take this path).
    let platform = std::env::var("QT_QPA_PLATFORM").unwrap_or_default();
    let wayland_wanted =
        platform == "wayland" || (platform.is_empty() && std::env::var("WAYLAND_DISPLAY").is_ok());
    if wayland_wanted && !egl_works() {
        eprintln!("FaceKey: EGL unavailable, falling back to XCB");
        std::env::set_var("QT_QPA_PLATFORM", "xcb");
    }

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
