#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("cxx-qt-lib/qlist.h");
        type QList_QString = cxx_qt_lib::QList<cxx_qt_lib::QString>;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QList_QString, face_models)]
        #[qproperty(QList_QString, video_devices)]
        #[qproperty(QString, status_message)]
        #[qproperty(bool, howdy_enabled)]
        #[qproperty(bool, device_supported)]
        #[qproperty(bool, camera_configured)]
        #[qproperty(bool, pam_sddm)]
        #[qproperty(bool, pam_kde)]
        #[qproperty(bool, pam_sudo)]
        #[qproperty(bool, pam_system_login)]
        #[qproperty(bool, pam_plasma_lm)]
        #[qproperty(bool, pam_polkit)]
        #[qproperty(bool, pam_hyprlock)]
        #[qproperty(bool, sddm_installed)]
        #[qproperty(bool, plasma_lm_installed)]
        #[qproperty(bool, hyprlock_installed)]
        #[qproperty(QString, app_version)]
        // ── Setup wizard / Doctor state ──
        #[qproperty(bool, setup_howdy)]
        #[qproperty(bool, setup_pam_python)]
        #[qproperty(bool, setup_models)]
        #[qproperty(bool, setup_toolchain)]
        #[qproperty(QString, setup_aur_helper)]
        #[qproperty(bool, setup_agent)]
        #[qproperty(bool, setup_ir_camera)]
        #[qproperty(QList_QString, camera_candidates)]
        #[qproperty(QList_QString, camera_paths)]
        #[qproperty(QString, suggested_camera)]
        #[qproperty(QString, install_log)]
        #[qproperty(bool, install_running)]
        #[qproperty(bool, install_done)]
        #[qproperty(QString, install_error)]
        type HowdyBackend = super::HowdyBackendRust;

        /// Check if device has supported IR camera
        #[qinvokable]
        fn check_device(self: Pin<&mut HowdyBackend>);

        /// Refresh the list of face models
        #[qinvokable]
        fn refresh_models(self: Pin<&mut HowdyBackend>);

        /// Add a new face model with the given name (async)
        #[qinvokable]
        fn add_model(self: Pin<&mut HowdyBackend>, name: QString);

        /// Check for pending add_model result from the background thread
        #[qinvokable]
        fn check_add_result(self: Pin<&mut HowdyBackend>) -> bool;

        /// Discard the just-registered face (delete it)
        #[qinvokable]
        fn discard_face(self: Pin<&mut HowdyBackend>);

        /// Remove a face model by index
        #[qinvokable]
        fn remove_model(self: Pin<&mut HowdyBackend>, index: i32);

        /// Toggle howdy enabled/disabled
        #[qinvokable]
        fn toggle_enabled(self: Pin<&mut HowdyBackend>);

        /// Run a test of face recognition
        #[qinvokable]
        fn run_test(self: Pin<&mut HowdyBackend>);

        /// Scan /dev/ for video* devices and populate video_devices
        #[qinvokable]
        fn load_video_devices(self: Pin<&mut HowdyBackend>);

        /// Preview a device with mpv
        #[qinvokable]
        fn test_camera(self: Pin<&mut HowdyBackend>, device: QString);

        /// Write device_path into the howdy config file
        #[qinvokable]
        fn save_device_path(self: Pin<&mut HowdyBackend>, device: QString);

        /// Read all PAM files and update the pam_* properties
        #[qinvokable]
        fn check_pam_status(self: Pin<&mut HowdyBackend>);

        /// Add, uncomment or comment the howdy line in the given PAM file
        #[qinvokable]
        fn toggle_pam(self: Pin<&mut HowdyBackend>, file: QString);

        /// Detect which display managers (SDDM / plasma-login-manager) are installed
        #[qinvokable]
        fn detect_display_managers(self: Pin<&mut HowdyBackend>);

        /// Run all setup preflight checks (howdy, pam-python, models,
        /// toolchain, AUR helper, polkit agent, IR camera)
        #[qinvokable]
        fn run_preflight(self: Pin<&mut HowdyBackend>);

        /// Probe /dev/video* devices: format summaries, IR heuristic,
        /// suggested_camera. Populates camera_candidates/camera_paths.
        #[qinvokable]
        fn probe_cameras(self: Pin<&mut HowdyBackend>);

        /// Install repo packages via pkexec pacman in a thread, streaming
        /// to /tmp/facekey_install.log
        #[qinvokable]
        fn start_repo_install(self: Pin<&mut HowdyBackend>);

        /// Append new install log output to install_log; returns true when
        /// the background install finished (check install_done/install_error)
        #[qinvokable]
        fn poll_install_log(self: Pin<&mut HowdyBackend>) -> bool;

        /// Open a user terminal running the AUR install (yay refuses root,
        /// so this runs unelevated and yay asks for sudo itself)
        #[qinvokable]
        fn launch_aur_install(self: Pin<&mut HowdyBackend>);

        /// True when howdy + pam-python are both present
        #[qinvokable]
        fn check_install_done(self: Pin<&mut HowdyBackend>) -> bool;
    }
}

use core::pin::Pin;
use cxx_qt_lib::{QList, QString};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Default)]
pub struct HowdyBackendRust {
    face_models: QList<QString>,
    video_devices: QList<QString>,
    status_message: QString,
    howdy_enabled: bool,
    device_supported: bool,
    camera_configured: bool,
    pam_sddm: bool,
    pam_kde: bool,
    pam_sudo: bool,
    pam_system_login: bool,
    pam_plasma_lm: bool,
    pam_polkit: bool,
    pam_hyprlock: bool,
    sddm_installed: bool,
    plasma_lm_installed: bool,
    hyprlock_installed: bool,
    app_version: QString,
    // ── Setup wizard / Doctor state ──
    setup_howdy: bool,
    setup_pam_python: bool,
    setup_models: bool,
    setup_toolchain: bool,
    setup_aur_helper: QString,
    setup_agent: bool,
    setup_ir_camera: bool,
    camera_candidates: QList<QString>,
    camera_paths: QList<QString>,
    suggested_camera: QString,
    install_log: QString,
    install_running: bool,
    install_done: bool,
    install_error: QString,
}

