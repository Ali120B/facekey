# FaceKey friend test — Pop!_OS checklist

You only need the AppImage file. Nothing gets installed by FaceKey
itself; every system change happens through a password popup you
approve, and every PAM edit keeps a timestamped backup.

## 0. Machine facts (paste into your reply)

```bash
cat /etc/os-release | head -n 4
ls /dev | grep video
v4l2-ctl --list-devices 2>/dev/null || sudo apt install -y v4l-utils
```

## 1. Wizard walkthrough
1. Run the AppImage. The setup wizard should appear.
2. Preflight: screenshot which rows are red.
3. Install: system packages first, then the terminal step
   (answer the Fast/Balanced/Secure question with Balanced).
4. Camera: does the suggestion match your real camera? Does the
   live preview show it? Save it.
5. Enroll a face, then Test recognition from the manager.

## 2. sudo
```bash
sudo -k
sudo whoami
```
Look at the camera on the empty prompt. Expected: login without typing.
Then type a wrong password path: run again, type your password immediately —
you still get in after the face timeout (~seconds).

## 3. Screen lock
Lock (Super+L), then try to unlock by looking at the camera.
Report: works / falls back to password / hangs (with how long).

## 4. Login screen (experimental — expect issues, report them)
Log out. At the greeter, press Enter on the empty password box and look
at the camera. Report exactly what happens (instant in / delay then
password / nothing). **Keep a terminal logged in via Ctrl+Alt+F3 as
your way back in, just in case.**

## 5. Negative cases
- Dark room enroll + unlock attempt.
- Another person's face on sudo (should fall back to password).
- Lid closed (Howdy should skip itself).

## 6. If anything fails, capture this

```bash
journalctl -b -p info | grep -iE "howdy|polkit|pam_unix.*failure" | tail -n 20
sudo cat /etc/pam.d/gdm-password 2>/dev/null | head -n 8
groups gdm 2>/dev/null; ls /dev/video*
```

Paste the checklist with PASS/FAIL per item plus the log output.
Screenshots of any error dialog help more than descriptions.
