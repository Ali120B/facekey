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
        #[qproperty(bool, install_failed)]
        #[qproperty(QString, install_error)]
        // ── Distro backend (task.md Phase 1) ──
        #[qproperty(QString, distro_id)]
        #[qproperty(QString, distro_like)]
        #[qproperty(QString, pkg_manager)]
        #[qproperty(bool, is_debian)]
        #[qproperty(QString, pam_module_kind)]
        #[qproperty(QString, desktop_id)]
        // ── Auth ordering: face-first vs password-first ──
        #[qproperty(QString, auth_order)]
        // ── Multi-user: which account Howdy commands target ──
        #[qproperty(QList_QString, login_users)]
        #[qproperty(QString, face_user)]
        // ── Self-update check ──
        #[qproperty(bool, update_available)]
        #[qproperty(QString, latest_version)]
        // ── Recognition tuning (Howdy config) ──
        #[qproperty(i32, tune_timeout)]
        #[qproperty(f64, tune_certainty)]
        #[qproperty(f64, tune_dark_threshold)]
        // ── Attempt snapshots gallery ──
        #[qproperty(QList_QString, snapshots)]
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

        /// True when running with --test-run (all responses simulated)
        #[qinvokable]
        fn is_test_run(self: Pin<&mut HowdyBackend>) -> bool;

        /// Detect the distro family for the installer backend
        /// (sets distro_id/distro_like/pkg_manager)
        #[qinvokable]
        fn detect_distro_info(self: Pin<&mut HowdyBackend>);

        /// List human login users (uid 1000–60000) and default face_user
        /// to the desktop user
        #[qinvokable]
        fn list_login_users(self: Pin<&mut HowdyBackend>);

        /// Check GitHub for a newer release (threaded, silent on failure)
        #[qinvokable]
        fn check_for_updates(self: Pin<&mut HowdyBackend>);

        /// Download the latest AppImage into ~/Downloads (threaded)
        #[qinvokable]
        fn download_update(self: Pin<&mut HowdyBackend>);

        /// Pick up a finished update check/download (GUI thread, poll me)
        #[qinvokable]
        fn poll_update(self: Pin<&mut HowdyBackend>) -> bool;

        /// Auth order preference: "face-first" (default) or "password-first".
        /// Face-first inserts the Howdy line before the system includes so a
        /// face is always attempted; password-first appends it at the end so
        /// a correct password sails through and face only triggers on an
        /// empty/failed password. Persists to ~/.config/facekey/auth_order.
        /// (Named apply_* because the auth_order qproperty owns set_auth_order.)
        #[qinvokable]
        fn apply_auth_order(self: Pin<&mut HowdyBackend>, mode: QString);

        /// Load the persisted auth order (call at startup)
        #[qinvokable]
        fn load_auth_order(self: Pin<&mut HowdyBackend>);

        /// Load recognition tuning values from howdy config.ini
        #[qinvokable]
        fn load_tuning(self: Pin<&mut HowdyBackend>);

        /// Save recognition tuning values (pkexec, staged + backed up)
        #[qinvokable]
        fn save_tuning(self: Pin<&mut HowdyBackend>, timeout: i32, certainty: f64, dark: f64);

        /// Refresh the attempt-snapshots list (newest first, file:// URLs)
        #[qinvokable]
        fn refresh_snapshots(self: Pin<&mut HowdyBackend>);

        /// Install + autostart a polkit agent and start one now.
        /// Picks hyprpolkitagent on Hyprland, else GNOME/KDE agent.
        #[qinvokable]
        fn fix_polkit_agent(self: Pin<&mut HowdyBackend>);
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
    install_failed: bool,
    install_error: QString,
    distro_id: QString,
    distro_like: QString,
    pkg_manager: QString,
    is_debian: bool,
    pam_module_kind: QString,
    desktop_id: QString,
    tune_timeout: i32,
    tune_certainty: f64,
    tune_dark_threshold: f64,
    snapshots: QList<QString>,
    auth_order: QString,
    login_users: QList<QString>,
    face_user: QString,
    update_available: bool,
    latest_version: QString,
}

const PAM_LINE_DEBIAN: &str = "auth sufficient pam_howdy.so";
const PAM_LINE_ARCH: &str = "auth sufficient pam_python.so /lib/security/howdy/pam.py";
// Kept for reference; use pam_module_lines() instead of these directly.
#[allow(dead_code)]
const PAM_LINE: &str = PAM_LINE_DEBIAN;
const PAM_SDDM: &str = "/etc/pam.d/sddm";
const PAM_KDE: &str = "/etc/pam.d/kde";
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

// ── Enrollment result channel (findings 5+8) ──
// Results travel in process memory, tagged with the request sequence that
// produced them. Stale completions (timeout abandon, second instance) and
// forged /tmp files can never be consumed as the active result.
struct AddResult {
    seq: u64,
    message: String,
    face_id: Option<i32>,
}

struct PendingAdd {
    seq: u64,
    rx: std::sync::mpsc::Receiver<AddResult>,
}

/// (next sequence, in-flight request, face id of last success)
static ADD_STATE: std::sync::Mutex<(u64, Option<PendingAdd>, Option<i32>)> =
    std::sync::Mutex::new((0, None, None));

/// Model ids parsed from `howdy list` output
fn parse_model_ids(output: &str) -> Vec<i32> {
    output
        .lines()
        .filter_map(|l| {
            let first = l.trim().split_whitespace().next()?;
            let c = first.chars().next()?;
            if c.is_ascii_digit() {
                first.parse::<i32>().ok()
            } else {
                None
            }
        })
        .collect()
}

fn highest_model_id(howdy: &str, user: &str) -> Option<i32> {
    let out = pkexec_howdy(howdy, user, &["list"]).ok()?;
    if !out.status.success() {
        return None;
    }
    parse_model_ids(&String::from_utf8_lossy(&out.stdout))
        .into_iter()
        .max()
}

/// Dry-run mode for UI walkthroughs (`facekey --test-run`): every
/// mutating/external call short-circuits to a canned response so no pkexec,
/// no howdy and no system change ever happens. QML can query it too.
fn test_run() -> bool {
    std::env::var("FACEKEY_TEST_RUN")
        .map(|v| v == "1")
        .unwrap_or(false)
}

/// Face user selected in the UI, falling back to the desktop user when
/// nothing (valid) is selected yet.
fn selected_user(backend: &qobject::HowdyBackend) -> String {
    let u = backend.face_user().to_string();
    if u.trim().is_empty() {
        target_user()
    } else {
        u.trim().to_string()
    }
}

