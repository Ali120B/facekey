# FaceKey — task plan (Pop!_OS support DROPPED in v4.0.0)

Arch-based only. The Debian backend (apt install, PPA handoff, GDM
matrix) was prototyped across Phase 1–2, then cut: impractical without
local hardware, and unverified PAM edits are a lockout risk. The work
survives in git history (`git log --all --grep="Phase 2"`,
`docs/friend-test.md` before removal) if it ever comes back.
Kept deliberately: distro detection props, PAM flavor auto-detect
(pam_python vs pam_howdy) — both harmless and correct everywhere.

## Phases (all complete unless noted)
- [x] Phase 0 — FaceKey rebrand
- [x] Phase 1 — Backend detection + install API
- [x] Phase 2 — was apt flow; cut, Arch-only wizard retained
- [x] Phase 3 — Doctor screen
- [x] Phase 4 — Camera copy for non-IR
- [x] Phase 5 — was friend-test kit; removed with Pop scope
- [x] Phase 6 — Release 4.0.0

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

- [x] **Phase 4 — Camera copy for non-IR**
  - [x] Wizard/manager/doctor wording works for RGB webcams too
  - [x] Heuristic still flags IR; RGB is explicit fallback (verified logic)

- [x] **Phase 5 — Friend test loop**
  - [x] `docs/friend-test.md`: checklist + log capture + GDM probes
  - [x] README notes Pop status
  - [ ] Awaiting friend hardware results (GDM greeter + lock service TBD)

- [x] **Phase 6 — Release**
  - [x] 3.2.0: audit fixes, test-run follow-ups, Debian backend,
        GDM support, non-IR copy, friend-test kit

## Decisions
- Arch-only until Debian matrix is hardware-verified (no unverified PAM).
- GDM greeter ships as experimental until friend proves it; sudo+lock first.
- Friend's Pop!_OS PC is QA; AppImage-only delivery (nothing to install).

## Errors encountered
| Error | Attempt | Resolution |
|-------|---------|------------|
| (none yet) | | |
