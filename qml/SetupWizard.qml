import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia

// First-run setup wizard: Welcome → Preflight → Install → Camera →
// Enroll → Integrate → Done. Shown over the manager until setup completes.
Rectangle {
    id: wizard
    anchors.fill: parent
    color: bg
    z: 100

    // Injected from main.qml
    property var backend
    property color bg: "#131519"
    property color card: "#1C1F26"
    property color field: "#14161B"
    property color line: "#2B303B"
    property color ink: "#ECEEF1"
    property color dim: "#9AA3B2"
    property color accent: "#8B7CFF"
    property color good: "#3DDC97"
    property color bad: "#FF6B6B"
    property color warn: "#FFC857"

    signal requestEnroll(string name)
    signal wizardFinished()

    property int step: 0
    property string enrollName: ""
    property bool polkitArmed: false
    property string camError: ""

    // Match a /dev path to Qt's camera device list by stable id instead
    // of position: Qt also enumerates metadata nodes, so indices shift.
    // NOTE: CameraDevice.id arrives as a Qt string object without JS
    // methods — String() it before calling endsWith(), or the binding
    // throws and the preview stays black.
    function qtCamIndex(devices, path) {
        path = String(path || "")
        if (path === "" || path === "undefined") return 0
        var base = path.split("/").pop()
        for (var i = 0; i < devices.length; i++) {
            var id = String(devices[i].id || "")
            if (id === path || (base !== "" && id.endsWith("/" + base))) return i
        }
        return 0
    }

    function needsInstall() {
        return !(backend.setup_howdy && backend.setup_pam_python)
    }
    function goNext() {
        // Skip the install step when nothing is missing — but never in
        // test mode, where the whole point is walking every screen.
        if (wizard.step === 1 && !wizard.needsInstall() && !backend.is_test_run()) {
            wizard.step = 3
            enterCamera()
            return
        }
        wizard.step = Math.min(6, wizard.step + 1)
        if (wizard.step === 3) enterCamera()
        if (wizard.step === 6) backend.refresh_models()
    }
    function goBack() {
        if (wizard.step === 3 && !wizard.needsInstall() && !backend.is_test_run()) {
            wizard.step = 1
            return
        }
        wizard.step = Math.max(0, wizard.step - 1)
    }
    function enterCamera() {
        backend.probe_cameras()
    }
    function checkRow(ok, title, sub) {
        return { ok: ok, title: title, sub: sub }
    }

    // Install log poller (repo packages)
    Timer {
        id: installPoller
        interval: 800
        repeat: true
        running: false
        onTriggered: {
            if (backend.poll_install_log()) {
                installPoller.stop()
                backend.run_preflight()
            }
        }
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 28
        spacing: 14

        // Progress header
        RowLayout {
            Layout.fillWidth: true
            Label {
                text: ["Welcome", "Preflight", "Install", "Camera", "Enroll", "Integrate", "Done"][wizard.step]
                font.pixelSize: 13
                color: dim
            }
            Item { Layout.fillWidth: true }
            Label { text: (wizard.step + 1) + " / 7"; font.pixelSize: 12; color: dim }
        }
        Rectangle { Layout.fillWidth: true; height: 3; radius: 2; color: line
            Rectangle {
                height: parent.height; radius: parent.radius; color: accent
                width: parent.width * (wizard.step + 1) / 7
                Behavior on width { NumberAnimation { duration: 200 } }
            }
        }

        StackLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            currentIndex: wizard.step

            // ── 0 Welcome ───────────────────────────────────────────────
            ColumnLayout {
                spacing: 14
                Item { Layout.fillHeight: true }
                Label {
                    Layout.alignment: Qt.AlignHCenter
                    text: "FaceKey"
                    font.pixelSize: 34
                    font.bold: true
                    color: ink
                }
                Label {
                    Layout.alignment: Qt.AlignHCenter
                    text: "Face ID for Linux"
                    font.pixelSize: 15
                    color: dim
                }
                Label {
                    Layout.fillWidth: true
                    horizontalAlignment: Text.AlignHCenter
                    wrapMode: Text.Wrap
                    font.pixelSize: 13
                    color: dim
                    text: "This wizard installs Howdy and its dependencies,\nfinds your IR camera, enrolls your face\nand wires up login, lock and sudo."
                }
                Item { Layout.fillHeight: true }
                RowLayout {
                    Layout.alignment: Qt.AlignHCenter
                    spacing: 8
                    UiButton { text: "Skip to manager"; kind: "ghost"; onClicked: wizard.wizardFinished() }
                    UiButton { text: "Get started"; kind: "accent"; onClicked: { backend.run_preflight(); wizard.goNext() } }
                }
            }

            // ── 1 Preflight ─────────────────────────────────────────────
            ColumnLayout {
                spacing: 10
                Label { text: "System check"; font.pixelSize: 20; font.bold: true; color: ink }
                Label { text: "What FaceKey found on this machine:"; font.pixelSize: 13; color: dim }

                UiCard {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    ListView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        clip: true
                        interactive: false
                        model: [
                            wizard.checkRow(backend.setup_howdy, "Howdy face engine", backend.setup_howdy ? "installed" : "missing — installed in the next step"),
                            wizard.checkRow(backend.setup_pam_python, "PAM module (pam-python)", backend.setup_pam_python ? "installed" : "missing — installed in the next step"),
                            wizard.checkRow(backend.setup_models, "AI face models", backend.setup_models ? "on disk" : "missing"),
                            wizard.checkRow(backend.setup_toolchain, "Build tools (gcc/make/pkgconf/fakeroot)", backend.setup_toolchain ? "ready for AUR builds" : "missing — installed in the next step"),
                            wizard.checkRow(backend.setup_aur_helper !== "", "AUR helper", backend.setup_aur_helper !== "" ? backend.setup_aur_helper : "none found (need yay or paru)"),
                            wizard.checkRow(backend.setup_agent, "Polkit agent", backend.setup_agent ? "running" : "not running — password popups fall back to terminal"),
                            wizard.checkRow(backend.setup_ir_camera, "IR camera", backend.setup_ir_camera ? "detected" : "none detected yet")
                        ]
                        delegate: RowLayout {
                            width: ListView.view.width
                            height: 34
                            spacing: 10
                            Rectangle {
                                width: 9; height: 9; radius: 5
                                Layout.alignment: Qt.AlignVCenter
                                color: modelData.ok ? good : warn
                            }
                            ColumnLayout {
                                Layout.fillWidth: true
                                spacing: 0
                                Label { text: modelData.title; font.pixelSize: 13; font.bold: true; color: ink }
                                Label { text: modelData.sub; font.pixelSize: 11; color: dim; elide: Text.ElideRight; Layout.fillWidth: true }
                            }
                        }
                    }
                }
                RowLayout {
                    Layout.fillWidth: true
                    UiButton { text: "‹ Back"; kind: "ghost"; onClicked: wizard.goBack() }
                    Item { Layout.fillWidth: true }
                    UiButton { text: "Re-check"; kind: "tonal"; onClicked: backend.run_preflight() }
                    UiButton { text: wizard.needsInstall() ? "Install missing" : "Continue"; kind: "accent"; onClicked: wizard.goNext() }
                }
            }

            // ── 2 Install ───────────────────────────────────────────────
            ColumnLayout {
                spacing: 10
                Label { text: "Install"; font.pixelSize: 20; font.bold: true; color: ink }
                Label {
                    Layout.fillWidth: true
                    wrapMode: Text.Wrap
                    font.pixelSize: 13
                    color: dim
                    text: "Step 1 installs system packages (one password popup).\nStep 2 opens a terminal for the AUR builds — yay asks for sudo there itself.\nHonest warning: python-dlib compiles from source, budget 20–60 minutes."
                }

                UiCard {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 210
                    RowLayout {
                        Layout.fillWidth: true
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 1
                            Label { text: "System packages"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "qt6, v4l-utils, mpv, polkit, build tools"; font.pixelSize: 11; color: dim }
                        }
                        UiButton {
                            text: backend.install_done ? "Done ✓" : "Install"
                            kind: "tonal"
                            enabled: !backend.install_running && !backend.install_done
                            onClicked: { backend.start_repo_install(); installPoller.start() }
                        }
                    }
                    ProgressBar {
                        Layout.fillWidth: true
                        visible: backend.install_running || backend.install_done
                        indeterminate: backend.install_running && !backend.install_done
                        value: backend.install_done ? 1 : 0
                    }
                    ScrollView {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        TextArea {
                            readOnly: true
                            wrapMode: Text.Wrap
                            font.family: "monospace"
                            font.pixelSize: 11
                            color: dim
                            text: backend.install_log
                        }
                    }
                    Label {
                        Layout.fillWidth: true
                        visible: backend.install_error !== ""
                        text: backend.install_error
                        color: bad
                        font.pixelSize: 12
                        wrapMode: Text.Wrap
                    }
                }

                UiCard {
                    Layout.fillWidth: true
                    Layout.preferredHeight: 110
                    RowLayout {
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 1
                            Label { text: "Howdy + PAM module (AUR)"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "Opens a terminal — type your sudo password there when asked"; font.pixelSize: 11; color: dim; wrapMode: Text.Wrap; Layout.fillWidth: true }
                        }
                        UiButton { text: "Open terminal installer"; kind: "tonal"; onClicked: backend.launch_aur_install() }
                    }
                    Label {
                        Layout.fillWidth: true
                        visible: backend.install_error !== ""
                        text: backend.install_error
                        color: bad
                        font.pixelSize: 12
                        wrapMode: Text.Wrap
                    }
                }

                RowLayout {
                    Layout.fillWidth: true
                    UiButton { text: "‹ Back"; kind: "ghost"; onClicked: wizard.goBack() }
                    Item { Layout.fillWidth: true }
                    UiButton {
                        text: "I finished — check again"
                        kind: "tonal"
                        onClicked: {
                            backend.run_preflight()
                            if (!wizard.needsInstall()) wizard.goNext()
                        }
                    }
                    UiButton { text: "Continue"; kind: "accent"; onClicked: wizard.goNext() }
                }
            }

            // ── 3 Camera ────────────────────────────────────────────────
            ColumnLayout {
                spacing: 10
                Label { text: "Pick your IR camera"; font.pixelSize: 20; font.bold: true; color: ink }
                Label {
                    Layout.fillWidth: true
                    wrapMode: Text.Wrap
                    font.pixelSize: 13
                    color: dim
                    text: "IR sensors usually expose a small square frame (e.g. 340×340). The guess is highlighted — confirm it in the live preview."
                }

                Rectangle {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    Layout.minimumHeight: 240
                    radius: 14
                    color: "#0B0C0F"
                    border.width: 1
                    border.color: line
                    clip: true

                    MediaDevices { id: qtCams }
                    Camera {
                        id: qtCam
                        active: wizard.visible && wizard.step === 3 && qtCams.videoInputs.length > 0
                        cameraDevice: {
                            if (qtCams.videoInputs.length === 0) return null
                            var path = (camCombo.currentIndex >= 0 && camCombo.currentIndex < backend.camera_paths.length)
                                ? backend.camera_paths[camCombo.currentIndex] : ""
                            return qtCams.videoInputs[qtCamIndex(qtCams.videoInputs, path)]
                        }
                        onErrorOccurred: (error, errorString) => { wizard.camError = errorString }
                    }
                    CaptureSession {
                        camera: qtCam
                        videoOutput: previewOut
                    }
                    VideoOutput {
                        id: previewOut
                        anchors.fill: parent
                        fillMode: VideoOutput.PreserveAspectFit
                    }
                    Label {
                        anchors.centerIn: parent
                        visible: qtCams.videoInputs.length === 0
                        text: "No Qt camera devices"
                        color: dim
                        font.pixelSize: 12
                    }
                }

                ComboBox {
                    id: camCombo
                    Layout.fillWidth: true
                    model: backend.camera_candidates
                }
                Label {
                    Layout.fillWidth: true
                    font.pixelSize: 12
                    color: accent
                    visible: backend.suggested_camera !== ""
                    text: "Suggestion: " + backend.suggested_camera + " — select its entry above and confirm in the preview"
                    wrapMode: Text.Wrap
                }
                Label {
                    Layout.fillWidth: true
                    font.pixelSize: 12
                    color: dim
                    text: "Qt sees " + qtCams.videoInputs.length + " device(s); the preview follows your pick by device id — trust the live image."
                    wrapMode: Text.Wrap
                }
                Label {
                    Layout.fillWidth: true
                    visible: wizard.camError !== ""
                    font.pixelSize: 12
                    color: bad
                    text: wizard.camError
                    wrapMode: Text.Wrap
                }

                RowLayout {
                    Layout.fillWidth: true
                    UiButton { text: "‹ Back"; kind: "ghost"; onClicked: wizard.goBack() }
                    Item { Layout.fillWidth: true }
                    UiButton { text: "Re-probe"; kind: "tonal"; onClicked: wizard.enterCamera() }
                    UiButton {
                        text: "Save & continue"
                        kind: "accent"
                        enabled: backend.camera_paths.length > 0
                        onClicked: {
                            backend.save_device_path(backend.camera_paths[camCombo.currentIndex])
                            backend.check_device()
                            backend.refresh_models()
                            wizard.goNext()
                        }
                    }
                }
            }

            // ── 4 Enroll ────────────────────────────────────────────────
            ColumnLayout {
                spacing: 10
                Item { Layout.fillHeight: true }
                Label { Layout.alignment: Qt.AlignHCenter; text: "Enroll your face"; font.pixelSize: 20; font.bold: true; color: ink }
                Label {
                    Layout.fillWidth: true
                    horizontalAlignment: Text.AlignHCenter
                    wrapMode: Text.Wrap
                    font.pixelSize: 13
                    color: dim
                    text: "Look at the IR camera when asked. You can add more faces later (glasses, low light…)."
                }
                TextField {
                    id: wizName
                    Layout.fillWidth: true
                    Layout.maximumWidth: 340
                    Layout.alignment: Qt.AlignHCenter
                    placeholderText: "Face label…"
                    font.pixelSize: 13
                    color: ink
                    background: Rectangle {
                        radius: 10
                        color: field
                        border.width: 1
                        border.color: wizName.activeFocus ? accent : line
                    }
                }
                UiButton {
                    Layout.alignment: Qt.AlignHCenter
                    text: "+ Enroll face"
                    kind: "accent"
                    enabled: wizName.text.trim() !== ""
                    onClicked: wizard.requestEnroll(wizName.text.trim())
                }
                Label {
                    Layout.alignment: Qt.AlignHCenter
                    font.pixelSize: 12
                    color: dim
                    text: backend.face_models.length === 0 ? "No faces enrolled yet"
                        : backend.face_models.length === 1 ? "1 face enrolled ✓" : backend.face_models.length + " faces enrolled ✓"
                }
                Item { Layout.fillHeight: true }
                RowLayout {
                    Layout.fillWidth: true
                    UiButton { text: "‹ Back"; kind: "ghost"; onClicked: wizard.goBack() }
                    Item { Layout.fillWidth: true }
                    UiButton {
                        text: "Continue"
                        kind: "accent"
                        enabled: backend.face_models.length > 0
                        onClicked: wizard.goNext()
                    }
                }
            }

            // ── 5 Integrate ─────────────────────────────────────────────
            ColumnLayout {
                spacing: 10
                Label { text: "Unlock with your face"; font.pixelSize: 20; font.bold: true; color: ink }
                Label { text: "Where Howdy should kick in (password always still works):"; font.pixelSize: 13; color: dim }

                UiCard {
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 12
                        RowLayout {
                            Layout.fillWidth: true
                            visible: backend.sddm_installed
                            Label { Layout.fillWidth: true; text: "Login screen (SDDM)"; font.pixelSize: 13; font.bold: true; color: ink }
                            UiSwitch { checked: backend.pam_sddm; onToggled: backend.toggle_pam("/etc/pam.d/sddm") }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            visible: backend.hyprlock_installed
                            Label { Layout.fillWidth: true; text: "Screen lock (hyprlock)"; font.pixelSize: 13; font.bold: true; color: ink }
                            UiSwitch { checked: backend.pam_hyprlock; onToggled: backend.toggle_pam("/etc/pam.d/hyprlock") }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Label { Layout.fillWidth: true; text: "Terminal (sudo)"; font.pixelSize: 13; font.bold: true; color: ink }
                            UiSwitch { checked: backend.pam_sudo; onToggled: backend.toggle_pam("/etc/pam.d/sudo") }
                        }
                        RowLayout {
                            Layout.fillWidth: true
                            Label { Layout.fillWidth: true; text: "Privilege prompts (polkit)"; font.pixelSize: 13; font.bold: true; color: ink }
                            UiSwitch {
                                property bool busy: false
                                checked: backend.pam_polkit
                                onToggled: {
                                    if (busy) return
                                    if (checked && !wizard.polkitArmed) {
                                        // Two-step confirm inline (no dialog plumbing
                                        // in wizard) — the revert must not recurse.
                                        wizard.polkitArmed = true
                                        busy = true
                                        checked = false
                                        busy = false
                                    } else {
                                        wizard.polkitArmed = false
                                        backend.toggle_pam("/etc/pam.d/polkit-1")
                                    }
                                }
                            }
                        }
                        Label {
                            Layout.fillWidth: true
                            visible: wizard.polkitArmed
                            text: "Polkit relaxes the helper sandbox for camera access. Toggle once more to confirm."
                            font.pixelSize: 12
                            color: warn
                            wrapMode: Text.Wrap
                        }
                    }
                }
                RowLayout {
                    Layout.fillWidth: true
                    UiButton { text: "‹ Back"; kind: "ghost"; onClicked: { wizard.polkitArmed = false; wizard.goBack() } }
                    Item { Layout.fillWidth: true }
                    UiButton { text: "Finish"; kind: "accent"; onClicked: wizard.goNext() }
                }
            }

            // ── 6 Done ──────────────────────────────────────────────────
            ColumnLayout {
                spacing: 14
                Item { Layout.fillHeight: true }
                Label { Layout.alignment: Qt.AlignHCenter; text: "✓"; font.pixelSize: 44; color: good }
                Label { Layout.alignment: Qt.AlignHCenter; text: "You're set"; font.pixelSize: 22; font.bold: true; color: ink }
                Label {
                    Layout.fillWidth: true
                    horizontalAlignment: Text.AlignHCenter
                    wrapMode: Text.Wrap
                    font.pixelSize: 13
                    color: dim
                    text: "Press Enter on an empty password prompt and look at the camera.\nYour password always works as backup."
                }
                Item { Layout.fillHeight: true }
                UiButton {
                    Layout.alignment: Qt.AlignHCenter
                    text: "Open FaceKey manager"
                    kind: "accent"
                    onClicked: wizard.wizardFinished()
                }
            }
        }
    }
}
