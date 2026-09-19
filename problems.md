# FaceKey audit findings

Audit scope: every tracked source, QML, build, packaging, and documentation file in
this repository. Findings are ordered by severity. This is a review report only;
no application code was changed.

## Critical

### 1. Root command injection during face enrollment

**Affected:** `src/bridge.rs`, `add_model` (around lines 758–769).

The face label, the resolved Howdy path, and the target username are interpolated
directly into a string executed as `pkexec /usr/bin/bash -c`. Removing only single
quotes from the label does not make it shell-safe: double quotes, `$()`, backticks,
backslashes, and newlines remain meaningful while Bash parses the generated command.
For example, a label containing `$(command)` is expanded by the root shell before it
is passed to Python. This permits arbitrary commands with the privileges granted by
the pkexec authorization dialog. The same construction makes unusual but permitted
environment-derived usernames unsafe.

**Fix:** Do not use a shell. Run Howdy and Python as separate `Command` invocations
with argument arrays, or pass the label and path through a root-owned helper using a
structured input format. Treat `target_user()` and `find_howdy()` results as untrusted
arguments, not shell syntax.

### 2. Raceable, predictable `/tmp` files are consumed by privileged commands

**Affected:** `src/bridge.rs`, fixed files used for PAM/config/polkit updates and
install state (around lines 728–729, 784–805, 1073–1083, 1282–1324, and 1588–1599).

The program writes predictable names in world-writable `/tmp` and later copies them
from a `pkexec` Bash process or `pkexec cp`. An attacker running as the desktop user
can replace a temporary file after the unprivileged write but before the privileged
copy. In particular, replacing `howdy_pam_tmp` or `howdy_config_tmp.ini` lets an
attacker substitute arbitrary content into a root-owned PAM or Howdy configuration
file after the user authorizes FaceKey. Symlinks can also cause the unprivileged
writer to overwrite files writable by the attacker. The result/status files are
similarly spoofable across app instances and users.

**Fix:** Use a private directory created with mode `0700` (for example via
`tempfile::TempDir`) and securely create files with `create_new`, mode `0600`, and
no symlink following. A privileged helper should receive data through a protected
file descriptor or verify ownership, mode, inode, and contents before use. Avoid
using `/tmp` as IPC; keep asynchronous operation state in process memory.

## High

### 3. `toggle_pam` accepts an arbitrary path and writes it with pkexec

**Affected:** `src/bridge.rs`, `toggle_pam` (around lines 1121–1200 and 1350–1352).

Although the shipped QML passes known PAM paths, the public QML invokable accepts a
caller-controlled `QString`, reads that file, modifies lines matching broad Howdy
substrings, and asks pkexec to copy the result back to the same path. There is no
allowlist, canonicalization, regular-file check, or protection against symlinks.
This turns any future QML injection, extension, or misuse of the exposed backend API
into an arbitrary privileged file-modification primitive for files readable by the
user.

**Fix:** Make the API accept a small enum rather than a path and map it internally to
an allowlisted canonical path. Reject non-regular files, avoid symlink traversal, and
perform the read/modify/write transaction in a purpose-built privileged helper.

### 4. Enabling polkit deliberately removes the helper's device isolation

**Affected:** `src/bridge.rs`, `toggle_pam` polkit branch (around lines 1289–1321).

The generated systemd drop-in sets `PrivateDevices=no` for
`polkit-agent-helper@.service`. This makes the host device namespace available to a
security-sensitive authentication helper; `DeviceAllow=char-video4linux rw` does not
restore the isolation once `PrivateDevices` is disabled. Face recognition may require
camera access, but this materially expands the attack surface of every polkit
authentication request and persists after the GUI exits.

**Fix:** Do not disable `PrivateDevices` globally. Use the least-privilege systemd
device controls supported by the target system (for example a narrowly scoped bind
or device policy), verify that they work on supported versions, and provide a tested
rollback path.

## Medium

### 5. Enrollment can be reported as successful by forged or stale result files

**Affected:** `src/bridge.rs`, `add_model`/`check_add_result`/`discard_face` (around
lines 723–732 and 809–841); `qml/main.qml`, enrollment poller (lines 188–203).