const PAM_LINE_DEBIAN: &str = "auth sufficient pam_howdy.so";
const PAM_LINE_ARCH: &str = "auth sufficient pam_python.so /lib/security/howdy/pam.py";
// Kept for reference; use pam_module_lines() instead of these directly.
#[allow(dead_code)]
const PAM_LINE: &str = PAM_LINE_DEBIAN;
const PAM_SDDM: &str = "/etc/pam.d/sddm";const PAM_KDE: &str = "/etc/pam.d/kde";
const PAM_SUDO: &str = "/etc/pam.d/sudo";
const PAM_SYSTEM_LOGIN: &str = "/etc/pam.d/system-local-login";
const PAM_PLASMA_LM: &str = "/etc/pam.d/plasmalogin";
const PAM_POLKIT: &str = "/etc/pam.d/polkit-1";
const PAM_HYPRLOCK: &str = "/etc/pam.d/hyprlock";
// polkit-agent-helper resolves PAM modules in a restricted sandbox; use the full path.
const PAM_POLKIT_LINE_DEBIAN: &str = "auth sufficient /lib/security/pam_howdy.so";
const PAM_POLKIT_LINE_ARCH: &str = "auth sufficient pam_python.so /lib/security/howdy/pam.py";
#[allow(dead_code)]
const PAM_POLKIT_LINE: &str = PAM_POLKIT_LINE_DEBIAN;

/// Pick the PAM line matching the installed Howdy flavour:
/// - Debian/upstream style ships pam_howdy.so -> use it
/// - Arch/CachyOS (howdy 2.6.x) uses pam_python.so + pam.py -> use that
fn pam_module_lines() -> (&'static str, &'static str) {
    if Path::new("/lib/security/pam_howdy.so").exists()
        || Path::new("/usr/lib/security/pam_howdy.so").exists()
    {
        (PAM_LINE_DEBIAN, PAM_POLKIT_LINE_DEBIAN)
    } else {
        (PAM_LINE_ARCH, PAM_POLKIT_LINE_ARCH)
    }
}

/// Read a file, falling back to `pkexec cat` when unprivileged reads fail
/// (some installs ship /usr/lib/security/howdy with root-only permissions).
fn read_file_privileged_fallback(path: &str) -> Option<String> {
    if let Ok(content) = fs::read_to_string(path) {
        return Some(content);
    }
    Command::new("pkexec")
        .args(["/usr/bin/cat", path])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
}

/// Returns true if a non-commented howdy auth line is present in the PAM file content
fn pam_howdy_active(content: &str) -> bool {
    content.lines().any(|line| {
        let t = line.trim();
        !t.starts_with('#') && (t.contains("pam_howdy.so") || t.contains("howdy/pam.py"))
    })
}

/// Run Howdy elevated via pkexec. Two quirks are handled here:
/// - `-U <user>` is always passed: under pkexec Howdy would otherwise resolve
///   the user as root and operate on root's (empty) model store.
/// - `SUDO_USER` is injected via /usr/bin/env: pkexec never sets it, but
///   Howdy's CLI gates on it ("Please run this command as root"). Without this
///   every elevated call fails even after successful authentication.
fn pkexec_howdy(howdy: &str, user: &str, args: &[&str]) -> std::io::Result<std::process::Output> {
    Command::new("pkexec")
        .arg("/usr/bin/env")
        .arg(format!("SUDO_USER={}", user))
        .arg(howdy)
        .arg("-U")
        .arg(user)
        .args(args)
        .output()
}


/// The desktop user Howdy models belong to. The GUI runs unprivileged as the
/// user, but its `pkexec howdy ...` children run as root — and Howdy resolves
/// the "current user" via getlogin()/SUDO_USER, which both yield root under
/// pkexec. So every user-scoped call must pass `-U <user>` explicitly.
fn target_user() -> String {
    for key in ["SUDO_USER", "USER", "LOGNAME"] {
        if let Ok(v) = std::env::var(key) {
            let v = v.trim().to_string();
            if !v.is_empty() && v != "root" {
                return v;
            }
        }
    }
    Command::new("logname")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "root")
        .unwrap_or_else(|| "root".to_string())
}

/// Resolve the absolute path to the howdy binary
fn find_howdy() -> Option<String> {
    if let Ok(out) = Command::new("which").arg("howdy").output() {
        if out.status.success() {
            if let Ok(s) = String::from_utf8(out.stdout) {
                let s = s.trim().to_string();
                if !s.is_empty() {
                    return Some(s);
                }
            }
        }
    }
    // Fallback for installs where `which` fails (e.g. restrictive perms on
    // /usr/lib/security/howdy breaking the symlink target check).
    for candidate in ["/usr/bin/howdy", "/usr/local/bin/howdy"] {
        if Path::new(candidate).exists() {
            return Some(candidate.to_string());
        }
    }
    None
}

/// Check if howdy is disabled in config content
fn is_howdy_disabled(config_content: &str) -> bool {
    for line in config_content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("disabled") {
            if let Some(value) = trimmed.split('=').nth(1) {
                let val = value.trim();
                return val.eq_ignore_ascii_case("true") || val == "1";
            }
        }
    }
    false // Default: not disabled (enabled)
}

