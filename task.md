# FaceKey v2 — guided installer + manager

Goal: all-in-one Face ID for Linux on Arch-based distros.
Auth UX: system (pkexec) dialogs. Scope: Arch/CachyOS only, with a distro
backend trait so Ubuntu can plug in later.

## Phase 0 — FaceKey rebrand ✅
- [x] Binary/package/desktop/policy/strings → facekey
- [x] Repo → github.com/Ali120B/facekey
- [x] Rebuild + smoke test + push

## Phase 1 — Backend detection + install API (bridge.rs)
- [x] Preflight properties: howdy, pam_python, dlib models, toolchain
      (gcc/make/pkgconf/fakeroot), AUR helper (yay/paru), polkit agent,
      IR camera
- [x] `run_preflight()` populating the above
- [x] `probe_cameras()` — per-/dev/videoN format summary via v4l2-ctl,
      square-lowres IR heuristic, `suggested_camera`
- [x] Repo-package install via pkexec pacman, streamed to
      /tmp/facekey_install.log (`start_repo_install`, `poll_install_log`)
- [x] AUR handoff: open user terminal running yay (foot/kitty/konsole/
      gnome-terminal/xterm), `check_install_done()` polling
- [x] Build + offscreen smoke + push

## Phase 2 — QML wizard screens
- [x] --test-run dry-run mode: simulated responses everywhere, verified
      zero pkexec/howdy/mpv/AUR spawns and zero polkit authorizations
- [x] Welcome → Preflight → Install (log view) → Camera (QtMultimedia
      in-app preview + IR suggestion) → Enroll (reuse flow) →
      Integrate (PAM toggles) → Done
- [x] Show wizard on first run (no howdy / no device), skip otherwise
- [x] Fixed UiSwitch programmatic-toggle recursion (busy guard, main too)
- [x] Screenshots under Xvfb (Welcome verified), lint + push

## Phase 3 — Doctor screen
- [x] Permanent health dialog (howdy, PAM module, models, agent, IR,
      device configured) with Re-check + Re-run setup entry
- [x] Health button in header, lint + push

## Phase 4 — Release
- [ ] Bump version, tag, AppImage draft release

## Install plan (Arch, verified on CachyOS 2026-09)
- Repo (pkexec): qt6-base qt6-declarative qt6-multimedia v4l-utils mpv
  polkit gcc make pkgconf fakeroot
- AUR (user terminal, yay refuses root): `yay -S --needed howdy pam-python`
  (pulls python-dlib → long compile, warn honestly in UI)
- dlib models ship inside the howdy package — no download step
- pam-python needs python2 (AUR dep, auto-resolved)
