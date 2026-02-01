import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Rectangle {
    id: dock
    width: 800
    height: 60
    color: "#1e1e1e"
    border.color: "#444"
    border.width: 1
    radius: 8

    // Connected to Rust via property binding
    required property var windowTiles
    required property real devicePixelRatio

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 8

        Text {
            text: "Niri Dock - " + dock.windowTiles.length + " windows"
            color: "#fff"
            font.pixelSize: 12 * dock.devicePixelRatio
            Layout.fillWidth: true
        }

        Rectangle {
            color: "#2a2a2a"
            height: 2
            Layout.fillWidth: true
        }

        Repeater {
            model: dock.windowTiles

            delegate: Rectangle {
                id: tileDelegate
                width: 120
                height: 40
                color: modelData.column_index % 2 === 0 ? "#3a3a3a" : "#4a4a4a"
                radius: 4
                border.width: 1
                border.color: "#666"

                ColumnLayout {
                    anchors.fill: parent
                    anchors.margins: 4
                    spacing: 2

                    Text {
                        text: modelData.app_id.substring(
                            modelData.app_id.lastIndexOf(".") + 1
                        )
                        color: "#aaa"
                        font.pixelSize: 10 * dock.devicePixelRatio
                        elide: Text.ElideRight
                        Layout.fillWidth: true
                    }

                    Text {
                        text: "Col " + modelData.column_index + " Tile " + modelData.tile_index
                        color: "#999"
                        font.pixelSize: 8 * dock.devicePixelRatio
                        Layout.fillWidth: true
                    }
                }

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        // Trigger focus action in Rust
                        dockBridge.focusWindow(modelData.id)
                    }
                }
            }
        }
    }
}