/// Returns true if the given package is installed (pacman -Q)
fn pacman_installed(pkg: &str) -> bool {
    Command::new("pacman")
        .args(["-Q", pkg])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

// ── Setup wizard helpers (Arch backend; see task.md install plan) ──

/// Repo packages FaceKey needs from pacman
const REPO_PACKAGES: &[&str] = &[
    "qt6-base",
    "qt6-declarative",
    "qt6-multimedia",
    "v4l-utils",
    "mpv",
    "polkit",
    "gcc",
    "make",
    "pkgconf",
    "fakeroot",
];

/// dlib model files that must ship inside the howdy package
const DLIB_MODELS: &[&str] = &[
    "shape_predictor_5_face_landmarks.dat",
    "dlib_face_recognition_resnet_model_v1.dat",
    "mmod_human_face_detector.dat",
];

/// First available program in PATH, or None
fn which_first(candidates: &[&str]) -> Option<String> {
    for prog in candidates {
        if let Ok(out) = Command::new("which").arg(prog).output() {
            if out.status.success() {
                if let Ok(s) = String::from_utf8(out.stdout) {
                    let s = s.trim().to_string();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
        }
    }
    None
}

fn pam_python_present() -> bool {
    Path::new("/usr/lib/security/pam_python.so").exists()
        || Path::new("/lib/security/pam_python.so").exists()
}

fn dlib_models_present() -> bool {
    DLIB_MODELS.iter().all(|f| {
        Path::new(&format!("/usr/lib/security/howdy/dlib-data/{}", f)).exists()
    })
}

fn polkit_agent_running() -> bool {
    Command::new("pgrep")
        .args([
            "-f",
            "hyprpolkitagent|polkit-gnome-authentication-agent|polkit-kde-authentication-agent|polkit-mate-authentication-agent|lxpolkit",
        ])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Sorted /dev/video* nodes
fn list_video_devices() -> Vec<String> {
    let mut device_list: Vec<String> = fs::read_dir("/dev")
        .into_iter()
        .flatten()
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            if name.starts_with("video") {
                Some(format!("/dev/{}", name))
            } else {
                None
            }
        })
        .collect();
    device_list.sort();
    device_list
}

/// Parsed capability summary for one /dev/videoN node
struct CameraProbe {
    path: String,
    summary: String,
    likely_ir: bool,
}

fn probe_video_device(path: &str) -> CameraProbe {
    let mut sizes: Vec<(u32, u32)> = Vec::new();
    if let Ok(out) = Command::new("v4l2-ctl")
        .args(["-d", path, "--list-formats-ext"])
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let t = line.trim();
                if let Some(rest) = t.strip_prefix("Size: Discrete ") {
                    let mut it = rest.split('x');
                    if let (Some(w), Some(h)) = (it.next(), it.next()) {
                        if let (Ok(w), Ok(h)) = (w.parse::<u32>(), h.parse::<u32>()) {
                            sizes.push((w, h));
                        }
                    }
                }
            }
        }
    }
    sizes.sort_by_key(|(w, h)| w.saturating_mul(*h));
    // IR sensors typically expose a small square frame (e.g. 340x340)
    let likely_ir = sizes
        .iter()
        .any(|(w, h)| w == h && *w <= 480 && *w >= 100);
    let summary = sizes
        .last()
        .map(|(w, h)| format!("{}x{}", w, h))
        .unwrap_or_else(|| "no capture formats".to_string());
    CameraProbe {
        path: path.to_string(),
        summary,
        likely_ir,
    }
}

impl qobject::HowdyBackend {
    /// Detect which display managers are present and set the corresponding properties.
    /// Falls back to showing SDDM if neither is found.
    pub fn detect_display_managers(mut self: Pin<&mut Self>) {
        let version = env!("APP_VERSION").to_string();
        self.as_mut().set_app_version(QString::from(&version));

        let sddm = Path::new("/usr/bin/sddm").exists() || pacman_installed("sddm");
        // Arch/CachyOS names the binary `plasmalogin` (package: plasma-login-manager)
        let plm = Path::new("/usr/bin/plasma-login-manager").exists()
            || Path::new("/usr/bin/plasmalogin").exists()
            || pacman_installed("plasma-login-manager");

        self.as_mut().set_sddm_installed(sddm);
        self.as_mut().set_plasma_lm_installed(plm);

        // Neither detected → fall back to showing SDDM (most common on Arch)
        if !sddm && !plm {
            self.as_mut().set_sddm_installed(true);
        }

        // Screen locker actually in use on Hyprland setups
        let hyprlock = Path::new("/usr/bin/hyprlock").exists() || pacman_installed("hyprlock");
        self.as_mut().set_hyprlock_installed(hyprlock);
    }

    /// Check if device has a supported IR camera for howdy
    pub fn check_device(mut self: Pin<&mut Self>) {
        // Check multiple conditions for device support:
        // 1. Check if howdy is installed
        // 2. Use v4l2-ctl to detect video devices (per Arch Wiki)
        // 3. Check howdy config for device path
        // 4. Prefer stable paths (/dev/v4l/by-path/) over /dev/videoX

        let mut supported = false;
        let mut status_msg = String::new();

        // Check if howdy command exists
        let howdy_exists = Command::new("which")
            .arg("howdy")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        if !howdy_exists {
            status_msg = "Howdy is not installed".to_string();
            self.as_mut().set_device_supported(false);
            self.as_mut().set_status_message(QString::from(&status_msg));
            return;
        }

        // Use v4l2-ctl to list devices (recommended by Arch Wiki)
        let v4l2_output = Command::new("v4l2-ctl").arg("--list-devices").output();

        let has_video_devices = match &v4l2_output {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                !stdout.trim().is_empty()
            }
            _ => {
                // Fallback: check /dev/v4l/by-path/ for stable device paths (per Arch Wiki)
                Path::new("/dev/v4l/by-path").exists()
                    && fs::read_dir("/dev/v4l/by-path")
                        .map(|entries| entries.count() > 0)
                        .unwrap_or(false)
            }
        };

        if !has_video_devices {
            status_msg =
                "No video devices found (install v4l-utils for better detection)".to_string();
            self.as_mut().set_device_supported(false);
            self.as_mut().set_status_message(QString::from(&status_msg));
            return;
        }

        // Check howdy config file for device_path
        // Priority order per Arch Wiki: /lib/security/howdy/config.ini is the main config
        let config_paths = [
            "/lib/security/howdy/config.ini",
            "/usr/lib/security/howdy/config.ini",
            "/etc/howdy/config.ini",
        ];