/// Compare dotted versions ("1.9.0" vs "v4.0.0"). Returns true when
/// `latest` is newer than `current`.
fn version_newer(current: &str, latest: &str) -> bool {
    fn parts(v: &str) -> Vec<u64> {
        v.trim()
            .trim_start_matches('v')
            .split('.')
            .map(|p| {
                p.chars()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap_or(0)
            })
            .collect()
    }
    let (mut a, mut b) = (parts(current), parts(latest));
    let n = a.len().max(b.len());
    a.resize(n, 0);
    b.resize(n, 0);
    a < b
}

/// Latest release via the /releases/latest redirect plus the deterministic
/// AppImage asset URL. Deliberately API-free: no token, no rate limits, no
/// JSON parsing (and the API 404s anonymously on this repo anyway).
/// Returns (tag, asset_url).
fn github_latest() -> Option<(String, String)> {
    let out = Command::new("curl")
        .args([
            "-Ls",
            "--max-time",
            "15",
            "-o",
            "/dev/null",
            "-w",
            "%{url_effective}",
            "https://github.com/Ali120B/facekey/releases/latest",
        ])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let url = String::from_utf8_lossy(&out.stdout).trim().to_string();
    let tag = url.rsplit("/tag/").next()?.to_string();
    let sane = tag.len() > 1
        && tag.starts_with('v')
        && tag[1..].chars().all(|c| c.is_ascii_digit() || c == '.');
    if !sane {
        return None;
    }
    let asset = format!(
        "https://github.com/Ali120B/facekey/releases/download/{}/facekey-{}-x86_64.AppImage",
        tag, tag
    );
    Some((tag, asset))
}

/// Only release assets from our own repo may be downloaded.
fn update_url_ok(url: &str) -> bool {
    (url.starts_with("https://github.com/Ali120B/facekey/releases/download/")
        || url.starts_with("https://objects.githubusercontent.com/"))
        && url.ends_with(".AppImage")
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
const REPO_PACKAGES_ARCH: &[&str] = &[
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

fn pam_howdy_present() -> bool {
    Path::new("/usr/lib/security/pam_howdy.so").exists()
        || Path::new("/lib/security/pam_howdy.so").exists()
}

/// Which PAM module flavor is installed, if any. Arch ships pam_python;
/// Debian-family ships the compiled pam_howdy.so. Empty = none.
fn pam_module_kind() -> &'static str {
    if pam_python_present() {
        "pam_python"
    } else if pam_howdy_present() {
        "pam_howdy"
    } else {
        ""
    }
}

fn dlib_models_present() -> bool {
    DLIB_MODELS
        .iter()
        .all(|f| Path::new(&format!("/usr/lib/security/howdy/dlib-data/{}", f)).exists())
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

// ── Privileged file staging (finding 2) ──
// Predictable /tmp names let another local user swap content between our
// write and the pkexec copy. Instead every staged file goes into a 0700
// per-user dir (root can still read it for the copy) under a unique
// create_new name, so nothing can be pre-planted or swapped.

/// Private staging dir for files a later pkexec call consumes
fn staging_dir() -> Option<std::path::PathBuf> {
    let mut user: String = target_user()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
        .collect();
    if user.is_empty() {
        user = "user".to_string();
    }
    let dir = std::env::temp_dir().join(format!("facekey-{}", user));
    std::fs::create_dir_all(&dir).ok()?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700));
    }
    Some(dir)
}

/// Write content to a uniquely-named 0600 file in the staging dir.
/// Never overwrites: create_new fails on collision and we retry.
fn stage_file(stem: &str, content: &str) -> Option<std::path::PathBuf> {
    let dir = staging_dir()?;
    for i in 0..100 {
        let path = dir.join(format!("{}-{}-{}.tmp", stem, std::process::id(), i));
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut f) => {
                use std::io::Write;
                if f.write_all(content.as_bytes()).is_err() {
                    let _ = std::fs::remove_file(&path);
                    return None;
                }
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
                }
                return Some(path);
            }
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(_) => return None,
        }
    }
    None
}

// ── Config backups (finding 9) ──
// The original bytes of every file we rewrite through pkexec are kept in
// ~/.local/share/facekey/backups/<name>.<unixts>.bak so a bad edit is a
// single `sudo cp` away from recovery.

/// Back up user-readable original content before a privileged rewrite.
/// Returns the backup path for the status message, if it worked.
fn backup_file(original_path: &str, content: &str) -> Option<String> {
    let home = std::env::var("HOME").ok()?;
    let dir = Path::new(&home).join(".local/share/facekey/backups");
    std::fs::create_dir_all(&dir).ok()?;
    let base = original_path.rsplit('/').next().unwrap_or("file");
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let dest = dir.join(format!("{}.{}.bak", base, ts));
    std::fs::write(&dest, content).ok()?;
    Some(dest.to_string_lossy().to_string())
}

/// PAM files FaceKey is allowed to manage (finding 3). The QML-exposed
/// toggle takes a path, but anything outside this list is refused — a
/// compromised or future caller cannot turn the invokable into an
/// arbitrary privileged file writer.
fn pam_managed(path: &str) -> bool {
    [
        PAM_SDDM,
        PAM_KDE,
        PAM_SUDO,
        PAM_SYSTEM_LOGIN,
        PAM_PLASMA_LM,
        PAM_POLKIT,
        PAM_HYPRLOCK,
    ]
    .contains(&path)
}

/// True only for existing regular files (rejects symlinks, dirs, …)
fn is_plain_file(path: &str) -> bool {
    std::fs::symlink_metadata(path)
        .map(|m| m.is_file() && !m.file_type().is_symlink())
        .unwrap_or(false)
}

/// Distro identity parsed from /etc/os-release
struct DistroInfo {
    id: String,
    like: String,
    pkg_manager: &'static str,
}

impl DistroInfo {
    /// "arch" for pacman systems, "debian" for apt systems, else "unknown".
    /// Phase 2+: installer backend branches on this, never on raw ids.
    fn family(&self) -> &'static str {
        match self.pkg_manager {
            "pacman" => "arch",
            "apt" => "debian",
            _ => "unknown",
        }
    }

    fn is_debian(&self) -> bool {
        self.family() == "debian"
    }
}

