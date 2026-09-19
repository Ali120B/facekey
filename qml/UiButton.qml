import QtQuick
import QtQuick.Controls

// Button with real affordance. kind: "accent" | "tonal" | "ghost" | "danger"
Button {
    id: root
    property string kind: "tonal"

    hoverEnabled: true
    padding: 0
    leftPadding: 16
    rightPadding: 16
    topPadding: 9
    bottomPadding: 9

    readonly property color _bg: {
        if (!root.enabled) return "#22262E"
        if (root.kind === "accent") {
            if (root.pressed) return "#6F5EF0"
            if (root.hovered) return "#9A8BFF"
            return "#8B7CFF"
        }
        if (root.kind === "ghost" || root.kind === "danger") {
            if (root.pressed) return "#2A2E37"
            if (root.hovered) return "#22262E"
            return "transparent"
        }
        // tonal
        if (root.pressed) return "#1E222B"
        if (root.hovered) return "#2E3440"
        return "#262B35"
    }
    readonly property color _fg: {
        if (!root.enabled) return "#5B6270"
        if (root.kind === "accent") return "#0C0D10"
        if (root.kind === "danger") return root.hovered ? "#FF8585" : "#FF7B7B"
        return "#ECEEF1"
    }

    background: Rectangle {
        radius: 10
        color: root._bg
        border.width: root.kind === "tonal" ? 1 : 0
        border.color: "#343A46"
    }
    contentItem: Text {
        text: root.text
        font: root.font
        color: root._fg
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
        elide: Text.ElideRight
    }
}
