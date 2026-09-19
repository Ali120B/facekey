# FaceKey v2 — guided installer + manager

Goal: all-in-one Face ID for Linux on Arch-based distros.
Auth UX: system (pkexec) dialogs. Scope: Arch/CachyOS only, with a distro
backend trait so Ubuntu can plug in later.

## Phase 0 — FaceKey rebrand ✅
- [x] Binary/package/desktop/policy/strings → facekey
- [x] Repo → github.com/Ali120B/facekey
- [x] Rebuild + smoke test + push

## Phase 1 — Backend detection + install API (bridge.rs)
- [ ] Preflight properties: howdy, pam_python, dlib models, toolchain
      (gcc/make/pkgconf/fakeroot), AUR helper (yay/paru), polkit agent,
      IR camera
- [ ] `run_preflight()` populating the above
- [ ] `probe_cameras()` — per-/dev/videoN format summary via v4l2-ctl,
      square-lowres IR heuristic, `suggested_camera`
- [ ] Repo-package install via pkexec pacman, streamed to
      /tmp/facekey_install.log (`start_repo_install`, `poll_install_log`)
- [ ] AUR handoff: open user terminal running yay (foot/kitty/konsole/
      gnome-terminal/xterm), `check_install_done()` polling
- [ ] Build + offscreen smoke + qmllint + push

## Phase 2 — QML wizard screens
- [ ] Welcome → Preflight → Install (log view) → Camera (QtMultimedia
      in-app preview + IR suggestion) → Enroll (reuse flow) →
      Integrate (PAM toggles) → Done
- [ ] Show wizard on first run (no howdy / no device), skip otherwise
- [ ] Screenshots under Xvfb, push

## Phase 3 — Doctor screen
- [ ] Reuse preflight as a permanent health page (incl. polkit-agent check)
- [ ] Push

## Phase 4 — Release
- [ ] Bump version, tag, AppImage draft release

## Install plan (Arch, verified on CachyOS 2026-09)
- Repo (pkexec): qt6-base qt6-declarative qt6-multimedia v4l-utils mpv
  polkit gcc make pkgconf fakeroot
- AUR (user terminal, yay refuses root): `yay -S --needed howdy pam-python`
  (pulls python-dlib → long compile, warn honestly in UI)
- dlib models ship inside the howdy package — no download step
- pam-python needs python2 (AUR dep, auto-resolved)