/// Parse /etc/os-release and pick the package manager family.
/// Arch-based (incl. CachyOS) → pacman; Debian-family (debian, ubuntu,
/// pop, linuxmint, …) → apt; anything else → unknown. Pure detection —
/// no behavior branches on it yet (Phase 2 wires the apt backend).
fn detect_distro() -> DistroInfo {
    let mut id = String::new();
    let mut like = String::new();
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            let line = line.trim();
            if let Some(v) = line.strip_prefix("ID=") {
                id = v.trim().trim_matches('"').to_lowercase();
            } else if let Some(v) = line.strip_prefix("ID_LIKE=") {
                like = v.trim().trim_matches('"').to_lowercase();
            }
        }
    }
    // Fall back to uname-based guess when os-release is missing
    if id.is_empty() {
        if which_first(&["pacman"]).is_some() {
            id = "arch".to_string();
        } else if which_first(&["apt-get", "apt"]).is_some() {
            id = "debian".to_string();
        }
    }
    let haystack = format!("{} {}", id, like);
    let pkg_manager = if haystack.split_whitespace().any(|w| w == "arch") {
        "pacman"
    } else if [
        "debian",
        "ubuntu",
        "pop",
        "linuxmint",
        "zorin",
        "elementary",
    ]
    .iter()
    .any(|w| haystack.split_whitespace().any(|t| t == *w))
    {
        "apt"
    } else {
        "unknown"
    };
    DistroInfo {
        id,
        like,
        pkg_manager,
    }
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
    let likely_ir = sizes.iter().any(|(w, h)| w == h && *w <= 480 && *w >= 100);
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

        // Neither detected → fall back to showing SDDM (most common on Arch).
        // On Debian-family the GDM row below is the one that matters.
        if !sddm && !plm {
            self.as_mut().set_sddm_installed(true);
        }

        // Screen locker actually in use on Hyprland setups
        let hyprlock = Path::new("/usr/bin/hyprlock").exists() || pacman_installed("hyprlock");
        self.as_mut().set_hyprlock_installed(hyprlock);
    }

    /// Check if device has a supported IR camera for howdy
    pub fn check_device(mut self: Pin<&mut Self>) {
        if test_run() {
            self.as_mut().set_device_supported(true);
            self.as_mut().set_howdy_enabled(true);
            self.as_mut()
                .set_status_message(QString::from("Test mode: simulated IR camera ready"));
            return;
        }
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
        if test_run() {
            let mut models = QList::<QString>::default();
            models.append_clone(&QString::from("0  2026-01-01 12:00  Demo Face"));
            models.append_clone(&QString::from("1  2026-01-02 12:00  Glasses"));
            self.as_mut().set_face_models(models);
            self.as_mut()
                .set_status_message(QString::from("Found 2 registered face(s) (test mode)"));
            return;
        }
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
        let user = selected_user(&self);
        let output = Command::new(&howdy)
            .args(["-U", &user, "list"])
            .output()
            .ok()
            .filter(|o| o.status.success())
            .or_else(|| {
                let user = selected_user(&self);
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

        // Per-request channel, set up before any branch so test and real
        // flows share the same completion plumbing.
        let seq = {
            let mut st = ADD_STATE.lock().unwrap();
            st.0 = st.0.wrapping_add(1);
            st.0
        };
        let (tx, rx) = std::sync::mpsc::channel();
        ADD_STATE.lock().unwrap().1 = Some(PendingAdd { seq, rx });

        if test_run() {
            self.as_mut()
                .set_status_message(QString::from("Look at the camera..."));
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let _ = tx.send(AddResult {
                    seq,
                    message: "Face registered successfully".to_string(),
                    face_id: None,
                });
            });
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

        // Spawn a thread to run the blocking pkexec calls.
        // NOTE: stock howdy ignores any name argument when -y is passed and
        // always labels the model "Model #N", so the requested name is applied
        // afterwards by patching the fresh entry in the JSON model store.
        // No shell is used anywhere here (finding 1): user, label and paths
        // travel as argv, never interpolated into a command string.
        let howdy_clone = howdy.clone();
        let label: String = name_str.chars().take(24).collect();
        let user_clone = {
            let u = self.as_ref().face_user().to_string();
            if u.trim().is_empty() {
                target_user()
            } else {
                u.trim().to_string()
            }
        };

        std::thread::spawn(move || {
            // Step 1: capture. Howdy prints no usable id, so face_id stays None
            // unless a future version reports one.
            let add_out = pkexec_howdy(&howdy_clone, &user_clone, &["add", "-y"]);
            let mut message;
            let mut face_id = None;
            let mut captured = false;
            match add_out {
                Ok(out) if out.status.success() => {
                    let stdout = String::from_utf8_lossy(&out.stdout);
                    face_id = stdout
                        .lines()
                        .find(|l| l.trim().starts_with("Face added as "))
                        .and_then(|l| {
                            l.trim()
                                .split_whitespace()
                                .last()
                                .and_then(|s| s.parse::<i32>().ok())
                        });
                    captured = true;
                    message = "Face registered successfully".to_string();
                }
                Ok(out) => {
                    let combined = format!(
                        "{}{}",
                        String::from_utf8_lossy(&out.stdout),
                        String::from_utf8_lossy(&out.stderr)
                    )
                    .trim()
                    .to_string();
                    message = if combined.is_empty() {
                        format!("Failed (exit code: {:?})", out.status.code())
                    } else {
                        format!("Failed: {}", combined)
                    };
                }
                Err(e) => {
                    message = format!("Failed: {}", e);
                }
            }

            // Step 2: apply the requested label by argv (never shell).
            // A second auth dialog may appear if the kept authorization lapsed.
            if captured {
                let model_dat = format!("/lib/security/howdy/models/{}.dat", user_clone);
                let patch = "import json,sys; p=sys.argv[1]; m=json.load(open(p)); m[-1]['label']=sys.argv[2][:24]; json.dump(m, open(p, 'w'))";
                let patch_args = [
                    "/usr/bin/python3".to_string(),
                    "-c".to_string(),
                    patch.to_string(),
                    model_dat.clone(),
                    label.clone(),
                ];
                let patch_out = Command::new("pkexec").args(&patch_args).output();
                match patch_out {
                    Ok(o) if o.status.success() => {}
                    Ok(o) => {
                        let err = String::from_utf8_lossy(&o.stderr).trim().to_string();
                        message = format!(
                            "Face captured but the label could not be applied{}",
                            if err.is_empty() {
                                String::new()
                            } else {
                                format!(": {}", err)
                            }
                        );
                    }
                    Err(e) => {
                        message = format!("Face captured but labeling failed: {}", e);
                    }
                }
            }

            let _ = tx.send(AddResult {
                seq,
                message,
                face_id,
            });
        });
    }

    /// Check for a completed add_model (called periodically from QML).
    /// Only the active request's result is accepted; stale orphans are dropped.
    /// Returns true if a result was found and processed
    pub fn check_add_result(mut self: Pin<&mut Self>) -> bool {
        let pending = ADD_STATE.lock().unwrap().1.take();
        let Some(p) = pending else {
            return false;
        };
        match p.rx.try_recv() {
            Ok(res) if res.seq == p.seq => {
                ADD_STATE.lock().unwrap().2 = res.face_id;
                self.as_mut()
                    .set_status_message(QString::from(&res.message));
                true
            }
            Ok(_) => false, // stale orphan from an abandoned attempt
            Err(std::sync::mpsc::TryRecvError::Empty) => {
                ADD_STATE.lock().unwrap().1 = Some(p);
                false
            }
            Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                self.as_mut()
                    .set_status_message(QString::from("Enrollment process died unexpectedly"));
                true
            }
        }
    }

    /// Discard the just-registered face (delete it)
    pub fn discard_face(mut self: Pin<&mut Self>) {
        if test_run() {
            self.as_mut()
                .set_status_message(QString::from("Face discarded (test mode)"));
            return;
        }
        let howdy = match find_howdy() {
            Some(p) => p,
            None => return,
        };
        let user = selected_user(&self);

        // Prefer the exact id from the last successful enrollment,
        // otherwise fall back to the highest known model id.
        let id = ADD_STATE
            .lock()
            .unwrap()
            .2
            .or_else(|| highest_model_id(&howdy, &user));
        match id {
            Some(id) => {
                let _ = pkexec_howdy(&howdy, &user, &["remove", "-y", &id.to_string()]);
                ADD_STATE.lock().unwrap().2 = None;
                self.as_mut()
                    .set_status_message(QString::from("Face discarded"));
            }
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Nothing to discard"));
            }
        }
    }

    /// Remove a face model by its ID
    pub fn remove_model(mut self: Pin<&mut Self>, index: i32) {
        if test_run() {
            let _ = index;
            self.as_mut()
                .set_status_message(QString::from("Model removed (test mode)"));
            self.refresh_models();
            return;
        }
        let howdy = match find_howdy() {
            Some(p) => p,
            None => {
                self.as_mut()
                    .set_status_message(QString::from("Howdy is not installed"));
                return;
            }
        };
        let user = selected_user(&self);
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
        if test_run() {
            let current = *self.as_ref().howdy_enabled();
            self.as_mut().set_howdy_enabled(!current);
            self.as_mut().set_status_message(QString::from(if current {
                "Howdy disabled (test mode)"
            } else {
                "Howdy enabled (test mode)"
            }));
            return;
        }
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
        if test_run() {
            let mut devices = QList::<QString>::default();
            devices.append_clone(&QString::from("/dev/video0"));
            devices.append_clone(&QString::from("/dev/video2"));
            self.as_mut().set_video_devices(devices);
            self.as_mut().set_camera_configured(true);
            return;
        }
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
                            let device = value.trim().trim_matches('"');
                            // Finding 7 (residue): a path that does not exist
                            // is not a configuration — keep the wizard honest.
                            if !device.is_empty()
                                && device != "none"
                                && device != "null"
                                && Path::new(device).exists()
                            {
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
        if test_run() {
            self.as_mut().set_status_message(QString::from(&format!(
                "Previewing {} (test mode — player not launched)",
                device_str
            )));
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

        if test_run() {
            self.as_mut().set_camera_configured(true);
            self.as_mut().set_status_message(QString::from(&format!(
                "Camera saved: {} (test mode)",
                device_str
            )));
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

        // Stage in the private dir, then copy it with elevated privileges.
        // A timestamped backup of the original is kept for recovery.
        let Some(staged) = stage_file("howdy-config", &new_content) else {
            self.as_mut()
                .set_status_message(QString::from("Failed to stage config file"));
            return;
        };
        backup_file(config_path, &current_content);

        let output = Command::new("pkexec")
            .args(["/usr/bin/cp"])
            .arg(&staged)
            .arg(config_path)
            .output();
        let _ = fs::remove_file(&staged);

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

        if test_run() {
            let fname = file_path.rsplit('/').next().unwrap_or(&file_path);
            let on = match file_path.as_str() {
                PAM_SDDM => {
                    let cur = *self.as_ref().pam_sddm();
                    self.as_mut().set_pam_sddm(!cur);
                    !cur
                }
                PAM_KDE => {
                    let cur = *self.as_ref().pam_kde();
                    self.as_mut().set_pam_kde(!cur);
                    !cur
                }
                PAM_SUDO => {
                    let cur = *self.as_ref().pam_sudo();
                    self.as_mut().set_pam_sudo(!cur);
                    !cur
                }
                PAM_SYSTEM_LOGIN => {
                    let cur = *self.as_ref().pam_system_login();
                    self.as_mut().set_pam_system_login(!cur);
                    !cur
                }
                PAM_PLASMA_LM => {
                    let cur = *self.as_ref().pam_plasma_lm();
                    self.as_mut().set_pam_plasma_lm(!cur);
                    !cur
                }
                PAM_HYPRLOCK => {
                    let cur = *self.as_ref().pam_hyprlock();
                    self.as_mut().set_pam_hyprlock(!cur);
                    !cur
                }
                _ if file_path.as_str() == PAM_POLKIT => {
                    let cur = *self.as_ref().pam_polkit();
                    self.as_mut().set_pam_polkit(!cur);
                    !cur
                }
                _ => {
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Unknown PAM file {} (test mode)",
                        fname
                    )));
                    return;
                }
            };
            self.as_mut().set_status_message(QString::from(&format!(
                "Howdy {} in {} (test mode)",
                if on { "enabled" } else { "disabled" },
                fname
            )));
            return;
        }
        // Finding 3: only managed PAM files, and only plain regular files.
        // The QML layer passes constants today, but the invokable must not
        // be an arbitrary privileged file writer for any future caller.
        if !pam_managed(&file_path) {
            let fname = file_path.rsplit('/').next().unwrap_or(&file_path);
            self.as_mut().set_status_message(QString::from(&format!(
                "Refusing to modify unmanaged PAM file {}",
                fname
            )));
            return;
        }
        if Path::new(&file_path).exists() && !is_plain_file(&file_path) {
            self.as_mut().set_status_message(QString::from(&format!(
                "Refusing to modify non-regular file {}",
                file_path
            )));
            return;
        }
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
            // Password-first mode appends at the end instead (same semantics
            // as the generic path above).
            let mut lines: Vec<String> = content
                .lines()
                .filter(|l| {
                    let t = l.trim();
                    !t.contains("pam_howdy.so") && !t.contains("howdy/pam.py")
                })
                .map(|l| l.to_string())
                .collect();
            if Self::read_auth_order() == "password-first" {
                lines.push(pam_module_lines().1.to_string());
            } else {
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
            }
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
                // Face-first (default): insert after #%PAM-1.0 header so the
                // face is always attempted before the password stack.
                // Password-first: append at the end, after the system
                // includes — a correct password succeeds first and Howdy
                // only runs on empty/failed passwords.
                let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
                if Self::read_auth_order() == "password-first" {
                    lines.push(pam_module_lines().0.to_string());
                } else {
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
                }
                lines.join("\n") + "\n"
            }
        };

        // Finding 9: keep a timestamped backup of the original first so a
        // bad edit is one `sudo cp` away from recovery.
        let backup_msg = if Path::new(&file_path).exists() {
            backup_file(&file_path, &content)
                .map(|b| format!(" (backup: {})", b))
                .unwrap_or_default()
        } else {
            String::new()
        };

        let Some(staged) = stage_file("howdy-pam", &new_content) else {
            self.as_mut()
                .set_status_message(QString::from("Failed to stage PAM file"));
            return;
        };
        let staged_str = staged.to_string_lossy().to_string();

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
                let Some(override_staged) = stage_file("howdy-polkit-override", override_content)
                else {
                    let _ = fs::remove_file(&staged);
                    self.as_mut()
                        .set_status_message(QString::from("Failed to stage polkit override"));
                    return;
                };
                let script = format!(
                    "cp '{}' /etc/pam.d/polkit-1 && \
                     mkdir -p '/etc/systemd/system/polkit-agent-helper@.service.d' && \
                     cp '{}' \
                        '/etc/systemd/system/polkit-agent-helper@.service.d/override.conf' && \
                     systemctl daemon-reload && \
                     systemctl restart polkit-agent-helper.socket",
                    staged_str,
                    override_staged.to_string_lossy()
                );
                let out = Command::new("pkexec")
                    .args(["/usr/bin/bash", "-c", &script])
                    .output();
                let _ = fs::remove_file(&override_staged);
                out
            } else {
                let script = format!(
                    "cp '{}' /etc/pam.d/polkit-1 && \
                     rm -f '/etc/systemd/system/polkit-agent-helper@.service.d/override.conf' && \
                     systemctl daemon-reload && \
                     systemctl restart polkit-agent-helper.socket",
                    staged_str
                );
                Command::new("pkexec")
                    .args(["/usr/bin/bash", "-c", &script])
                    .output()
            };
            let _ = fs::remove_file(&staged);
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
            .args(["/usr/bin/cp"])
            .arg(&staged)
            .arg(file_path.as_str())
            .output();
        let _ = fs::remove_file(&staged);

        match output {
            Ok(out) if out.status.success() => {
                let new_state = !currently_enabled;
                let fname = file_path.rsplit('/').next().unwrap_or(&file_path);
                let msg = if new_state {
                    format!("Howdy enabled in {}{}", fname, backup_msg)
                } else {
                    format!("Howdy disabled in {}{}", fname, backup_msg)
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
        self.as_mut().set_status_message(QString::from(
            "Test running — preview window opens, press any key to close it",
        ));

        if test_run() {
            self.as_mut()
                .set_status_message(QString::from("Test complete (test mode): face recognized"));
            return;
        }

        let user = selected_user(&self);
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

    /// Whether --test-run dry-run mode is active (for QML bindings)
    pub fn is_test_run(self: Pin<&mut Self>) -> bool {
        let _ = self;
        test_run()
    }

    /// Detect the distro family for the installer backend.
    /// Read-only (/etc/os-release); Arch behavior is unchanged.
    pub fn detect_distro_info(mut self: Pin<&mut Self>) {
        let d = detect_distro();
        self.as_mut().set_distro_id(QString::from(&d.id));
        self.as_mut().set_distro_like(QString::from(&d.like));
        self.as_mut().set_pkg_manager(QString::from(d.pkg_manager));
        self.as_mut().set_is_debian(d.is_debian());
        // Desktop session id (COSMIC/GNOME/KDE/…) for environment hints.
        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_else(|_| "unknown".into());
        self.as_mut().set_desktop_id(QString::from(&desktop));
    }

    /// Check GitHub for a newer release (threaded; result via poll_update).
    /// Writes /tmp/facekey_update_check.json: {"tag":…,"url":…} or "NONE".
    pub fn check_for_updates(mut self: Pin<&mut Self>) {
        if test_run() {
            self.as_mut().set_update_available(false);
            self.as_mut().set_latest_version(QString::from(""));
            let _ = std::fs::write("/tmp/facekey_update_check.json", "NONE");
            return;
        }
        std::thread::spawn(|| {
            let payload = match github_latest() {
                Some((tag, url)) => format!("{{\"tag\":\"{}\",\"url\":\"{}\"}}", tag, url),
                None => "NONE".to_string(),
            };
            let _ = std::fs::write("/tmp/facekey_update_check.json", payload);
        });
    }

    /// Pick up a finished update check or download (GUI thread).
    /// Returns true when something was consumed.
    pub fn poll_update(mut self: Pin<&mut Self>) -> bool {
        if let Ok(content) = std::fs::read_to_string("/tmp/facekey_update_check.json") {
            let _ = std::fs::remove_file("/tmp/facekey_update_check.json");
            let content = content.trim().to_string();
            if content != "NONE" {
                let tag = content
                    .split("\"tag\":\"")
                    .nth(1)
                    .and_then(|s| s.split('"').next())
                    .unwrap_or("")
                    .to_string();
                if !tag.is_empty() && version_newer(env!("CARGO_PKG_VERSION"), &tag) {
                    self.as_mut().set_latest_version(QString::from(&tag));
                    self.as_mut().set_update_available(true);
                    self.as_mut().set_status_message(QString::from(&format!(
                        "Update available: {} (see header)",
                        tag
                    )));
                }
            }
            return true;
        }
        if let Ok(msg) = std::fs::read_to_string("/tmp/facekey_update_dl.txt") {
            let _ = std::fs::remove_file("/tmp/facekey_update_dl.txt");
            self.as_mut().set_status_message(QString::from(&msg));
            return true;
        }
        false
    }

    /// Download the latest AppImage into ~/Downloads (threaded).
    /// The URL is re-validated (github.com/Ali120B/facekey/releases only)
    /// before curl ever sees it.
    pub fn download_update(mut self: Pin<&mut Self>) {
        if test_run() {
            self.as_mut()
                .set_status_message(QString::from("Update downloaded (test mode)"));
            return;
        }
        self.as_mut()
            .set_status_message(QString::from("Downloading update — watch ~/Downloads…"));
        std::thread::spawn(|| {
            let msg = match github_latest() {
                Some((tag, url)) if update_url_ok(&url) => {
                    let name = url.rsplit('/').next().unwrap_or("facekey.AppImage");
                    let dest = match std::env::var("HOME") {
                        Ok(h) => format!("{}/Downloads/{}", h, name),
                        Err(_) => format!("/tmp/{}", name),
                    };
                    match Command::new("curl")
                        .args(["-L", "--max-time", "600", "-o", &dest, &url])
                        .output()
                    {
                        Ok(o) if o.status.success() => {
                            let _ = Command::new("chmod").args(["+x", &dest]).output();
                            format!("Update {} downloaded — restart the app to use it", tag)
                        }
                        _ => "Update download failed — check network".to_string(),
                    }
                }
                Some(_) => "Update URL failed validation — aborted".to_string(),
                None => "Could not reach GitHub releases".to_string(),
            };
            let _ = std::fs::write("/tmp/facekey_update_dl.txt", msg);
        });
    }

    /// List human login users (uid 1000–60000) and default face_user
    /// to the desktop user
    pub fn list_login_users(mut self: Pin<&mut Self>) {
        if test_run() {
            let mut users = QList::<QString>::default();
            users.append_clone(&QString::from("testuser"));
            self.as_mut().set_login_users(users);
            self.as_mut().set_face_user(QString::from("testuser"));
            return;
        }
        let mut users: Vec<String> = Vec::new();
        if let Ok(passwd) = fs::read_to_string("/etc/passwd") {
            for line in passwd.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() > 2 {
                    if let Ok(uid) = parts[2].parse::<u32>() {
                        if (1000..60000).contains(&uid) {
                            users.push(parts[0].to_string());
                        }
                    }
                }
            }
        }
        users.sort();
        users.dedup();
        if users.is_empty() {
            users.push(target_user());
        }
        let mut qusers = QList::<QString>::default();
        for u in &users {
            qusers.append_clone(&QString::from(u.as_str()));
        }
        self.as_mut().set_login_users(qusers);
        let cur = selected_user(&self);
        if !users.iter().any(|u| u == &cur) {
            let me = target_user();
            let def = if users.iter().any(|u| u == &me) {
                me
            } else {
                users[0].clone()
            };
            self.as_mut().set_face_user(QString::from(def.as_str()));
        }
    }

    fn auth_order_file() -> Option<std::path::PathBuf> {
        std::env::var("HOME")
            .ok()
            .map(|h| Path::new(&h).join(".config/facekey/auth_order"))
    }

    /// Persisted auth order, defaulting to face-first
    fn read_auth_order() -> String {
        Self::auth_order_file()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .map(|s| s.trim().to_string())
            .filter(|s| s == "password-first" || s == "face-first")
            .unwrap_or_else(|| "face-first".to_string())
    }

    /// Load the persisted auth order (call at startup)
    pub fn load_auth_order(mut self: Pin<&mut Self>) {
        // read_file exists so test-run can observe the default too
        let mode = if test_run() {
            "face-first".to_string()
        } else {
            Self::read_auth_order()
        };
        self.as_mut().set_auth_order(QString::from(&mode));
    }

    /// Set + persist the auth order preference
    pub fn apply_auth_order(mut self: Pin<&mut Self>, mode: QString) {
        let mode = mode.to_string();
        let mode = if mode == "password-first" {
            "password-first"
        } else {
            "face-first"
        };
        if !test_run() {
            if let Some(p) = Self::auth_order_file() {
                if let Some(parent) = p.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&p, mode);
            }
        }
        self.as_mut().set_auth_order(QString::from(mode));
    }

    /// Parse a float config value with fallback (skips comments)
    fn tune_value(content: &str, key: &str, fallback: f64) -> f64 {
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with('#') {
                continue;
            }
            if let Some(rest) = t.strip_prefix(key) {
                let rest = rest.trim();
                if rest.starts_with('=') {
                    if let Ok(v) = rest[1..].trim().parse::<f64>() {
                        return v;
                    }
                }
            }
        }
        fallback
    }

    /// Load recognition tuning values from howdy config.ini
    pub fn load_tuning(mut self: Pin<&mut Self>) {
        let mut timeout = 4;
        let mut certainty = 3.5;
        let mut dark = 50.0;
        for candidate in [
            "/usr/lib/security/howdy/config.ini",
            "/lib/security/howdy/config.ini",
            "/etc/howdy/config.ini",
        ] {
            if let Some(content) = read_file_privileged_fallback(candidate) {
                timeout = Self::tune_value(&content, "timeout", 4.0) as i32;
                certainty = Self::tune_value(&content, "certainty", 3.5);
                dark = Self::tune_value(&content, "dark_threshold", 50.0);
                break;
            }
        }
        self.as_mut().set_tune_timeout(timeout);
        self.as_mut().set_tune_certainty(certainty);
        self.as_mut().set_tune_dark_threshold(dark);
    }

    /// Refresh the attempt-snapshots list (newest first, file:// URLs).
    /// Snapshots are world-readable stills Howdy saves per attempt —
    /// useful to see what the camera saw when a login failed.
    pub fn refresh_snapshots(mut self: Pin<&mut Self>) {
        let mut names: Vec<String> = fs::read_dir("/usr/lib/security/howdy/snapshots")
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .filter_map(|e| {
                let n = e.file_name().to_string_lossy().to_string();
                if n.ends_with(".jpg") || n.ends_with(".png") {
                    Some(n)
                } else {
                    None
                }
            })
            .collect();
        names.sort();
        names.reverse();
        names.truncate(30);
        let mut list = QList::<QString>::default();
        for n in &names {
            list.append_clone(&QString::from(
                format!("file:///usr/lib/security/howdy/snapshots/{}", n).as_str(),
            ));
        }
        self.as_mut().set_snapshots(list);
        self.as_mut()
            .set_status_message(QString::from(&format!("{} snapshot(s)", names.len())));
    }

    /// Install + autostart a polkit agent and start one now.
    /// Choice: hyprpolkitagent on Hyprland, else GNOME, else KDE agent.
    /// Autostart via XDG entry always, plus the Hyprland config when present.
    pub fn fix_polkit_agent(mut self: Pin<&mut Self>) {
        if test_run() {
            self.as_mut()
                .set_status_message(QString::from("Polkit agent started (test mode)"));
            return;
        }
        if polkit_agent_running() {
            self.as_mut().set_setup_agent(true);
            self.as_mut()
                .set_status_message(QString::from("Polkit agent already running"));
            return;
        }
        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("XDG_SESSION_DESKTOP"))
            .unwrap_or_default()
            .to_lowercase();
        let on_hyprland = desktop.contains("hyprland");
        let candidates: &[&str] = if on_hyprland {
            &[
                "/usr/lib/hyprpolkitagent/hyprpolkitagent",
                "/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1",
                "/usr/lib/polkit-kde-authentication-agent-1",
            ]
        } else {
            &[
                "/usr/lib/polkit-gnome/polkit-gnome-authentication-agent-1",
                "/usr/lib/polkit-kde-authentication-agent-1",
                "/usr/lib/hyprpolkitagent/hyprpolkitagent",
            ]
        };
        let agent = candidates.iter().find(|p| Path::new(p).exists());
        let agent = match agent {
            Some(a) => a.to_string(),
            None => {
                self.as_mut().set_status_message(QString::from(
                    "No polkit agent installed (hyprpolkitagent / polkit-gnome / polkit-kde)",
                ));
                return;
            }
        };

        // XDG autostart covers GNOME/KDE/COSMIC and autostart daemons.
        let mut notes = Vec::new();
        if let Ok(home) = std::env::var("HOME") {
            let dir = Path::new(&home).join(".config/autostart");
            if std::fs::create_dir_all(&dir).is_ok() {
                let entry = format!(
                    "[Desktop Entry]\nType=Application\nName=FaceKey polkit agent\nExec={}\nNoDisplay=true\nX-GNOME-Autostart-enabled=true\n",
                    agent
                );
                if std::fs::write(dir.join("facekey-polkit-agent.desktop"), entry).is_ok() {
                    notes.push("autostart entry written");
                }
            }
        }
        // Hyprland does not process XDG autostart: patch its own config.
        if on_hyprland {
            if let Ok(home) = std::env::var("HOME") {
                let hp = Path::new(&home).join(".config/hypr");
                for cfg in ["hyprland.lua", "hyprland.conf"] {
                    let p = hp.join(cfg);
                    if let Ok(content) = std::fs::read_to_string(&p) {
                        if !content.contains("hyprpolkitagent")
                            && !content.contains("polkit-kde-authentication-agent")
                            && !content.contains("polkit-gnome-authentication-agent")
                        {
                            let mut c = content;
                            if !c.ends_with('\n') {
                                c.push('\n');
                            }
                            if cfg.ends_with(".lua") {
                                c.push_str("    hl.exec_cmd(\"/usr/lib/hyprpolkitagent/hyprpolkitagent\")\n");
                            } else {
                                c.push_str(
                                    "exec-once = /usr/lib/hyprpolkitagent/hyprpolkitagent\n",
                                );
                            }
                            if std::fs::write(&p, c).is_ok() {
                                notes.push("hyprland autostart patched");
                            }
                        } else {
                            notes.push("hyprland autostart already present");
                        }
                        break;
                    }
                }
            }
        }
        // Start one right now (detached): it inherits our session env.
        let started = Command::new(&agent)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .is_ok();
        std::thread::sleep(std::time::Duration::from_secs(2));
        let running = polkit_agent_running();
        self.as_mut().set_setup_agent(running);
        let mut msg = if running {
            "Polkit agent started".to_string()
        } else if started {
            "Agent launched but not detected yet — check after login".to_string()
        } else {
            "Could not launch the agent".to_string()
        };
        if !notes.is_empty() {
            msg.push_str(&format!(" ({})", notes.join(", ")));
        }
        self.as_mut().set_status_message(QString::from(&msg));
    }

    /// Save recognition tuning values (staged + backed up + pkexec)
    pub fn save_tuning(mut self: Pin<&mut Self>, timeout: i32, certainty: f64, dark: f64) {
        if test_run() {
            self.as_mut().set_tune_timeout(timeout);
            self.as_mut().set_tune_certainty(certainty);
            self.as_mut().set_tune_dark_threshold(dark);
            self.as_mut()
                .set_status_message(QString::from("Recognition tuning saved (test mode)"));
            return;
        }
        let timeout = timeout.clamp(1, 30);
        let certainty = certainty.clamp(1.0, 10.0);
        let dark = dark.clamp(0.0, 100.0);

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

        let mut out = String::new();
        // Rewrite each key in its own pass so replacements never collide.
        let mut pass1 = String::new();
        for line in current_content.lines() {
            let t = line.trim();
            if !t.starts_with('#') && t.starts_with("timeout") && t.contains('=') {
                pass1.push_str(&format!("timeout = {}\n", timeout));
            } else {
                pass1.push_str(line);
                pass1.push('\n');
            }
        }
        let mut pass2 = String::new();
        for line in pass1.lines() {
            let t = line.trim();
            if !t.starts_with('#') && t.starts_with("certainty") && t.contains('=') {
                pass2.push_str(&format!("certainty = {}\n", certainty));
            } else {
                pass2.push_str(line);
                pass2.push('\n');
            }
        }
        for line in pass2.lines() {
            let t = line.trim();
            if !t.starts_with('#') && t.starts_with("dark_threshold") && t.contains('=') {
                out.push_str(&format!("dark_threshold = {}\n", dark));
            } else {
                out.push_str(line);
                out.push('\n');
            }
        }
        let has = |key: &str| {
            current_content.lines().any(|l| {
                let t = l.trim();
                !t.starts_with('#') && t.starts_with(key) && t.contains('=')
            })
        };
        if !has("timeout") {
            out.push_str(&format!("timeout = {}\n", timeout));
        }
        if !has("certainty") {
            out.push_str(&format!("certainty = {}\n", certainty));
        }
        if !has("dark_threshold") {
            out.push_str(&format!("dark_threshold = {}\n", dark));
        }

        let Some(staged) = stage_file("howdy-tuning", &out) else {
            self.as_mut()
                .set_status_message(QString::from("Failed to stage config file"));
            return;
        };
        backup_file(config_path, &current_content);
        let output = Command::new("pkexec")
            .args(["/usr/bin/cp"])
            .arg(&staged)
            .arg(config_path)
            .output();
        let _ = fs::remove_file(&staged);
        match output {
            Ok(o) if o.status.success() => {
                self.as_mut().set_tune_timeout(timeout);
                self.as_mut().set_tune_certainty(certainty);
                self.as_mut().set_tune_dark_threshold(dark);
                self.as_mut()
                    .set_status_message(QString::from("Recognition tuning saved"));
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                self.as_mut().set_status_message(QString::from(&format!(
                    "Failed to save tuning: {}",
                    stderr
                )));
            }
            Err(e) => {
                self.as_mut()
                    .set_status_message(QString::from(&format!("Error: {}", e)));
            }
        }
    }

    /// Run all setup preflight checks
    pub fn run_preflight(mut self: Pin<&mut Self>) {
        if test_run() {
            self.as_mut().set_setup_howdy(true);
            self.as_mut().set_setup_pam_python(true);
            self.as_mut().set_setup_models(true);
            self.as_mut().set_setup_toolchain(true);
            self.as_mut()
                .set_setup_aur_helper(QString::from("yay (test)"));
            self.as_mut().set_setup_agent(true);
            self.as_mut().set_setup_ir_camera(true);
            return;
        }
        self.as_mut().set_setup_howdy(find_howdy().is_some());
        // Any usable PAM flavor counts (pam_python on Arch, pam_howdy on Debian)
        let kind = pam_module_kind();
        self.as_mut().set_setup_pam_python(!kind.is_empty());
        self.as_mut().set_pam_module_kind(QString::from(kind));
        self.as_mut().set_setup_models(dlib_models_present());
        let debian = detect_distro().is_debian();
        self.as_mut().set_is_debian(debian);
        // Debian-family installs are not supported in this release, so the
        // Arch checks below run everywhere (harmless elsewhere).
        self.as_mut().set_setup_toolchain(
            ["gcc", "make", "pkgconf", "fakeroot"]
                .iter()
                .all(|p| which_first(&[*p]).is_some()),
        );
        let helper = which_first(&["yay", "paru"]).unwrap_or_default();
        self.as_mut().set_setup_aur_helper(QString::from(&helper));
        self.as_mut().set_setup_agent(polkit_agent_running());
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
        if test_run() {
            // Mirror the real 4-node layout (incl. metadata nodes) so the
            // test walkthrough exercises the same indices as hardware.
            let mut candidates = QList::<QString>::default();
            let mut paths = QList::<QString>::default();
            for (d, s) in [
                ("/dev/video0", "/dev/video0 — 1280x720"),
                ("/dev/video1", "/dev/video1 — no capture formats"),
                ("/dev/video2", "/dev/video2 — 340x340 · likely IR"),
                ("/dev/video3", "/dev/video3 — no capture formats"),
            ] {
                candidates.append_clone(&QString::from(s));
                paths.append_clone(&QString::from(d));
            }
            self.as_mut().set_camera_candidates(candidates);
            self.as_mut().set_camera_paths(paths);
            self.as_mut()
                .set_suggested_camera(QString::from("/dev/video2"));
            self.as_mut().set_setup_ir_camera(true);
            return;
        }
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
        if test_run() {
            self.as_mut().set_install_running(true);
            self.as_mut().set_install_done(false);
            self.as_mut().set_install_failed(false);
            self.as_mut().set_install_error(QString::from(""));
            self.as_mut().set_install_log(QString::from(
                "Test mode: simulating system-package install…\n",
            ));
            std::thread::spawn(move || {
                for line in [
                    "resolving dependencies…",
                    "looking for conflicting packages…",
                    "Packages (9) qt6-base-6.11.2  qt6-declarative-6.11.2  qt6-multimedia-6.11.2",
                    "           v4l-utils-1.32.0  mpv-0.41.0  polkit-127  gcc-16.2.1  make-4.4.1",
                    "Total Download Size:   48.20 MiB",
                    "Total Installed Size:  212.44 MiB",
                    ":: Retrieving packages … 100%",
                    ":: Processing package changes …",
                    "installing qt6-base … done",
                    "installing v4l-utils mpv polkit … done",
                    "installing build tools (gcc/make/pkgconf/fakeroot) … done",
                    "All system packages installed. AUR step is next: howdy + pam-python.",
                ] {
                    std::thread::sleep(std::time::Duration::from_millis(700));
                    let mut prev =
                        std::fs::read_to_string("/tmp/facekey_install.log").unwrap_or_default();
                    prev.push_str(line);
                    prev.push('\n');
                    let _ = std::fs::write("/tmp/facekey_install.log", prev);
                }
                let _ = std::fs::write("/tmp/facekey_install_done", "0");
            });
            return;
        }
        self.as_mut().set_install_running(true);
        self.as_mut().set_install_done(false);
        self.as_mut().set_install_failed(false);
        self.as_mut().set_install_error(QString::from(""));
        let _ = fs::remove_file("/tmp/facekey_install_done");
        self.as_mut().set_install_log(QString::from(
            "Installing system packages — authenticate in the popup…\n",
        ));
        std::thread::spawn(move || {
            let _ = fs::write(
                "/tmp/facekey_install.log",
                "FaceKey system-package install\n",
            );
            let debian = detect_distro().is_debian();
            if debian {
                // Debian-family installs are not supported in this release.
                let _ = fs::write("/tmp/facekey_install_done", "UNSUPPORTED");
                return;
            }
            let pkgs = REPO_PACKAGES_ARCH.join(" ");
            let install_cmd = format!(
                "pacman -S --needed --noconfirm {} >> /tmp/facekey_install.log 2>&1",
                pkgs
            );
            let script = format!(
                "{}; code=$?; chmod 644 /tmp/facekey_install.log; echo $code > /tmp/facekey_install_done; exit $code",
                install_cmd
            );
            let _ = Command::new("pkexec")
                .args(["/usr/bin/bash", "-c", &script])
                .output();
            // Finding 6: if pkexec itself failed (no auth, no binary), the
            // script above never ran and wrote no marker — publish one so
            // the poller cannot spin forever.
            if !Path::new("/tmp/facekey_install_done").exists() {
                let _ = fs::write("/tmp/facekey_install_done", "PKEXEC_FAILED");
            }
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
            match code.trim() {
                "0" => {
                    self.as_mut().set_install_done(true);
                }
                // Finding 6: a failed install is a FAILED state with retry,
                // never a completed one.
                other => {
                    self.as_mut().set_install_failed(true);
                    let detail = if other == "PKEXEC_FAILED" {
                        "authorization was cancelled or pkexec failed — press Install to retry"
                    } else if other == "UNSUPPORTED" {
                        "Debian-family installs are not supported in this release"
                    } else {
                        "package install failed — see log above, then press Install to retry"
                    };
                    self.as_mut().set_install_error(QString::from(detail));
                }
            }
            return true;
        }
        false
    }

    /// Open a user terminal running the engine install.
    /// Arch only: yay/paru build howdy + pam-python from the AUR (helpers
    /// refuse root, so this runs unelevated and yay asks for sudo itself).
    /// Debian-family installs are not supported in this release.
    pub fn launch_aur_install(mut self: Pin<&mut Self>) {
        if test_run() {
            self.as_mut().set_install_error(QString::from(""));
            self.as_mut().set_install_log(QString::from(
                "Test mode: would open a terminal running `yay -S howdy pam-python`.\nPress “I finished — check again” to continue the walkthrough.",
            ));
            return;
        }
        let debian = detect_distro().is_debian();
        if debian {
            self.as_mut().set_install_error(QString::from(
                "Debian-family installs are not supported in this release",
            ));
            return;
        }
        let inner = {
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
            // NOTE: python-dlib compiles from source here — honest 20-60 min
            // on fresh machines. The terminal stays open so the log is visible.
            format!(
                "echo 'FaceKey: installing howdy + pam-python (dlib compile takes a while, leave it running)'; {} -S --needed --noconfirm howdy pam-python",
                helper_base
            )
        };
        let term = match which_first(&["foot", "kitty", "konsole", "gnome-terminal", "xterm"]) {
            Some(t) => t,
            None => {
                self.as_mut()
                    .set_install_error(QString::from("No terminal emulator found"));
                return;
            }
        };
        // NOTE: python-dlib compiles from source here — honest 20-60 min
        // on fresh machines. The terminal stays open so the log is visible.
        let script = format!(
            "{}; echo; echo '--- FaceKey: close this window when finished, then press Continue ---'; read _",
            inner
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
                self.as_mut()
                    .set_install_error(QString::from(&format!("Could not open terminal: {}", e)));
            }
        }
    }

    /// True when howdy + a PAM module are both present (refreshes preflight)
    pub fn check_install_done(mut self: Pin<&mut Self>) -> bool {
        if test_run() {
            return true;
        }
        let done = find_howdy().is_some() && !pam_module_kind().is_empty();
        if done {
            self.as_mut().set_setup_howdy(true);
            self.as_mut().set_setup_pam_python(true);
            self.as_mut().set_setup_models(dlib_models_present());
        }
        done
    }
}
