import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtMultimedia
import com.howdy.gui

ApplicationWindow {
    id: window
    visible: true
    width: 640
    height: 940
    minimumWidth: 560
    minimumHeight: 760
    title: "FaceKey" + (backend.is_test_run() ? " — test mode" : "")

    // ── Forced dark theme ───────────────────────────────────────────────────
    palette.window: "#131519"
    palette.windowText: "#ECEEF1"
    palette.base: "#14161B"
    palette.alternateBase: "#22262F"
    palette.text: "#ECEEF1"
    palette.placeholderText: "#7C8494"
    palette.button: "#262B35"
    palette.buttonText: "#ECEEF1"
    palette.highlight: "#8B7CFF"
    palette.highlightedText: "#0C0D10"
    palette.mid: "#2B303B"
    palette.dark: "#0E1013"
    palette.shadow: "#000000"

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

    // Inline camera preview (off by default so the device stays free
    // for enroll/test, which need exclusive access)
    property bool previewOn: false

    color: bg

    HowdyBackend {
        id: backend
        Component.onCompleted: {
            backend.detect_display_managers()
            backend.load_video_devices()
            backend.check_pam_status()
            backend.run_preflight()
            backend.probe_cameras()
            // First run (or broken setup) → guided wizard; it covers the
            // old standalone camera dialog, enroll and PAM wiring.
            // In --test-run the wizard always shows for walkthroughs.
            if (backend.is_test_run() || !backend.setup_howdy || !backend.setup_pam_python || !backend.camera_configured) {
                setupWizard.visible = true
            } else {
                backend.check_device()
                backend.refresh_models()
            }
            previewCombo.currentIndex = Math.max(0, backend.camera_paths.indexOf(backend.suggested_camera))
        }
    }

    Timer {
        id: refreshTimer
        interval: 400
        onTriggered: backend.refresh_models()
    }

    // ── Polkit disclaimer ───────────────────────────────────────────────────
    Dialog {
        id: polkitWarningDialog
        title: "Polkit integration"
        modal: true
        closePolicy: Popup.NoAutoClose
        anchors.centerIn: Overlay.overlay
        width: 520
        background: Rectangle { color: card; radius: 14; border.width: 1; border.color: line }

        ColumnLayout {
            width: parent.width
            spacing: 12

            Label {
                Layout.fillWidth: true
                text: "This relaxes the polkit helper sandbox so Howdy can reach the camera."
                font.pixelSize: 13
                wrapMode: Text.Wrap
                color: warn
            }
            Rectangle {
                Layout.fillWidth: true
                color: field
                radius: 8
                height: fileList.implicitHeight + 16
                Column {
                    id: fileList
                    anchors { left: parent.left; right: parent.right; top: parent.top; margins: 8 }
                    spacing: 4
                    Label { text: "• /etc/pam.d/polkit-1"; font.pixelSize: 12; font.family: "monospace"; color: dim }
                    Label { text: "• /etc/systemd/system/polkit-agent-helper@.service.d/override.conf"; font.pixelSize: 12; font.family: "monospace"; color: dim; wrapMode: Text.Wrap; width: parent.width - 16 }
                }
            }
            RowLayout {
                Layout.fillWidth: true
                Item { Layout.fillWidth: true }
                UiButton {
                    text: "Cancel"
                    kind: "ghost"
                    onClicked: {
                        polkitSwitch.busy = true
                        polkitSwitch.checked = false
                        polkitSwitch.busy = false
                        polkitWarningDialog.close()
                    }
                }
                UiButton {
                    text: "Enable"
                    kind: "accent"
                    onClicked: {
                        polkitWarningDialog.close()
                        backend.toggle_pam("/etc/pam.d/polkit-1")
                    }
                }
            }
        }
    }

    // ── Register face ───────────────────────────────────────────────────────
    Dialog {
        id: registerDialog
        title: "New face"
        modal: true
        closePolicy: Popup.NoAutoClose
        anchors.centerIn: Overlay.overlay
        width: 420
        background: Rectangle { color: card; radius: 14; border.width: 1; border.color: line }

        property string modelName: ""
        property bool capturing: false
        property bool captureFailed: false
        property bool captureSuccess: false
        property string failureMessage: ""

        onClosed: {
            capturing = false
            captureFailed = false
            captureSuccess = false
            failureMessage = ""
        }

        Timer {
            id: captureTimeout
            interval: 90000
            running: registerDialog.capturing
            onTriggered: {
                registerDialog.capturing = false
                registerDialog.captureFailed = true
                registerDialog.failureMessage = "Timed out — no face detected. Get closer, face the camera, try again."
            }
        }

        Timer {
            id: resultPoller
            interval: 500
            running: registerDialog.capturing
            repeat: true
            onTriggered: {
                if (backend.check_add_result()) {
                    var msg = backend.status_message
                    registerDialog.capturing = false
                    if (msg === "Face registered successfully") {
                        registerDialog.captureSuccess = true
                    } else {
                        registerDialog.captureFailed = true
                        registerDialog.failureMessage = msg
                    }
                }
            }
        }

        ColumnLayout {
            width: parent.width
            spacing: 14

            Rectangle {
                Layout.alignment: Qt.AlignHCenter
                width: 150; height: 150
                radius: 75
                color: "transparent"
                border.width: 2
                border.color: registerDialog.captureSuccess ? good
                            : registerDialog.captureFailed ? bad
                            : registerDialog.capturing     ? warn
                            :                               accent
                SequentialAnimation on opacity {
                    running: registerDialog.capturing
                    loops: Animation.Infinite
                    NumberAnimation { to: 0.35; duration: 700 }
                    NumberAnimation { to: 1.0; duration: 700 }
                }

                Rectangle {
                    anchors.centerIn: parent
                    width: 64; height: 76
                    radius: 32
                    color: line
                }
                Row {
                    anchors.centerIn: parent
                    anchors.verticalCenterOffset: -14
                    spacing: 16
                    Rectangle { width: 8; height: 6; radius: 3; color: ink; opacity: 0.7 }
                    Rectangle { width: 8; height: 6; radius: 3; color: ink; opacity: 0.7 }
                }
                Rectangle {
                    width: parent.width - 28; height: 2
                    anchors.horizontalCenter: parent.horizontalCenter
                    color: warn
                    opacity: 0.9
                    visible: registerDialog.capturing
                    SequentialAnimation on y {
                        running: registerDialog.capturing
                        loops: Animation.Infinite
                        NumberAnimation { to: 14; duration: 800; easing.type: Easing.InOutSine }
                        NumberAnimation { to: 132; duration: 800; easing.type: Easing.InOutSine }
                    }
                }
            }

            Label {
                Layout.alignment: Qt.AlignHCenter
                font.pixelSize: 14
                font.bold: true
                color: registerDialog.captureSuccess ? good
                     : registerDialog.captureFailed ? bad : ink
                text: registerDialog.captureSuccess ? "Captured"
                    : registerDialog.capturing     ? "Scanning — hold still"
                    : registerDialog.captureFailed ? "Not detected"
                    :                                "Ready"
            }

            Label {
                Layout.fillWidth: true
                horizontalAlignment: Text.AlignHCenter
                wrapMode: Text.Wrap
                font.pixelSize: 12
                color: registerDialog.captureFailed ? bad : dim
                text: registerDialog.captureSuccess
                    ? "Face captured. Save it or cancel to discard."
                    : registerDialog.capturing
                        ? "Look directly at the IR camera."
                        : registerDialog.captureFailed
                            ? "Check lighting, face the camera, move closer.\n\n" + registerDialog.failureMessage
                            : "Face the IR camera in good light, then start."
            }

            Rectangle {
                Layout.fillWidth: true
                height: 3; radius: 2
                color: line
                visible: registerDialog.capturing
                Rectangle {
                    height: parent.height; radius: parent.radius
                    color: warn
                    SequentialAnimation on width {
                        running: registerDialog.capturing
                        loops: Animation.Infinite
                        NumberAnimation { to: registerDialog.width - 48; duration: 1000; easing.type: Easing.InOutSine }
                        NumberAnimation { to: 0; duration: 1000; easing.type: Easing.InOutSine }
                    }
                }
            }

            RowLayout {
                Layout.alignment: Qt.AlignHCenter
                spacing: 8
                UiButton {
                    text: "Cancel"
                    kind: "ghost"
                    enabled: !registerDialog.capturing
                    onClicked: {
                        if (registerDialog.captureSuccess)
                            backend.refresh_models()
                        registerDialog.close()
                    }
                }
                UiButton {
                    text: registerDialog.captureSuccess ? "Save face"
                        : registerDialog.captureFailed ? "Try again" : "Start capture"
                    kind: "accent"
                    enabled: !registerDialog.capturing
                    onClicked: {
                        if (registerDialog.captureSuccess) {
                            backend.refresh_models()
                            registerDialog.close()
                        } else {
                            registerDialog.captureFailed = false
                            registerDialog.capturing = true
                            backend.add_model(registerDialog.modelName)
                        }
                    }
                }
            }
        }
    }

    // ── Main layout (scrolls when the preview card expands) ───────────────
    ScrollView {
        anchors.fill: parent
        contentWidth: availableWidth
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

        ColumnLayout {
            anchors { left: parent.left; right: parent.right; top: parent.top; margins: 24 }
            spacing: 14

        // Header: status pill + title + version
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Rectangle {
                radius: 12
                implicitHeight: 24
                implicitWidth: pillLabel.implicitWidth + 24
                color: backend.device_supported ? "#14332A" : "#3A1D1D"
                border.width: 1
                border.color: backend.device_supported ? "#1F5C46" : "#6E2B2B"
                Label {
                    id: pillLabel
                    anchors.centerIn: parent
                    font.pixelSize: 12
                    font.bold: true
                    color: backend.device_supported ? good : bad
                    text: backend.device_supported ? "● Ready" : "● No camera"
                }
            }
            ColumnLayout {
                spacing: 0
                Label { text: "FaceKey"; font.pixelSize: 20; font.bold: true; color: ink }
                Label { text: "Face ID for Linux"; font.pixelSize: 12; color: dim }
            }
            Item { Layout.fillWidth: true }
            Label { text: "v" + backend.app_version; font.pixelSize: 12; color: dim }
            UiButton {
                text: "Health"
                kind: "ghost"
                font.pixelSize: 12
                onClicked: {
                    backend.run_preflight()
                    doctorDialog.open()
                }
            }
        }

        // Faces card (list + register merged)
        UiCard {
            Layout.fillWidth: true
            Layout.preferredHeight: 320

            RowLayout {
                Layout.fillWidth: true
                Label { text: "Faces"; font.pixelSize: 15; font.bold: true; color: ink }
                Rectangle {
                    radius: 9
                    implicitHeight: 20
                    implicitWidth: countLabel.implicitWidth + 16
                    color: "#262B35"
                    Label {
                        id: countLabel
                        anchors.centerIn: parent
                        font.pixelSize: 11
                        font.bold: true
                        color: dim
                        text: modelList.count
                    }
                }
                Item { Layout.fillWidth: true }
                UiButton {
                    text: "⟳ Refresh"
                    kind: "ghost"
                    font.pixelSize: 12
                    onClicked: backend.refresh_models()
                }
            }

            ListView {
                id: modelList
                Layout.fillWidth: true
                Layout.fillHeight: true
                model: backend.face_models
                clip: true
                spacing: 2

                delegate: ItemDelegate {
                    width: modelList.width
                    height: 52
                    hoverEnabled: true
                    background: Rectangle {
                        radius: 10
                        color: parent.hovered ? "#22262E" : "transparent"
                    }

                    RowLayout {
                        anchors.fill: parent
                        anchors.leftMargin: 8
                        anchors.rightMargin: 8
                        spacing: 8

                        ColumnLayout {
                            Layout.fillWidth: true
                            spacing: 1
                            Label {
                                Layout.fillWidth: true
                                font.pixelSize: 13
                                font.bold: true
                                color: ink
                                elide: Text.ElideRight
                                text: {
                                    var parts = modelData.trim().split(/\s+/)
                                    return parts.length > 3 ? parts.slice(3).join(" ") : modelData.trim()
                                }
                            }
                            Label {
                                font.pixelSize: 11
                                color: dim
                                text: {
                                    var parts = modelData.trim().split(/\s+/)
                                    var when = parts.length > 2 ? " · " + parts[1] : ""
                                    return "ID " + parts[0] + when
                                }
                            }
                        }
                        UiButton {
                            text: "Remove"
                            kind: "danger"
                            font.pixelSize: 12
                            onClicked: {
                                var fid = parseInt(modelData.trim().split(/\s+/)[0])
                                backend.remove_model(fid)
                            }
                        }
                    }
                }

                Label {
                    anchors.centerIn: parent
                    text: "No faces yet — register one below"
                    visible: modelList.count === 0
                    color: dim
                    font.italic: true
                    font.pixelSize: 12
                }
            }

            Rectangle { Layout.fillWidth: true; height: 1; color: line }

            RowLayout {
                Layout.fillWidth: true
                spacing: 8
                TextField {
                    id: newModelName
                    Layout.fillWidth: true
                    placeholderText: "New face label…"
                    font.pixelSize: 13
                    color: ink
                    background: Rectangle {
                        radius: 10
                        color: field
                        border.width: 1
                        border.color: newModelName.activeFocus ? accent : line
                    }
                    onAccepted: addButton.clicked()
                }
                UiButton {
                    id: addButton
                    text: "+ Register"
                    kind: "accent"
                    font.pixelSize: 13
                    enabled: backend.device_supported && newModelName.text.trim() !== ""
                    onClicked: {
                        window.previewOn = false
                        registerDialog.modelName = newModelName.text.trim()
                        newModelName.text = ""
                        registerDialog.open()
                    }
                }
            }
        }

        // Quick actions: two real buttons
        RowLayout {
            Layout.fillWidth: true
            spacing: 8
            UiButton {
                Layout.fillWidth: true
                text: "◎  Test recognition"
                kind: "tonal"
                font.pixelSize: 13
                enabled: backend.device_supported
                onClicked: {
                    window.previewOn = false
                    backend.run_test()
                }
            }
            UiButton {
                Layout.fillWidth: true
                text: backend.howdy_enabled ? "○  Disable Howdy" : "●  Enable Howdy"
                kind: "tonal"
                font.pixelSize: 13
                onClicked: backend.toggle_enabled()
            }
        }

        // Camera card with inline live preview (collapsible so the
        // device stays free for enroll/test, which need exclusive access)
        UiCard {
            Layout.fillWidth: true
            Layout.preferredHeight: previewOn ? 360 : 76
            Behavior on Layout.preferredHeight { NumberAnimation { duration: 200 } }

            RowLayout {
                Layout.fillWidth: true
                ColumnLayout {
                    Layout.fillWidth: true
                    spacing: 2
                    Label { text: "Infrared camera"; font.pixelSize: 13; font.bold: true; color: ink }
                    Label {
                        text: backend.camera_configured ? "Device configured" : "Not configured"
                        font.pixelSize: 11
                        color: dim
                    }
                }
                UiButton {
                    text: previewOn ? "Stop" : "Preview"
                    kind: "tonal"
                    font.pixelSize: 12
                    onClicked: previewOn = !previewOn
                }
            }

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                Layout.minimumHeight: 180
                visible: previewOn
                radius: 10
                color: "#0B0C0F"
                border.width: 1
                border.color: line
                clip: true

                MediaDevices { id: mgrCams }
                Camera {
                    id: mgrCam
                    active: previewOn && mgrCams.videoInputs.length > 0
                    cameraDevice: mgrCams.videoInputs.length > 0 ? mgrCams.videoInputs[previewCombo.currentIndex < mgrCams.videoInputs.length ? previewCombo.currentIndex : 0] : null
                }
                CaptureSession {
                    camera: mgrCam
                    videoOutput: mgrPreview
                }
                VideoOutput {
                    id: mgrPreview
                    anchors.fill: parent
                    fillMode: VideoOutput.PreserveAspectFit
                }
            }

            RowLayout {
                Layout.fillWidth: true
                visible: previewOn
                spacing: 8
                ComboBox {
                    id: previewCombo
                    Layout.fillWidth: true
                    model: backend.camera_candidates
                    font.pixelSize: 12
                }
                UiButton {
                    text: "Use this camera"
                    kind: "accent"
                    font.pixelSize: 12
                    enabled: backend.camera_paths.length > 0
                    onClicked: {
                        window.previewOn = false
                        backend.save_device_path(backend.camera_paths[previewCombo.currentIndex])
                        backend.check_device()
                        backend.refresh_models()
                    }
                }
            }
        }

        // Integration card
        UiCard {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.minimumHeight: 170
            clip: true

            Label { text: "System integration"; font.pixelSize: 15; font.bold: true; color: ink }

            ScrollView {
                Layout.fillWidth: true
                Layout.fillHeight: true
                contentWidth: availableWidth
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff

                ColumnLayout {
                    width: parent.width
                    spacing: 12

                    RowLayout {
                        Layout.fillWidth: true
                        visible: backend.sddm_installed
                        ColumnLayout {
                            spacing: 1
                            Label { text: "Login screen"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "SDDM"; font.pixelSize: 11; color: dim }
                        }
                        Item { Layout.fillWidth: true }
                        UiSwitch { checked: backend.pam_sddm; onToggled: backend.toggle_pam("/etc/pam.d/sddm") }
                    }
                    RowLayout {
                        Layout.fillWidth: true
                        visible: backend.plasma_lm_installed
                        ColumnLayout {
                            spacing: 1
                            Label { text: "Login screen"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "Plasma Login Manager"; font.pixelSize: 11; color: dim }
                        }
                        Item { Layout.fillWidth: true }
                        UiSwitch { checked: backend.pam_plasma_lm; onToggled: backend.toggle_pam("/etc/pam.d/plasmalogin") }
                    }
                    RowLayout {
                        Layout.fillWidth: true
                        visible: backend.hyprlock_installed
                        ColumnLayout {
                            spacing: 1
                            Label { text: "Screen lock"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "hyprlock (in use)"; font.pixelSize: 11; color: dim }
                        }
                        Item { Layout.fillWidth: true }
                        UiSwitch { checked: backend.pam_hyprlock; onToggled: backend.toggle_pam("/etc/pam.d/hyprlock") }
                    }
                    RowLayout {
                        Layout.fillWidth: true
                        ColumnLayout {
                            spacing: 1
                            Label { text: "Screen lock"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "KDE lock screen"; font.pixelSize: 11; color: dim }
                        }
                        Item { Layout.fillWidth: true }
                        UiSwitch { checked: backend.pam_kde; onToggled: backend.toggle_pam("/etc/pam.d/kde") }
                    }
                    RowLayout {
                        Layout.fillWidth: true
                        ColumnLayout {
                            spacing: 1
                            Label { text: "Terminal"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "sudo password prompts"; font.pixelSize: 11; color: dim }
                        }
                        Item { Layout.fillWidth: true }
                        UiSwitch { checked: backend.pam_sudo; onToggled: backend.toggle_pam("/etc/pam.d/sudo") }
                    }
                    RowLayout {
                        Layout.fillWidth: true
                        ColumnLayout {
                            spacing: 1
                            Label { text: "Privilege prompts"; font.pixelSize: 13; font.bold: true; color: ink }
                            Label { text: "polkit / pkexec"; font.pixelSize: 11; color: dim }
                        }
                        Item { Layout.fillWidth: true }
                        UiSwitch {
                            id: polkitSwitch
                            property bool busy: false
                            checked: backend.pam_polkit
                            onToggled: {
                                if (busy) return
                                if (checked) {
                                    polkitWarningDialog.open()
                                } else {
                                    backend.toggle_pam("/etc/pam.d/polkit-1")
                                }
                            }
                        }
                    }
                }
            }
        }

        // Status + footer
        Label {
            Layout.fillWidth: true
            horizontalAlignment: Text.AlignHCenter
            elide: Text.ElideRight
            font.pixelSize: 12
            color: dim
            text: backend.status_message
        }
        Row {
            Layout.alignment: Qt.AlignHCenter
            spacing: 6
            Label { text: "Howdy"; font.pixelSize: 12; font.bold: true; color: accent
                MouseArea {
                    anchors.fill: parent; cursorShape: Qt.PointingHandCursor
                    onClicked: Qt.openUrlExternally("https://github.com/boltgolt/howdy")
                }
            }
            Label { text: "·"; font.pixelSize: 12; color: dim }
            Label { text: "Ali120B"; font.pixelSize: 12; font.bold: true; color: dim
                MouseArea {
                    anchors.fill: parent; cursorShape: Qt.PointingHandCursor
                    onClicked: Qt.openUrlExternally("https://github.com/Ali120B")
                }
            }
        }
    }

    // ── Doctor dialog (permanent preflight) ───────────────────────────────
    Dialog {
        id: doctorDialog
        title: "System health"
        modal: true
        anchors.centerIn: Overlay.overlay
        width: 480
        background: Rectangle { color: card; radius: 14; border.width: 1; border.color: line }

        ColumnLayout {
            width: parent.width
            spacing: 10

            Repeater {
                model: [
                    { ok: backend.setup_howdy, title: "Howdy face engine", sub: backend.setup_howdy ? "installed" : "missing — re-run setup below" },
                    { ok: backend.setup_pam_python, title: "PAM module", sub: backend.setup_pam_python ? "pam_python.so present" : "missing — face auth cannot engage" },
                    { ok: backend.setup_models, title: "AI models", sub: backend.setup_models ? "dlib data on disk" : "missing" },
                    { ok: backend.setup_agent, title: "Polkit agent", sub: backend.setup_agent ? "running" : "not running — dialogs fall back to terminal" },
                    { ok: backend.setup_ir_camera, title: "IR camera", sub: backend.setup_ir_camera ? "detected" : "none detected" },
                    { ok: backend.camera_configured, title: "Camera configured", sub: backend.camera_configured ? "device_path is set" : "not set" }
                ]
                RowLayout {
                    Layout.fillWidth: true
                    spacing: 10
                    Rectangle {
                        width: 9; height: 9; radius: 5
                        Layout.alignment: Qt.AlignVCenter
                        color: modelData.ok ? good : bad
                    }
                    ColumnLayout {
                        Layout.fillWidth: true
                        spacing: 0
                        Label { text: modelData.title; font.pixelSize: 13; font.bold: true; color: ink }
                        Label { text: modelData.sub; font.pixelSize: 11; color: dim; elide: Text.ElideRight; Layout.fillWidth: true }
                    }
                }
            }

            RowLayout {
                Layout.fillWidth: true
                UiButton {
                    text: "Re-check"
                    kind: "tonal"
                    onClicked: {
                        backend.run_preflight()
                        backend.load_video_devices()
                        backend.check_pam_status()
                    }
                }
                Item { Layout.fillWidth: true }
                UiButton { text: "Close"; kind: "ghost"; onClicked: doctorDialog.close() }
                UiButton {
                    text: "Re-run setup"
                    kind: "accent"
                    onClicked: {
                        doctorDialog.close()
                        setupWizard.step = 1
                        setupWizard.visible = true
                    }
                }
            }
        }
    }
    }

    // ── First-run setup wizard overlay ──────────────────────────────────
    // (Dialogs like registerDialog live on the overlay layer above this.)
    SetupWizard {
        id: setupWizard
        visible: false
        backend: backend
        onRequestEnroll: (name) => {
            window.previewOn = false
            registerDialog.modelName = name
            registerDialog.open()
        }
        onWizardFinished: {
            setupWizard.visible = false
            backend.check_device()
            backend.refresh_models()
        }
    }
}
