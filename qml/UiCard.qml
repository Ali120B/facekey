import QtQuick
import QtQuick.Layouts

// Dark card container. Content goes in the inner column.
Rectangle {
    id: root
    property int pad: 16
    property int gap: 10
    default property alias content: inner.children

    color: "#1C1F26"
    radius: 14
    border.width: 1
    border.color: "#2B303B"

    ColumnLayout {
        id: inner
        anchors.fill: parent
        anchors.margins: root.pad
        spacing: root.gap
    }
}
