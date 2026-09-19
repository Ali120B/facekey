import QtQuick
import QtQuick.Controls

// Proper toggle: visible track + knob. Explicit geometry so it can never
// collapse to zero size inside layouts.
Switch {
    id: root

    hoverEnabled: true
    padding: 0
    implicitWidth: 48
    implicitHeight: 27

    indicator: Rectangle {
        x: 0
        y: 0
        width: 48
        height: 27
        radius: 14
        color: {
            if (!root.enabled) return "#22262E"
            if (root.checked) return root.pressed ? "#6F5EF0" : "#8B7CFF"
            return root.hovered ? "#3A404C" : "#2E333E"
        }
        border.width: root.checked ? 0 : 1
        border.color: "#454C5B"

        Rectangle {
            width: 21; height: 21; radius: 11
            anchors.verticalCenter: parent.verticalCenter
            x: root.checked ? parent.width - width - 3 : 3
            color: root.checked ? "#FFFFFF" : "#9AA3B2"
            Behavior on x { NumberAnimation { duration: 140; easing.type: Easing.OutCubic } }
        }
    }

    contentItem: Item {}
}
