# FaceKey Pop!_OS findings (research log)

## 2026-09-20 — install landscape (web research)
- Howdy ships per-distro: Ubuntu PPA `ppa:boltgolt/howdy`, AUR `howdy`,
  Fedora COPR, openSUSE wiki. Source: boltgolt/howdy README.
- System76 publishes an official Pop!_OS Howdy guide (updated Aug 2026):
  `sudo add-apt-repository -y ppa:boltgolt/howdy` then
  `sudo apt install -y howdy`. Guide covers screen-lock login only.
- The .deb install is INTERACTIVE: debconf asks Fast/Balanced/Secure
  certainty profile mid-install → installer must use terminal handoff,
  never silent pkexec.
- Post-install the .deb auto-downloads face-recognition deps.
- Default `device_path = /dev/v4l/by-path/none` → our probe+save flow applies.
- dlib build-from-source can hang at 100% for 1+ min (upstream note) —
  prefer PPA binaries, never source builds in the wizard.
- PPA tutorials confirm 24.04 (+25.10) coverage (ubuntuhandbook).
- Upstream GDM greeter login is flaky: boltgolt/howdy#927 (no auth at
  system login on 24.04), #976 (3.0 beta login problem needing manual
  compare.py edit). GNOME lock screen is the proven path.
- AUR `howdy` 2.6.1-3 depends: libinih libevdev python python-dlib.
  AUR `pam-python` 1.0.8-4 depends: pam python2.
- CachyOS/Arch notes: `howdy` is AUR-only (not in repos); toolchain
  (gcc/make/pkgconf/fakeroot) present individually, no base-devel group;
  dlib models ship inside the package (no download step).

## Camera facts (verified on this box)
- RGB /dev/video0 (MJPG/YUYV up to 1280x720), IR /dev/video2 (YUYV 340x340).
- IR heuristic: square frame, 100–480px per side.
- RGB fallback: first device with capture formats when no IR found.