`check_add_result` trusts the contents of `/tmp/howdy_add_result.txt` without tying
it to an enrollment request. Any local process can write the exact success string,
causing the UI to claim a face was captured. A stale completion from another
FaceKey instance can be consumed by the wrong dialog. The related
`howdy_new_face_id.txt` can cause “Discard” to remove a different numeric model ID
for the current user.

**Fix:** Keep a per-request result channel in memory, include an unguessable request
identifier if IPC is unavoidable, and only permit discard for the ID returned by the
same active enrollment operation.

### 6. A failed or cancelled package install is shown as completed and cannot be retried

**Affected:** `src/bridge.rs`, `poll_install_log` (lines 1615–1624);
`qml/SetupWizard.qml`, install button (lines 233–238).

When the completion marker contains a nonzero exit status, `poll_install_log` still
sets `install_done` to `true`. The UI changes the button to “Done ✓” and disables it,
even while also displaying an error. In addition, the background thread ignores the
outer pkexec result. If authorization is denied/cancelled or Bash cannot start, the
script never writes the completion marker, leaving `install_running` true and the
poller running forever.

**Fix:** Set `install_done` only on exit status zero, expose a distinct failed state,
and always publish a completion result from the Rust thread—including pkexec spawn,
authorization, and child-process failures. Allow retry after cleanup.

### 7. Camera/configuration detection treats commented settings as active and does not
verify the configured device

**Affected:** `src/bridge.rs`, `check_device` (lines 544–585) and
`load_video_devices` (lines 952–976).

Both parsers accept a line beginning with `device_path` without first rejecting a
commented line. `load_video_devices` then marks the camera configured without checking
that the referenced path exists. This can bypass the first-run wizard and display a
ready/configured state even when Howdy has no usable camera; `check_device` can also
read a commented `disabled` directive as effective.

**Fix:** Parse INI syntax rather than using prefixes; ignore comments, select the
effective section/key, and require a valid existing character device (or validated
stable symlink target) before reporting configuration success.

### 8. Enrollment timeout abandons a live privileged operation

**Affected:** `qml/main.qml`, lines 177–205 and 303–327; `src/bridge.rs`,
lines 758–806.

After 90 seconds the QML timer stops polling and presents a retry state, but the
background pkexec/Howdy enrollment continues. Its later result remains in `/tmp` and
can be consumed by a subsequent attempt; a face can be enrolled after the user was
told it timed out. There is no cancellation, operation identifier, or prevention of
overlapping enrollment threads.

**Fix:** Model enrollment as one tracked operation, disable retry until it completes
or is explicitly cancelled, terminate the child process on timeout where safe, and
discard late results that do not match the active operation ID.

## Low / compatibility and correctness

### 9. The PAM editor is not transaction-safe and can create unsafe service configs

**Affected:** `src/bridge.rs`, `toggle_pam` (lines 1177–1279).

The code reads a PAM file unprivileged, makes broad substring replacements, then
copies an entire reconstructed file over `/etc/pam.d/...`. It does not preserve a
backup, validate the resulting PAM stack, retain final-file metadata, or detect a
concurrent package/admin update. It also seeds a missing `/etc` file from a vendor
file, potentially freezing an outdated vendor configuration. A malformed or
incompatible generated stack can lock users out of sudo, login, or the screen locker.

**Fix:** Use a narrowly scoped, validated PAM drop-in where the distribution supports
it; otherwise make timestamped backups, atomically install a root-created temporary
file with preserved permissions/ownership, validate before activation, and offer a
recovery command.

### 10. The codebase fails its Rust formatting check

**Affected:** `build.rs` and `src/bridge.rs`.

`cargo fmt --check` reports formatting diffs in these files. This is a CI-quality
failure and makes automated formatting enforcement fail before functional tests can
be trusted.

**Fix:** Run `cargo fmt` and enforce `cargo fmt --check` in CI.

### 11. Full compilation could not be verified in the documented environment

**Affected:** build configuration/development workflow.

`cargo check` fails before compiling the application because `cxx-qt-build` cannot
find a Qt installation. The README lists the required packages, but the project does
not provide a CI environment or preflight that verifies the supported build setup.

**Fix:** Add CI on a supported Arch-based image with Qt development tooling, and
document the exact Qt/clang packages and environment variables required by
`cxx-qt-build`.
