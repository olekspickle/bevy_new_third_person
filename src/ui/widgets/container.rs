use super::*;

/// An extension trait for spawning UI containers.
pub trait UiRoot {
    /// Spawns a root node that covers the full screen
    /// and centers its content horizontally and vertically.
    fn ui_root(&mut self) -> EntityCommands;
}

impl UiRoot for Commands<'_, '_> {
    fn ui_root(&mut self) -> EntityCommands {
        self.spawn((
            Name::new("UI Root"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                row_gap: Px(10.0),
                ..default()
            },
        ))
    }
}

pub trait GenericContainer {
    /// Spawns a container node with specified node settings
    fn container(&mut self, node: Node) -> EntityCommands;
}

impl<T: Spawn> GenericContainer for T {
    fn container(&mut self, node: Node) -> EntityCommands {
        self.spawn((
            Name::new("Container"),
            Node {
                width: Percent(100.0),
                height: Percent(100.0),
                ..node
            },
        ))
    }
}
