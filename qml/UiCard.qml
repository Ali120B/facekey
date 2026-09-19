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

    // Size to content unless the parent layout overrides (fillHeight /
    // preferredHeight). Without this the card collapses and rows overlap.
    implicitHeight: inner.implicitHeight + pad * 2
    implicitWidth: inner.implicitWidth + pad * 2

    ColumnLayout {
        id: inner
        anchors.fill: parent
        anchors.margins: root.pad
        spacing: root.gap
    }
}