        let mut device_path_configured = false;
        for config_path in &config_paths {
            if Path::new(config_path).exists() {
                if let Some(content) = read_file_privileged_fallback(config_path) {
                    // Check if device_path is set and not "none"
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if trimmed.starts_with("device_path") {
                            if let Some(value) = trimmed.split('=').nth(1) {
                                let device = value.trim();
                                if !device.is_empty() && device != "none" && device != "null" {
                                    // Check if the configured device actually exists
                                    // Supports both /dev/videoX and stable /dev/v4l/by-path/ paths
                                    if Path::new(device).exists() {
                                        device_path_configured = true;
                                        supported = true;
                                        // Show friendly path info
                                        if device.contains("/by-path/")
                                            || device.contains("/by-id/")
                                        {
                                            status_msg = format!(
                                                "Device configured (stable path): {}",
                                                device
                                            );
                                        } else {
                                            status_msg = format!("Device found: {}", device);
                                        }
                                    } else {
                                        status_msg =
                                            format!("Configured device {} not found", device);
                                    }
                                }
                            }
                            break;
                        }
                    }

                    // Also check if howdy is disabled in config
                    let howdy_enabled = !is_howdy_disabled(&content);
                    self.as_mut().set_howdy_enabled(howdy_enabled);
                }
                break;
            }
        }

        if !device_path_configured && has_video_devices {
            // Devices exist but may not be configured
            supported = true;
            status_msg =
                "Video device(s) found - run 'sudo howdy config' to set device_path".to_string();
        }

        self.as_mut().set_device_supported(supported);
        self.as_mut().set_status_message(QString::from(&status_msg));
    }

    /// Refresh the list of face models from howdy
    pub fn refresh_models(mut self: Pin<&mut Self>) {
        let howdy = match find_howdy() {
            Some(p) => p,
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Howdy is not installed"));
                return;
            }
        };
        // Try without elevated privileges first; the models dir is often user-readable.
        // Only fall back to pkexec if the unprivileged call fails — this avoids an
        // extra password prompt on every startup / after every add/remove.
        // NOTE: always pass -U: under pkexec Howdy would otherwise resolve the
        // user as root and list root's (empty) models instead of the user's.
        let user = target_user();
        let output = Command::new(&howdy)
            .args(["-U", &user, "list"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .or_else(|| {
                let user = target_user();
                pkexec_howdy(&howdy, &user, &["list"]).ok()
            });

        match output {
            Some(out) => {
                // A failed pkexec (auth rejected/cancelled) or a failing howdy
                // must NOT look like "no faces" — say what actually happened.
                if !out.status.success() {
                    let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                    let stdout = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    let detail = if stderr.contains("Not authorized")
                        || stderr.contains("dismissed")
                        || stderr.contains("cancelled")
                        || stderr.contains("Authentication failed")
                    {
                        "authorization failed or was cancelled".to_string()
                    } else if !stderr.is_empty() {
                        stderr.lines().last().unwrap_or("unknown error").to_string()
                    } else if !stdout.is_empty() {
                        stdout.lines().last().unwrap_or("unknown error").to_string()
                    } else {
                        format!("exit code: {:?}", out.status.code())
                    };
                    self.as_mut().set_face_models(QList::<QString>::default());
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Could not list models: {}",
                        detail
                    )));
                    return;
                }
                let stdout = String::from_utf8_lossy(&out.stdout);
                let stderr = String::from_utf8_lossy(&out.stderr);
                let mut models = QList::<QString>::default();

                // Check for "No users registered" message (howdy outputs this when empty)
                let combined = format!("{}{}", stdout, stderr).to_lowercase();
                if combined.contains("no users") || combined.contains("no models") {
                    self.as_mut().set_face_models(models);
                    self.as_mut()
                        .set_status_message(QString::from("No faces registered"));
                    return;
                }

                // Parse howdy list output - each line after header is a model
                for line in stdout.lines() {
                    let trimmed = line.trim();
                    // Skip empty lines, header lines, and separator lines
                    if trimmed.is_empty()
                        || trimmed.starts_with("ID")
                        || trimmed.contains("──")
                        || trimmed.contains("---")
                    {
                        continue;
                    }
                    // Parse lines like: "0  model_name  2024-01-15"
                    if let Some(first_char) = trimmed.chars().next() {
                        if first_char.is_ascii_digit() {
                            models.append_clone(&QString::from(trimmed));
                        }
                    }
                }

                let count = models.len();
                self.as_mut().set_face_models(models);
                if count > 0 {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Found {} registered face(s)",
                        count
                    )));
                } else {
                    self.as_mut()
                        .set_status_message(QString::from("No faces registered"));
                }
            }
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Failed to list models"));
            }
        }
    }

    /// Add a new face model — runs howdy asynchronously
    pub fn add_model(mut self: Pin<&mut Self>, name: QString) {
        let name_str = name.to_string();
        if name_str.is_empty() {
            self.as_mut()
                .set_status_message(QString::from("Failed: Please enter a model name"));
            return;
        }

        let howdy = match find_howdy() {
            Some(p) => p,
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Failed: Howdy is not installed"));
                return;
            }
        };

        // Set initial status - this tells QML capture has started
        self.as_mut()
            .set_status_message(QString::from("Look at the camera..."));

        // Spawn a thread to run the blocking pkexec call.
        // NOTE: stock howdy ignores any name argument when -y is passed and
        // always labels the model "Model #N", so the requested name is applied
        // afterwards by patching the fresh entry in the JSON model store —
        // chained in the SAME pkexec call so there is only one auth dialog.
        let howdy_clone = howdy.clone();
        let name_clone = name_str.clone();
        let user_clone = target_user();

        std::thread::spawn(move || {
            let label: String = name_clone.chars().take(24).collect();
            let safe_label = label.replace('\'', "");
            let script = format!(
                "/usr/bin/env SUDO_USER={u} {h} -U {u} add -y && /usr/bin/python3 -c \"import json; p='/lib/security/howdy/models/{u}.dat'; m=json.load(open(p)); m[-1]['label']='{l}'; json.dump(m, open(p, 'w'))\"",
                u = user_clone,
                h = howdy_clone,
                l = safe_label
            );
            let output = Command::new("pkexec")
                .args(["/usr/bin/bash", "-c", &script])
                .output();

            let result_msg = match output {
                Ok(out) if out.status.success() => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    let face_id = stdout
                        .lines()
                        .find(|l| l.trim().starts_with("Face added as "))
                        .and_then(|l| {
                            l.trim()
                                .split_whitespace()
                                .last()
                                .and_then(|s| s.parse::<i32>().ok())
                        });
                    if let Some(id) = face_id {
                        let _ = std::fs::write("/tmp/howdy_new_face_id.txt", id.to_string());
                    }
                    "Face registered successfully".to_string()
                }
                Ok(out) => {
                    let combined = format!(
                        "{}{}",
                        String::from_utf8_lossy(&out.stdout),
                        String::from_utf8_lossy(&out.stderr)
                    )
                    .trim()
                    .to_string();
                    if combined.is_empty() {
                        format!("Failed (exit code: {:?})", out.status.code())
                    } else {
                        format!("Failed: {}", combined)
                    }
                }
                Err(e) => format!("Failed: {}", e),
            };

            let _ = std::fs::write("/tmp/howdy_add_result.txt", &result_msg);
        });
    }

    /// Check if there's a pending add_model result (called periodically from QML)
    /// Returns true if a result was found and processed
    pub fn check_add_result(mut self: Pin<&mut Self>) -> bool {
        if let Ok(result) = std::fs::read_to_string("/tmp/howdy_add_result.txt") {
            let _ = std::fs::remove_file("/tmp/howdy_add_result.txt");
            self.as_mut().set_status_message(QString::from(&result));
            return true;
        }
        false
    }

    /// Discard the just-registered face (delete it)
    pub fn discard_face(mut self: Pin<&mut Self>) {
        let howdy = match find_howdy() {
            Some(p) => p,
            None => return,
        };

        if let Ok(id_str) = std::fs::read_to_string("/tmp/howdy_new_face_id.txt") {
            let _ = std::fs::remove_file("/tmp/howdy_new_face_id.txt");
            if let Ok(id) = id_str.trim().parse::<i32>() {
                let user = target_user();
                let _ = pkexec_howdy(&howdy, &user, &["remove", "-y", &id.to_string()]);
            }
        }
        self.as_mut()
            .set_status_message(QString::from("Face discarded"));
    }

    /// Remove a face model by its ID
    pub fn remove_model(mut self: Pin<&mut Self>, index: i32) {
        let howdy = match find_howdy() {
            Some(p) => p,
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Howdy is not installed"));
                return;
            }
        };
        let user = target_user();
        let output = pkexec_howdy(&howdy, &user, &["remove", "-y", &index.to_string()]);

        match output {
            Ok(out) => {
                if out.status.success() {
                    self.as_mut()
                        .set_status_message(QString::from("Model removed"));
                    self.refresh_models();
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    self.as_mut()
                        .set_status_message(QString::from(&format!("Failed: {}", stderr)));
                }
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error: {}", e)));
            }
        }
    }

    /// Toggle howdy enabled/disabled
    pub fn toggle_enabled(mut self: Pin<&mut Self>) {
        let howdy = match find_howdy() {
            Some(p) => p,
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Howdy is not installed"));
                return;
            }
        };
        let current = *self.as_ref().howdy_enabled();
        let arg = if current { "1" } else { "0" };

        let output = Command::new("pkexec")
            .args([&howdy, "disable", arg])
            .output();

        match output {
            Ok(out) => {
                if out.status.success() {
                    self.as_mut().set_howdy_enabled(!current);
                    let msg = if current {
                        "Howdy disabled"
                    } else {
                        "Howdy enabled"
                    };
                    self.as_mut().set_status_message(QString::from(msg));
                } else {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    self.as_mut()
                        .set_status_message(QString::from(&format!("Failed: {}", stderr)));
                }
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error: {}", e)));
            }
        }
    }

    /// Scan /dev/ for video* devices and check if howdy's device_path is already configured
    pub fn load_video_devices(mut self: Pin<&mut Self>) {
        let device_list: Vec<String> = list_video_devices();

        let mut devices = QList::<QString>::default();
        for d in &device_list {
            devices.append_clone(&QString::from(d.as_str()));
        }
        self.as_mut().set_video_devices(devices);

        // Check if device_path is already set to something other than "none" in the config
        let config_candidates = [
            "/usr/lib/security/howdy/config.ini",
            "/lib/security/howdy/config.ini",
            "/etc/howdy/config.ini",
        ];
        let mut configured = false;
        for candidate in &config_candidates {
            if let Some(content) = read_file_privileged_fallback(candidate) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if !trimmed.starts_with('#') && trimmed.starts_with("device_path") {
                        if let Some(value) = trimmed.split('=').nth(1) {
                            let device = value.trim();
                            if !device.is_empty() && device != "none" && device != "null" {
                                configured = true;
                            }
                        }
                        break;
                    }
                }
                break;
            }
        }
        self.as_mut().set_camera_configured(configured);
    }

    /// Launch mpv to preview the selected camera device
    pub fn test_camera(mut self: Pin<&mut Self>, device: QString) {
        let device_str = device.to_string();
        if device_str.is_empty() {
            self.as_mut()
                .set_status_message(QString::from("No device selected"));
            return;
        }
        match Command::new("mpv").arg(&device_str).spawn() {
            Ok(_) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Previewing {} — close mpv when done",
                    device_str
                )));
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Failed to launch mpv: {}", e)));
            }
        }
    }

    /// Write device_path into the howdy config file using pkexec
    pub fn save_device_path(mut self: Pin<&mut Self>, device: QString) {
        let device_str = device.to_string();
        if device_str.is_empty() {
            self.as_mut()
                .set_status_message(QString::from("No device selected"));
            return;
        }

        let config_candidates = [
            "/usr/lib/security/howdy/config.ini",
            "/lib/security/howdy/config.ini",
            "/etc/howdy/config.ini",
        ];

        let mut config_path: Option<&str> = None;
        let mut current_content = String::new();
        for candidate in &config_candidates {
            if Path::new(candidate).exists() {
                if let Some(content) = read_file_privileged_fallback(candidate) {
                    current_content = content;
                    config_path = Some(candidate);
                    break;
                }
            }
        }

        let config_path = match config_path {
            Some(p) => p,
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Howdy config not found"));
                return;
            }
        };

        // Replace the device_path line, or append it if absent
        let mut found = false;
        let mut lines: Vec<String> = current_content
            .lines()
            .map(|line| {
                let trimmed = line.trim();
                if !trimmed.starts_with('#') && trimmed.starts_with("device_path") {
                    found = true;
                    format!("device_path = {}", device_str)
                } else {
                    line.to_string()
                }
            })
            .collect();
        if !found {
            lines.push(format!("device_path = {}", device_str));
        }
        let new_content = lines.join("\n") + "\n";

        // Write to a temp file, then copy it with elevated privileges
        let tmp = "/tmp/howdy_config_tmp.ini";
        if let Err(e) = fs::write(tmp, &new_content) {
            self.as_mut()
                .set_status_message(QString::from(&format!("Failed to write temp file: {}", e)));
            return;
        }

        let output = Command::new("pkexec")
            .args(["/usr/bin/cp", tmp, config_path])
            .output();
        let _ = fs::remove_file(tmp);

        match output {
            Ok(out) if out.status.success() => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Camera saved: {}", device_str)));
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                self.as_mut()
                    .set_status_message(QString::from(&format!("Failed to save: {}", stderr)));
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error: {}", e)));
            }
        }
    }

    /// Read all PAM files and update the corresponding properties
    pub fn check_pam_status(mut self: Pin<&mut Self>) {
        let read = |p| {
            fs::read_to_string(p)
                .map(|c| pam_howdy_active(&c))
                .unwrap_or(false)
        };
        self.as_mut().set_pam_sddm(read(PAM_SDDM));
        self.as_mut().set_pam_kde(read(PAM_KDE));
        self.as_mut().set_pam_sudo(read(PAM_SUDO));
        self.as_mut().set_pam_system_login(read(PAM_SYSTEM_LOGIN));
        self.as_mut().set_pam_plasma_lm(read(PAM_PLASMA_LM));
        self.as_mut().set_pam_polkit(read(PAM_POLKIT));
        self.as_mut().set_pam_hyprlock(read(PAM_HYPRLOCK));
    }

    /// Toggle the howdy line in the given PAM file path:
    ///   off → on : uncomment existing commented line, or insert a new one
    ///   on  → off: comment out the active line
    pub fn toggle_pam(mut self: Pin<&mut Self>, file: QString) {
        let file_path = file.to_string();

        let content = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                // No /etc/pam.d/ override yet — seed from vendor config if available
                let vendor = file_path.replace("/etc/pam.d/", "/usr/lib/pam.d/");
                match fs::read_to_string(&vendor) {
                    Ok(c) => c,
                    Err(_) => {
                        self.as_mut().set_status_message(QString::from(&format!(
                            "No PAM config found for {}",
                            file_path
                        )));
                        return;
                    }
                }
            }
            Err(e) => {
                self.as_mut().set_status_message(QString::from(&format!(
                    "Cannot read {}: {}",
                    file_path, e
                )));
                return;
            }
        };

        let currently_enabled = pam_howdy_active(&content);

        let new_content = if currently_enabled {
            // Disable: comment out every active howdy auth line
            content
                .lines()
                .map(|line| {
                    let t = line.trim();
                    if !t.starts_with('#')
                        && (t.contains("pam_howdy.so") || t.contains("howdy/pam.py"))
                    {
                        format!("#{}", line)
                    } else {
                        line.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("\n")
                + "\n"
        } else if file_path.as_str() == PAM_POLKIT {
            // For polkit: always use the full module path and remove any pre-existing
            // howdy lines (commented or active) to produce a clean, canonical file.
            let mut lines: Vec<String> = content
                .lines()
                .filter(|l| {
                    let t = l.trim();
                    !t.contains("pam_howdy.so") && !t.contains("howdy/pam.py")
                })
                .map(|l| l.to_string())
                .collect();
            let pos = if lines
                .first()
                .map(|l| l.starts_with("#%PAM-1.0"))
                .unwrap_or(false)
            {
                1
            } else {
                0
            };
            lines.insert(pos, pam_module_lines().1.to_string());
            lines.join("\n") + "\n"
        } else {
            let has_commented = content.lines().any(|line| {
                let t = line.trim();
                t.starts_with('#') && (t.contains("pam_howdy.so") || t.contains("howdy/pam.py"))
            });
            if has_commented {
                // Uncomment: strip leading # and optional space
                content
                    .lines()
                    .map(|line| {
                        let t = line.trim();
                        if t.starts_with('#')
                            && (t.contains("pam_howdy.so") || t.contains("howdy/pam.py"))
                        {
                            t.strip_prefix('#').unwrap_or(t).trim_start().to_string()
                        } else {
                            line.to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
                    + "\n"
            } else {
                // Insert after #%PAM-1.0 header if present, otherwise at the top
                let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                let pos = if lines
                    .first()
                    .map(|l| l.starts_with("#%PAM-1.0"))
                    .unwrap_or(false)
                {
                    1
                } else {
                    0
                };
                lines.insert(pos, pam_module_lines().0.to_string());
                lines.join("\n") + "\n"
            }
        };

        let tmp = "/tmp/howdy_pam_tmp";
        if let Err(e) = fs::write(tmp, &new_content) {
            self.as_mut()
                .set_status_message(QString::from(&format!("Failed to write temp file: {}", e)));
            return;
        }

        // polkit needs its systemd service sandbox relaxed so Howdy can open the
        // camera (/dev/video*).  polkit-agent-helper@.service ships with PrivateDevices=yes
        // which hides all devices — compare.py sees an empty /dev and captures 0 frames.
        // NOTE: stock howdy 2.6.x (Arch/CachyOS) has no `howdy set` subcommand, so the
        // timeout tweaks are applied by editing config.ini directly instead of chaining
        // CLI calls (a failing `howdy set` would abort the whole && chain and skip the
        // daemon-reload). The howdy package also already ships a vendor override with
        // PrivateDevices=no, so our /etc drop-in is just belt and suspenders.
        // We do everything in one pkexec bash invocation to avoid multiple auth dialogs.
        if file_path.as_str() == PAM_POLKIT {
            let output = if !currently_enabled {
                let override_content =
                    "[Service]\nPrivateDevices=no\nDeviceAllow=char-video4linux rw\n";
                let _ = fs::write("/tmp/howdy_polkit_override.conf", override_content);
                let script =
                    "cp /tmp/howdy_pam_tmp /etc/pam.d/polkit-1 && \
                     mkdir -p '/etc/systemd/system/polkit-agent-helper@.service.d' && \
                     cp /tmp/howdy_polkit_override.conf \
                        '/etc/systemd/system/polkit-agent-helper@.service.d/override.conf' && \
                     systemctl daemon-reload && \
                     systemctl restart polkit-agent-helper.socket";
                Command::new("pkexec")
                    .args(["/usr/bin/bash", "-c", &script])
                    .output()
            } else {
                let script =
                    "cp /tmp/howdy_pam_tmp /etc/pam.d/polkit-1 && \
                     rm -f '/etc/systemd/system/polkit-agent-helper@.service.d/override.conf' && \
                     systemctl daemon-reload && \
                     systemctl restart polkit-agent-helper.socket";
                Command::new("pkexec")
                    .args(["/usr/bin/bash", "-c", script])
                    .output()
            };
            let _ = fs::remove_file(tmp);
            let _ = fs::remove_file("/tmp/howdy_polkit_override.conf");
            match output {
                Ok(out) if out.status.success() => {
                    self.as_mut().set_pam_polkit(!currently_enabled);
                    let msg = if !currently_enabled {
                        "Howdy enabled for polkit (camera sandbox configured)"
                    } else {
                        "Howdy disabled for polkit"
                    };
                    self.as_mut().set_status_message(QString::from(msg));
                }
                Ok(out) => {
                    let stderr = String::from_utf8_lossy(&out.stderr);
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Failed to configure polkit: {}",
                        stderr
                    )));
                }
                Err(e) => {
                    self.as_mut()
                        .set_status_message(QString::from(&format!("Error: {}", e)));
                }
            }
            return;
        }

        let output = Command::new("pkexec")
            .args(["/usr/bin/cp", tmp, file_path.as_str()])
            .output();
        let _ = fs::remove_file(tmp);

        match output {
            Ok(out) if out.status.success() => {
                let new_state = !currently_enabled;
                let fname = file_path.rsplit('/').next().unwrap_or(&file_path);
                let msg = if new_state {
                    format!("Howdy enabled in {}", fname)
                } else {
                    format!("Howdy disabled in {}", fname)
                };
                match file_path.as_str() {
                    PAM_SDDM => self.as_mut().set_pam_sddm(new_state),
                    PAM_KDE => self.as_mut().set_pam_kde(new_state),
                    PAM_SUDO => self.as_mut().set_pam_sudo(new_state),
                    PAM_HYPRLOCK => self.as_mut().set_pam_hyprlock(new_state),
                    PAM_SYSTEM_LOGIN => self.as_mut().set_pam_system_login(new_state),
                    PAM_PLASMA_LM => self.as_mut().set_pam_plasma_lm(new_state),
                    _ => {}
                }
                self.as_mut().set_status_message(QString::from(&msg));
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                self.as_mut().set_status_message(QString::from(&format!(
                    "Failed to update PAM: {}",
                    stderr
                )));
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error: {}", e)));
            }
        }
    }

    /// Run face recognition test
    pub fn run_test(mut self: Pin<&mut Self>) {
        let howdy = match find_howdy() {
            Some(p) => p,
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Howdy is not installed"));
                return;
            }
        };
        self.as_mut()
            .set_status_message(QString::from("Test running — preview window opens, press any key to close it"));

        let user = target_user();
        // Prefer unprivileged: `howdy test` opens an OpenCV preview window,
        // which needs the user's display. Under pkexec the display env is
        // stripped and Qt crashes (xcb). Models, config and camera are all
        // user-accessible, so root is not required — SUDO_USER is faked purely
        // to satisfy Howdy's "run as root" gate.
        let output = Command::new("/usr/bin/env")
            .arg(format!("SUDO_USER={}", user))
            .arg(&howdy)
            .args(["-U", &user, "test"])
            .output();
        let output = match output {
            Ok(o) if o.status.success() => Ok(o),
            // Fall back to elevated (e.g. root-only model stores on other
            // distros) — the preview may not survive there, but the error
            // text will say why.
            _ => pkexec_howdy(&howdy, &user, &["test"]),
        };

        match output {
            Ok(out) => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let result = if out.status.success() {
                    format!(
                        "Test complete: {}",
                        stdout.lines().last().unwrap_or("Success")
                    )
                } else {
                    format!("Test failed: {}", String::from_utf8_lossy(&out.stderr))
                };
                self.as_mut().set_status_message(QString::from(&result));
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error: {}", e)));
            }
        }
    }

    // ── Setup wizard / Doctor ──

    /// Run all setup preflight checks
    pub fn run_preflight(mut self: Pin<&mut Self>) {
        self.as_mut().set_setup_howdy(find_howdy().is_some());
        self.as_mut()
            .set_setup_pam_python(pam_python_present());
        self.as_mut()
            .set_setup_models(dlib_models_present());
        self.as_mut().set_setup_toolchain(
            ["gcc", "make", "pkgconf", "fakeroot"]
                .iter()
                .all(|p| which_first(&[*p]).is_some()),
        );
        let helper = which_first(&["yay", "paru"]).unwrap_or_default();
        self.as_mut()
            .set_setup_aur_helper(QString::from(&helper));
        self.as_mut()
            .set_setup_agent(polkit_agent_running());
        let mut ir = false;
        for d in list_video_devices() {
            if probe_video_device(&d).likely_ir {
                ir = true;
                break;
            }
        }
        self.as_mut().set_setup_ir_camera(ir);
    }

    /// Probe cameras with format summaries + IR heuristic
    pub fn probe_cameras(mut self: Pin<&mut Self>) {
        let devices = list_video_devices();
        let mut candidates = QList::<QString>::default();
        let mut paths = QList::<QString>::default();
        let mut suggested = String::new();
        let mut first_usable = String::new();
        for d in &devices {
            let probe = probe_video_device(d);
            if first_usable.is_empty() && probe.summary != "no capture formats" {
                first_usable = d.clone();
            }
            if suggested.is_empty() && probe.likely_ir {
                suggested = d.clone();
            }
            let display = if probe.likely_ir {
                format!("{} — {} · likely IR", d, probe.summary)
            } else {
                format!("{} — {}", d, probe.summary)
            };
            candidates.append_clone(&QString::from(display.as_str()));
            paths.append_clone(&QString::from(d.as_str()));
        }
        if suggested.is_empty() {
            suggested = first_usable;
        }
        self.as_mut().set_camera_candidates(candidates);
        self.as_mut().set_camera_paths(paths);
        self.as_mut()
            .set_suggested_camera(QString::from(&suggested));
        self.as_mut().set_setup_ir_camera(!suggested.is_empty());
    }

    /// Install repo packages via pkexec pacman in a thread
    pub fn start_repo_install(mut self: Pin<&mut Self>) {
        self.as_mut().set_install_running(true);
        self.as_mut().set_install_done(false);
        self.as_mut()
            .set_install_error(QString::from(""));
        self.as_mut().set_install_log(QString::from(
            "Installing system packages — authenticate in the popup…\n",
        ));
        std::thread::spawn(move || {
            let _ = fs::write(
                "/tmp/facekey_install.log",
                "FaceKey system-package install\n",
            );
            let pkgs = REPO_PACKAGES.join(" ");
            let script = format!(
                "pacman -S --needed --noconfirm {} >> /tmp/facekey_install.log 2>&1; code=$?; chmod 644 /tmp/facekey_install.log; echo $code > /tmp/facekey_install_done; exit $code",
                pkgs
            );
            let _ = Command::new("pkexec")
                .args(["/usr/bin/bash", "-c", &script])
                .output();
        });
    }

    /// Stream install log; true when the background install finished
    pub fn poll_install_log(mut self: Pin<&mut Self>) -> bool {
        if let Ok(content) = fs::read_to_string("/tmp/facekey_install.log") {
            let lines: Vec<&str> = content.lines().collect();
            let tail = if lines.len() > 80 {
                &lines[lines.len() - 80..]
            } else {
                &lines[..]
            };
            self.as_mut()
                .set_install_log(QString::from(&tail.join("\n")));
        }
        if let Ok(code) = fs::read_to_string("/tmp/facekey_install_done") {
            let _ = fs::remove_file("/tmp/facekey_install_done");
            self.as_mut().set_install_running(false);
            self.as_mut().set_install_done(true);
            if code.trim() != "0" {
                self.as_mut().set_install_error(QString::from(
                    "Package install failed — see log above",
                ));
            }
            return true;
        }
        false
    }

    /// Open a user terminal running the AUR install (yay refuses root,
    /// so this runs unelevated and yay asks for sudo itself)
    pub fn launch_aur_install(mut self: Pin<&mut Self>) {
        let helper = match which_first(&["yay", "paru"]) {
            Some(h) => h,
            None => {
                self.as_mut().set_install_error(QString::from(
                    "No AUR helper found (install yay or paru first)",
                ));
                return;
            }
        };
        let helper_base = helper.rsplit('/').next().unwrap_or(&helper).to_string();
        let term = match which_first(&[
            "foot",
            "kitty",
            "konsole",
            "gnome-terminal",
            "xterm",
        ]) {
            Some(t) => t,
            None => {
                self.as_mut().set_install_error(QString::from(
                    "No terminal emulator found",
                ));
                return;
            }
        };
        // NOTE: python-dlib compiles from source here — honest 20-60 min
        // on fresh machines. The terminal stays open so the log is visible.
        let script = format!(
            "echo 'FaceKey: installing howdy + pam-python (dlib compile takes a while, leave it running)'; {} -S --needed --noconfirm howdy pam-python; echo; echo '--- FaceKey: close this window when finished, then press Continue ---'; read _",
            helper_base
        );
        let mut cmd = Command::new(&term);
        let term_base = term.rsplit('/').next().unwrap_or(&term);
        if term_base == "konsole" || term_base == "xterm" {
            cmd.args(["-e", "bash", "-c", &script]);
        } else if term_base == "gnome-terminal" {
            cmd.args(["--", "bash", "-c", &script]);
        } else {
            // foot, kitty: command directly
            cmd.args(["bash", "-c", &script]);
        }
        match cmd.spawn() {
            Ok(_) => {
                self.as_mut().set_install_error(QString::from(""));
            }
            Err(e) => {
                self.as_mut().set_install_error(QString::from(&format!(
                    "Could not open terminal: {}",
                    e
                )));
            }
        }
    }

    /// True when howdy + pam-python are both present (refreshes preflight)
    pub fn check_install_done(mut self: Pin<&mut Self>) -> bool {
        let done = find_howdy().is_some() && pam_python_present();
        if done {
            self.as_mut().set_setup_howdy(true);
            self.as_mut().set_setup_pam_python(true);
            self.as_mut()
                .set_setup_models(dlib_models_present());
        }
        done
    }
}
