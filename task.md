# FaceKey Pop!_OS support — task plan

Goal: FaceKey installs, enrolls and manages Howdy on Debian-family
systems (Pop!_OS first) exactly like it does on Arch, for IR **and**
normal RGB cameras. Auth: system dialogs. No blind PAM edits — every
new matrix entry is verified on hardware before it ships.

## Phases

- [x] **Phase 1 — Distro backend foundation** (no hardware needed)
  - [x] `detect_distro()` from /etc/os-release (ID + ID_LIKE)
  - [x] Expose `distro_id`, `distro_like`, `pkg_manager` to QML
  - [x] Arch path byte-identical to today (no behavior change)
  - [x] Build + smoke + push

- [x] **Phase 2 — apt install flow**
  - [x] Terminal-handoff PPA + apt (debconf stays interactive)
  - [x] pkexec apt for repo deps (v4l-utils)
  - [x] PAM flavor auto-detect (pam_python vs pam_howdy) in preflight + done-check
  - [x] Preflight/Install UI adapts per distro (is_debian)
  - [x] GDM backend (detect + toggle + experimental UI rows, HW verification pending)
  - [x] Build + lint + smoke (both modes) + contract check + push

- [ ] **Phase 3 — GDM/GNOME PAM matrix** (needs hardware)
  - [ ] Identify greeter service (gdm-password?) + lock service on Pop
  - [ ] gdm user camera access (video group, like sddm was)
  - [ ] Toggles behind hardware verification ONLY
  - [ ] sudo entry (same pattern as Arch, verify)

- [ ] **Phase 4 — Camera copy for non-IR**
  - [ ] Wizard/preflight wording: IR recommended, RGB supported
  - [ ] Heuristic labels RGB picks explicitly (no behavior change)

- [ ] **Phase 5 — Friend test loop**
  - [ ] AppImage test builds + written checklist (sudo, lock, logout,
        reboot-login, dark room, wrong face) + one-command log capture
  - [ ] Fix cycles from journal evidence

- [ ] **Phase 6 — Release**
  - [ ] Version bump, tag, AppImage, README (Pop section)

## Decisions
- Arch-only until Debian matrix is hardware-verified (no unverified PAM).
- GDM greeter ships as experimental until friend proves it; sudo+lock first.
- Friend's Pop!_OS PC is QA; AppImage-only delivery (nothing to install).

## Errors encountered
| Error | Attempt | Resolution |
|-------|---------|------------|
| (none yet) | | |
